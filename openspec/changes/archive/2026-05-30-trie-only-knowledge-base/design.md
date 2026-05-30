## Context

Today the graph crate defines `VersionHistory` (two `Vec`s of version records) and `Snapshot` (two radix tries). `Session` holds both. On load, storage deserializes JSON arrays into `VersionHistory`, then `Snapshot::materialize` inserts every record into tries. On mutation, `append_*` pushes to the vec (using vec scans for `next_version` and `previous_digest`), then `Session::rematerialize` re-walks the entire vec into tries. Reads and mutation validation use the trie; append metadata uses the vec. This duplication is the root of the "two winner paths" problem.

CRDT trie keys already encode winner order (entity prefix + big-endian version + digest + previous_digest). `winner_under_prefix` + last-child descent selects the same record as `max_by(version, digest)` on the vec.

## Goals / Non-Goals

**Goals:**

- Single in-memory type (`KnowledgeBase`) holding block and edge version tries as the authoritative store.
- Append inserts one new trie key per version record; derive `version` and `previous_digest` from trie CRDT winner before insert.
- Merge unions tries by full key (idempotent for identical records; digest dedup implicit in key equality).
- Storage v1 unchanged: JSON arrays are a persistence envelope; load → trie inserts, save → trie export.
- Preserve all external behavior: active reads, conflict resolution, mutations, queries, CLI, round-trip semantics.
- Remove `VersionHistory`, `Snapshot::materialize`, `Session::rematerialize`, and vec-based winner methods.

**Non-Goals:**

- New on-disk format (v2 native trie serialization).
- Append-order WAL or chronological replication log.
- Renaming every `Snapshot` reference in tests/docs if a type alias preserves compatibility short-term.
- Changing CRDT rules, digest computation, or CLI commands.

## Decisions

### 1. Introduce `KnowledgeBase` as the unified store

**Choice:** Rename/refactor `Snapshot` into `KnowledgeBase` (or re-export `Snapshot` as a deprecated alias for one release). Struct holds `block_versions` and `edge_versions` tries only.

**Rationale:** One type, one winner path. Name reflects that this is the store, not a derived view.

**Alternatives considered:**

- Keep `Snapshot` name — shorter diff but perpetuates "derived view" confusion.
- Keep `VersionHistory` name on trie — wrong metaphor (not a vec log).

### 2. Move append/merge onto `KnowledgeBase`

**Choice:**

```text
KnowledgeBase::append_block_version(id, tombstoned, properties) -> BlockVersion
KnowledgeBase::append_edge_version(...) -> EdgeVersion
KnowledgeBase::merge(&self, other: &Self) -> Self
KnowledgeBase::crdt_winner_block(id) -> Option<&BlockVersion>   // includes tombstones
KnowledgeBase::active_block(id) -> Option<Block>                 // excludes tombstoned winner
```

Before insert: `next_version = crdt_winner.map(|v| v.version + 1).unwrap_or(1)`; `previous_digest = crdt_winner.map(|v| v.digest)`.

Insert: compute digest, build full key, `trie.insert(key, record)`.

**Rationale:** Single lookup path; O(trie depth) per append instead of O(history len).

### 3. Keep JSON v1 arrays as export-only persistence

**Choice:** `storage::load` deserializes `KnowledgeBaseFile`, inserts each record into empty `KnowledgeBase`. `storage::save` iterates both tries, collects records into vecs, serializes. Export order: sorted by full trie key bytes for stable diffs.

**Rationale:** No migration for existing files; storage crate owns the thin DTO.

**Alternatives considered:**

- Persist trie structure natively — deferred to future format v2.

### 4. Session holds `KnowledgeBase` only

**Choice:**

```rust
pub struct Session {
    kb: KnowledgeBase,
    path: PathBuf,
    dirty: bool,
}
```

Mutations call `kb.append_*` directly. No rematerialize. `snapshot()` accessor returns `&KnowledgeBase` (or keep name `snapshot()` as alias).

**Rationale:** Eliminates dual-state sync bugs.

### 5. Merge via trie union

**Choice:** `KnowledgeBase::merge` clone left trie, insert all keys from right trie (copy-on-write). Dedup by full key; identical digest → identical key → idempotent.

**Rationale:** Replaces `merge_histories` vec union + digest set.

### 6. Distinguish CRDT winner vs active entity

**Choice:** Document and name explicitly:

- `crdt_winner_*` — for `previous_digest` chain (tombstones included)
- `active_*` — for queries and mutation validation (tombstoned winner → absent)

**Rationale:** Prevents conflating "chain head" with "visible block."

## Risks / Trade-offs

- **[Large refactor touches many tests]** → Implement in graph first with type alias `Snapshot = KnowledgeBase` temporarily; migrate call sites crate by crate.
- **[Export order differs from append order]** → Spec clarifies record set equality, not vec order; sort keys on save for stability.
- **[Breaking internal API]** → No CLI change; document in README architecture section.
- **[Load performance unchanged]** → Still O(n) inserts on load; acceptable for v0; native format later if needed.

## Migration Plan

1. Implement `KnowledgeBase` with append/merge/read methods (lift from `Snapshot` + `version.rs`).
2. Switch storage load/save to use `KnowledgeBase`.
3. Switch session/mutations to single store; delete rematerialize.
4. Delete `VersionHistory` and materialize paths.
5. Run full `cargo test`; verify existing JSON fixtures round-trip.
6. Update README architecture diagram.

Rollback: revert branch; no file format change.

## Open Questions

- **Type alias period:** Keep `pub type Snapshot = KnowledgeBase` in `lib.rs` for minimal diff, or rename everywhere in one pass? (Recommend: rename in graph, alias in `lib.rs` if external callers exist — currently only internal crates.)
