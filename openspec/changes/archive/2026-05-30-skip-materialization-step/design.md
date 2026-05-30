## Context

`radix-trie-map` introduced a two-phase snapshot build: trie merge, then eager `rebuild_active_view()` into `HashMap` caches with global invariant filtering (missing endpoints, duplicate parents, cycles, single-root validation).

The trie already supports O(trie depth) winner lookup and prefix scans. This change removes phase 2 entirely and reads directly from `block_versions` / `edge_versions` with **pure per-call recomputation** — no cached active maps, no lazy invalidation overlay.

## Assumptions

1. **Mutations preserve invariants** — successful mutation operations (`create_block`, `move_block`, `delete_block`) never append version records that violate graph invariants. Validation runs before append (existing mutation path).
2. **Reads check cheap invariants only** — no global chain/cycle analysis, no graph-wide exclusion passes, no "invalid root clears entire snapshot" behavior at read time.

Replicated or merged histories may contain invariant violations; those are out of scope for read-time repair in v1.

## Goals / Non-Goals

**Goals:**

- `Snapshot` holds only `block_versions` and `edge_versions` tries.
- `materialize` / `materialize_from` merge version records into tries — nothing else.
- Every read API recomputes its answer from tries on each call (no cross-call cache).
- Cheap per-entity checks at return time: tombstone, local endpoint presence, edge type mapping.
- Keep incremental trie merge on mutation.

**Non-Goals:**

- Global cycle detection or duplicate-parent sweeps at read time.
- Lazy or eager invariant overlay caches.
- Changing CRDT keys, storage format, or public API signatures.
- Guaranteeing correct active view for adversarial/corrupt merged histories.

## Decisions

### 1. Snapshot struct

**Choice:**

```rust
pub struct Snapshot {
    block_versions: RadixTrieMap<BlockVersion>,
    edge_versions: RadixTrieMap<EdgeVersion>,
}
```

No `blocks`, `edges`, `root_id`, `LazyInvariants`, or generation counters.

### 2. Materialize = trie merge only

**Choice:** `materialize_from` inserts/merges version records into tries and returns. No post-pass.

**Rationale:** Materialization is authoritative trie state; reads derive active view on demand.

### 3. Pure per-call read model

**Choice:** Each public read method independently:

1. Resolves winners via `winner_under_prefix` (or prefix scan + per-entity winner).
2. Applies tombstone exclusion.
3. Applies **cheap local checks** relevant to that call (see decision 4).
4. Maps to `Block` / `Edge` or absence.

No shared cache between calls. `blocks()` and `edges()` scan distinct entity prefixes from tries on every invocation.

**Rationale:** Simplest model; matches user requirement; avoids stale state.

### 4. Cheap read-time checks (local only)

**Choice:**

| Check | When | Cost |
|-------|------|------|
| Tombstone | Every winner resolution | O(1) field read |
| Unknown edge type | Edge reads | O(1) enum mapping |
| Missing endpoint | Returning a specific edge | 2 × `block()` winner lookups |
| Missing block | `block(id)` | winner only |

**Not performed at read time:**

- Cycle detection in parent edges
- Global duplicate-parent counting
- Graph-wide root validation with full clear
- Pre-scan exclusion sets

**`root_id()`:** Per call, scan distinct block entity prefixes; return the id of a non-tombstoned winner with no active parent edge (via point `parent_of` lookup). If mutation assumptions hold, exactly one exists. If zero or multiple, return `InvalidGraph` error (no silent clear-all).

**`parent_of(child)`:** Scan edge trie for parent-type edges with matching source, or iterate candidate parent edge entity keys; return target from first non-tombstoned winner with existing endpoints. No global parent-count validation.

**Rationale:** Mutations guarantee consistency for the single-writer path; reads stay O(local) not O(graph).

### 5. Removed global filter passes

**Choice:** Delete `rebuild_active_view`, `drop_cycle_edges`, `drop_conflicting_parent_edges`, and `recompute_root` clear-all behavior.

**Rationale:** These are global analyses incompatible with pure per-call cheap reads.

**Alternatives considered:**
- Lazy invariant cache — rejected (user wants pure per-call).
- Keep cycles at read time — rejected (user wants no global chain analysis).

### 6. Mutation contract

**Choice:** Document that `crates/core/src/mutation.rs` validation is the sole gate for graph invariants before append. Snapshot reads trust this for session-scoped use.

**Rationale:** Aligns read looseness with write strictness.

### 7. Session rematerialize

**Choice:** Unchanged call pattern; `materialize_from(Some(&snapshot), &history)` now only merges tries.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Merged/replicated histories may expose invalid edges/blocks | Document assumption; mutation path remains strict |
| `root_id()` / `blocks()` scan tries every call | Acceptable for v0 scale; trie scans are prefix-bounded |
| Behavior change vs old global filtering | Update tests; note in change summary |
| `parent_of` may scan edge entities | O(edges) per call; optimize later with reverse index if needed |

## Migration Plan

1. Refactor read helpers to pure trie queries + cheap checks.
2. Remove HashMap fields and global filter functions.
3. Update tests removing cycle/global-root scenarios; keep mutation + winner tests.
4. Run full workspace test suite.

Rollback: restore eager `rebuild_active_view` from git history.

## Open Questions

- **`parent_of` scan strategy:** Full edge-entity iteration vs nav-prefix heuristics. **Recommend:** iterate distinct edge entity prefixes from trie, filter `Parent` type + matching source — simple, correct under mutation assumption.
