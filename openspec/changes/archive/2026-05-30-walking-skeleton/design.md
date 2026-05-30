## Context

pu-erh is a block-based graph knowledge base built in Rust. The target architecture separates a user session (Core) from an in-memory logical graph, on-disk storage, and a future DataFusion query engine. This change implements the thinnest vertical slice: CLI → session → graph ↔ storage, with a small query language and mutation commands. No application code exists yet.

## Goals / Non-Goals

**Goals:**

- Prove the layered architecture with a working CLI
- Establish block model and in-memory graph as the canonical logical representation
- Persist and restore a knowledge base from a single file
- Support hierarchy navigation via `parent:<uuid>` and `children:<uuid>`
- Support create, move (reparent), and delete mutations
- Lay out a Rust workspace structure that future changes (DataFusion, links, App) can extend

**Non-Goals:**

- DataFusion integration (deferred to a follow-up change)
- Non-hierarchical edge types beyond `parent` (links, refs — deferred; edge model supports them later)
- Property-based filtering or analytical queries
- GUI, REPL, or interactive TUI
- Schema validation or migrations
- Concurrent access, locking, or multi-user sessions

## Decisions

### 1. Rust monorepo layout: `cli`, `core`, `graph`, `storage`

**Choice:** Four crates in a Cargo workspace:

| Crate | Responsibility |
|-------|----------------|
| `graph` | Block model and in-memory graph (logical data model) |
| `storage` | On-disk persistence — load and save knowledge base files |
| `core` | User session, query language, and mutations; coordinates `graph` and `storage` |
| `cli` | Binary entry point (clap commands) |

```
cli ──▶ core ──▶ graph
         │
         └──▶ storage ──▶ graph
```

**Rationale:** Mirrors the target architecture (CLI → Core → Graph / Storage). Each crate has a single responsibility. `graph` and `storage` are reusable without the CLI; `core` owns session orchestration.

**Alternatives considered:**
- Two crates (`core` bundling graph + storage, plus `cli`) — fewer boundaries but doesn't reflect the logical/physical model split
- Single binary crate — too monolithic for future App/GUI reuse

### 2. Block IDs: UUID v4

**Choice:** Opaque `Uuid` (v4) for all block identifiers.

**Rationale:** Stable across save/load; no collision risk; standard format for CLI and storage.

**Alternatives considered:**
- Incremental IDs (`block_id_0`) — simpler for debugging but fragile for merge/sync later
- UUID v7 — time-sortable; defer until storage ordering matters

### 3. In-memory graph: block map + edge map

**Choice:** Two maps:

- **Blocks:** `HashMap<Uuid, Block>` keyed by block id. Block contains id and a properties map (generic key/value dictionary). No block type — all blocks are structurally identical.
- **Edges:** `HashMap<EdgeKey, Edge>` keyed by composite `{target_id}{type}{source_id}`. An edge has source id, target id, edge type string, and a properties map.

Hierarchy uses edges with type `"parent"`: source is the child, target is the parent.

```
EdgeKey = target_id ++ type ++ source_id

Example: child C under parent P
  key:   P + "parent" + C
  edge:  source=C, target=P, type="parent", properties={}
```

**Lookups:**

| Query | Mechanism |
|-------|-----------|
| `children:<id>` | Prefix scan on edge keys starting with `<id>` + `"parent"` → collect sources |
| `parent:<id>` | Find edge with type `"parent"` and source `<id>` → return its target |

**Rationale:** Separates node data from relations. Target-prefix keys make incoming-edge queries (children of a parent) efficient via prefix scan. The edge map generalizes to non-hierarchical relation types in later changes without changing the block model.

**Alternatives considered:**
- Parent pointer on block — simpler for parent/children only; doesn't generalize to other edge types
- petgraph crate — useful for analytics later; adds dependency before needed
- Source-prefix key only — efficient for outgoing edges but not for children lookup

### 3b. Properties: untyped key/value maps

**Choice:** Both blocks and edges carry a `Properties` map (`HashMap<String, Value>` or equivalent). No block-level type discriminator — meaning and structure come from properties and edge types.

