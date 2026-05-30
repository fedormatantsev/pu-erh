## Why

The radix-trie snapshot already stores all version records in `block_versions` / `edge_versions` tries and resolves winners in O(trie depth) via `winner_under_prefix`. The follow-on `rebuild_active_view()` pass duplicates that work into `HashMap` caches on every rematerialize. With trie-backed reads, materialization can be trie construction only — each read recomputes what it needs with no cached active view and no deferred invalidation layer.

## What Changes

- Remove eager `rebuild_active_view()` from `Snapshot::materialize` / `materialize_from`; building/updating the tries **is** materialization.
- Drop `blocks`, `edges`, and `root_id` cached fields from `Snapshot`.
- Serve all read APIs via **pure per-call** trie queries: winner resolution + tombstone check + cheap local invariant checks only.
- **Loosen read-time invariant strictness:** assume successful mutations never leave the graph in a violated state; reads do not run global analysis (no cycle detection, no graph-wide root invalidation that clears all entities).
- Cheap read checks retained: tombstone exclusion; edge endpoint existence when returning a specific edge; unknown edge type rejection.
- Remove global invariant filtering passes (`drop_cycle_edges`, `drop_conflicting_parent_edges`, invalid-root clears-all).
- Simplify `Session::rematerialize` to trie merge only.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `immutable-snapshot`: Trie-only materialization; pure per-call active reads; cheap local invariant checks only; global invariant filtering removed from read path.
- `session`: Rematerialization merges version records into tries only; relies on mutation path to preserve invariants.

## Impact

- **`crates/graph/src/snapshot.rs`**: Remove HashMap caches, `rebuild_active_view`, and global filter passes; refactor reads to pure trie queries.
- **`crates/core/src/mutation.rs`**: Document/enforce that mutations preserve invariants (existing validation path).
- **Tests**: Update tests that depended on global cycle/root clearing behavior; add coverage for per-call read paths.
- **Behavior change**: Corrupt or merged histories with cycles/conflicting parents may surface differently than before — acceptable per loosened read contract.
- **No storage format change**; public `Snapshot` method signatures unchanged.
