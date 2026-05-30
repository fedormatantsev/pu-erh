## Context

pu-erh's walking-skeleton stores authoritative state as a mutable `Graph` (block/edge maps) serialized to JSON. Mutations modify the graph in place. This change replaces that with an **append-only version history** for blocks and edges. Each mutation inserts a new version of the affected entity (same id, next version) linked by cryptographic digests. Concurrent edits produce **branches** in history. Conflicts are **not** resolved during replication — only at **read time**.

## Goals / Non-Goals

**Goals:**

- Version every block and edge: `version`, `digest`, `previous_digest`
- Append-only mutations (insert new version records, never update in place)
- Deterministic read-time winner: **max version, then max digest**
- Union version histories on replication merge (no conflict repair at merge)
- Materialize immutable snapshot at read time; filter invariant violations silently
- Preserve CLI/query/mutation behavior for single-writer linear history

**Non-goals:**

- Network sync protocol
- Property-level CRDT merge (winning version's properties are taken as-is)
- DataFusion integration
- Snapshot JSON migration
- Compaction of old versions

## Decisions

### 1. Version history as source of truth

**Choice:** Authoritative persisted data is the union of all **block version records** and **edge version records**. There is no separate mutation envelope (`DeltaEnvelope`). Each mutation appends one or more new version rows.

```
BlockVersion  { id, version, digest, previous_digest, properties, tombstoned? }
EdgeVersion   { source, target, edge_type, version, digest, previous_digest, properties, tombstoned? }
```

The materialized `Snapshot` is **never** stored as authoritative state — only derived at read time.

**Rationale:** Full history enables branching, audit, and replication by union. Digest chains anchor each version to its predecessor for causal debugging.

**Alternatives considered:**
- Delta envelopes with merge at write time — hides branching, resolves too early
- Snapshot + tail — dual source of truth
- Mutable graph — current walking-skeleton; no history

### 2. Version, digest, and previous digest

**Choice:**

| Field | Role |
|-------|------|
| `version` | Monotonic u64 per entity id (block uuid or edge key); starts at 1 |
| `digest` | BLAKE3 hash of the record's in-memory fields (see below) |
| `previous_digest` | `digest` of the prior version in this entity's chain; `None` for first version |

**Digest computation:** BLAKE3 over the record's **in-memory representation** using **in-place incremental hashing** — feed field values directly into a `Hasher` in a fixed order. Do **not** serialize to JSON (or any intermediate buffer) to compute the hash.

Block hash input order (example): `id` → `version` → `tombstoned` → `properties` (keys sorted, then values) → …

Edge hash input order (example): `source` → `target` → `edge_type` → `version` → `tombstoned` → `properties` (sorted) → …

Store `digest` and `previous_digest` as fixed-length byte arrays (32 bytes); compare lexicographically for tie-breaks.

**Rationale:** `previous_digest` links versions into chains and enables branching detection. BLAKE3 is fast for incremental/in-place hashing on structured data. Avoiding serialization prevents format drift from affecting digests.

**Alternatives considered:**
- Lamport clocks / replica ids — rejected; user requires digest-based resolution
- Content-addressed only (no version number) — version makes winner selection explicit

### 3. Mutations append new versions

**Choice:** Each successful mutation appends new version record(s):

| User mutation | Appended records |
|---------------|------------------|
| Initialize KB | `BlockVersion` v1 for root (`previous_digest: None`) |
| Create block | `BlockVersion` v1 for new block + `EdgeVersion` v1 for `parent` edge |
| Move block | `EdgeVersion` v(n+1) tombstoning old parent edge + `EdgeVersion` v(m+1) adding new parent edge |
| Delete block | `BlockVersion` v(n+1) with `tombstoned: true` |

Validation runs against the **current materialized snapshot** before append (single-writer path). Invalid mutations append nothing.

**Rationale:** Same user-visible semantics as walking-skeleton; history captures every change as new versions.

**Alternatives considered:**
- Single combined record per mutation — loses per-entity history granularity

### 4. Branching

**Choice:** Concurrent mutations may produce **multiple version records for the same entity id at the same version number** (or divergent chains sharing a `previous_digest`). This is normal — history branches.

```
         digest=A (v1)
        /              \
   digest=B (v2)    digest=C (v2)   ← concurrent edits
```

Replication **unions** all records from all sources. No deduplication or winner selection at merge time.

**Rationale:** Conflicts are deferred to read time per user requirement.

### 5. Read-time conflict resolution

**Choice:** To materialize a snapshot:

1. Group block versions by block `id`; group edge versions by `(source, target, edge_type)`
2. For each group, select the **winner**: record with **max `version`**; tie-break by **max `digest`** (lexicographic compare)
3. Build candidate graph from winners (exclude `tombstoned` blocks; exclude tombstoned edges)
4. Run invariant checks; **drop** any block or edge that violates invariants (do not error)
5. Expose result as immutable `Snapshot`

**Rationale:** Deterministic, local, no user interaction. Max version + max digest is stable across replicas.

**Invariants checked at read time (non-exhaustive):**
- Exactly one non-tombstoned root block (no `parent` edge as source)
- At most one active `parent` edge per child block
- No cycles in active `parent` edges
- Edge endpoints reference existing non-tombstoned blocks

Violating entities are omitted from the snapshot.

**Alternatives considered:**
- Fail on invariant violation — rejected; user requires ignore at read time
- Resolve conflicts at replication — rejected

### 6. Immutable snapshot

**Choice:** `Snapshot` exposes the same read API as today's `Graph` (`block`, `parent`, `children`, `root_id`, …). Built only via `Snapshot::materialize(history: &VersionHistory) -> Snapshot`. No public mutation methods.

**Rationale:** Query and CLI code change minimally; immutability enforced by type system.

### 7. Storage format v1

**Choice:**

```json
{
  "format_version": 1,
  "block_versions": [ /* BlockVersion[] */ ],
  "edge_versions": [ /* EdgeVersion[] */ ]
}
```

Missing file → empty history; first save appends root block v1.
Replication merge → concatenate + dedupe identical records by full content hash (optional) or keep all.

**Rationale:** Human-debuggable; append-only; mirrors in-memory model.

### 8. Session refactor

**Choice:** `Session` holds `{ history: VersionHistory, snapshot: Snapshot, path, dirty }`. On open: load history → materialize snapshot. On mutation: validate → append version(s) → rematerialize snapshot. On save: write history.

### 9. Breaking change: no snapshot JSON migration

**Choice:** v1 version-history format only.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| History grows unbounded | Document; compaction is follow-up |
| Read-time materialization cost O(records) | Accept for v0; index by entity id later |
| Ignoring invalid entities may hide data issues | Log at debug level; document invariants |
| **BREAKING** storage format | `format_version` field; manual re-create |
| Branching produces surprising winners | Deterministic rule documented; single-writer path matches old behavior |

## Migration Plan

No automatic migration. Recreate knowledge bases from scratch.

## Open Questions

- Dedupe identical records on replication union? **Recommend: yes, by digest equality.**
- Include explicit `tombstoned` bool vs sentinel property? **Recommend: explicit bool on version record.**