**Rationale:** Maximum flexibility for a generic knowledge base. Edge type distinguishes relation kinds; block properties hold arbitrary data.

**Alternatives considered:**
- Typed blocks with schema registry — deferred; adds validation complexity
- Block `type` field — rejected; properties alone are sufficient

### 4. Storage format: JSON file

**Choice:** Single JSON file containing serialized blocks and edges (via serde), e.g. `{ "blocks": [...], "edges": [...] }`.

**Rationale:** Human-readable for debugging; zero schema tooling needed; easy round-trip tests. Path passed to CLI (e.g. `pu-erh --file ./kb.json query ...`).

**Alternatives considered:**
- SQLite — better for large graphs; deferred
- MessagePack/bincode — smaller/faster; less debuggable for v0
- Directory of block files — flexible but complex for walking skeleton

### 5. Session lifecycle: load-on-open, save-on-command

**Choice:** Each CLI invocation loads the file (or initializes a new graph with a root block if missing), executes one or more subcommands, then saves if mutations occurred.

**Rationale:** No long-running daemon; simple mental model for v0.

**Alternatives considered:**
- Long-running session daemon — needed for App later, not CLI v0
- Auto-save after every mutation within one invocation — same effect, explicit dirty flag

### 6. Query language: prefix parser, no AST yet

**Choice:** String prefix dispatch: `parent:<uuid>` and `children:<uuid>`. Parser validates prefix and parses UUID; executor walks graph.

**Rationale:** Minimal grammar; direct graph traversal without DataFusion. Extensible to richer syntax later.

**Alternatives considered:**
- Full parser combinator crate (nom, pest) — justified when grammar grows
- Route all queries through DataFusion now — premature; hierarchy walks don't need SQL

### 7. Mutations: imperative commands, validated in core

**Choice:** CLI subcommands `create`, `move`, `delete` call core mutation API. Core validates (e.g. no cycles on move, parent exists, block exists) and returns errors.

**Rationale:** Clear separation: query language is read-only; writes go through explicit commands.

**Cycle prevention on move:** Reject move if new parent is the block itself or any of its ancestors (walk parent edges upward from the new parent). Reject move that would remove a block's parent edge (only the root block has no parent).

### 7b. Single root block

**Choice:** Every knowledge base has exactly one root block, created automatically when initializing a new graph (missing storage file). The root has no `parent` edge. All other blocks MUST be created with a parent. The root block MUST NOT be deleted or reparented to root.

**Rationale:** Provides a stable anchor for the hierarchy. Prevents multiple disconnected top-level trees in v0.

**Alternatives considered:**
- Allow multiple root blocks via create — rejected; user requires single auto-created root
- Explicit `root` block type — rejected; root is structurally identical, distinguished by absence of parent edge

### 8. CLI structure (clap)

**Choice:**

```
pu-erh [--file PATH] <subcommand>

Subcommands:
  query <expr>          # e.g. parent:550e8400-e29b-41d4-a716-446655440000
  create --parent UUID   # required; root block exists automatically in new graphs
  move <id> --parent UUID
  delete <id>
```

Default `--file` to `./pu-erh.json` or require explicit path (prefer explicit to avoid surprises).

**Output:** Query results print block id and properties. Create prints the new block's UUID.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| JSON doesn't scale to large knowledge bases | Accept for v0; document limit; migrate format in future change |
| Parent query requires source lookup (not target-prefix) | Accept O(edges) scan for v0; add source index in later change if needed |
| Edge key collisions | Enforce unique (target, type, source) triple; reject duplicate edge on create |
| Missing file semantics | Initialize graph with single root block; invalid JSON is error |
| Property values untyped | Use `serde_json::Value` or string-keyed map; schema validation deferred |

## Migration Plan

Greenfield — no migration. First release creates `pu-erh.json` on first save.

## Open Questions

- Default `--file` path: `./pu-erh.json` vs required argument? **Recommend: required `--file` for explicitness in v0.**
- Delete behavior: reject if block has children, or cascade delete? **Recommend: reject with error in v0.**
