## 1. Remove eager active view

- [x] 1.1 Remove `blocks`, `edges`, and `root_id` fields from `Snapshot`
- [x] 1.2 Delete `rebuild_active_view()` and global filter passes (`drop_cycle_edges`, `drop_conflicting_parent_edges`, `recompute_root` clear-all)
- [x] 1.3 Update `materialize` / `materialize_from` to trie-merge only

## 2. Pure per-call read helpers

- [x] 2.1 Add `active_block(id)` — `winner_under_prefix` + tombstone check, no cache
- [x] 2.2 Add `active_edge(target, type, source)` — winner + tombstone + endpoint existence via two `active_block` calls
- [x] 2.3 Add `distinct_block_ids()` / `distinct_edge_entities()` iterators over trie entity prefixes

## 3. Refactor public read APIs

- [x] 3.1 Wire `block`, `get_block`, `get_edge` through per-call helpers
- [x] 3.2 Implement `parent_of` via edge-entity scan filtered to parent type + source (per call)
- [x] 3.3 Implement `children_of` via nav-prefix scan + per-entity winner (per call)
- [x] 3.4 Implement `blocks`, `edges`, `edges_with_prefix`, counts via trie scans (per call)
- [x] 3.5 Implement `root_id` via per-call scan for non-tombstoned block without parent edge; error if not exactly one

## 4. Snapshot diff and session

- [x] 4.1 Update `Snapshot::diff` entity decoding to use per-call active resolution
- [x] 4.2 Confirm `Session::rematerialize` is trie-merge only; mutation validation unchanged
- [x] 4.3 Remove/update tests asserting global cycle filtering or invalid-root clear-all

## 5. Verification

- [x] 5.1 Update `snapshot.rs` tests for loosened read invariants and per-call behavior
- [x] 5.2 Run full workspace `cargo test`
- [x] 5.3 Document mutation-presumed-validity assumption in mutation module (brief comment)
