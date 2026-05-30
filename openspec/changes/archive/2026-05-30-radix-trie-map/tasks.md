## 1. Radix trie core

- [x] 1.1 Add `radix_trie` module with `Node` enum (`Inner { children_mask, children }`, `Leaf { key, value }`) and `RadixTrieMap<V>` wrapper (`Option<Arc<Node<V>>>`)
- [x] 1.2 Implement 256-bit bitmask helpers (test bit, set bit, rank/select for sparse child indexing)
- [x] 1.3 Implement `get(&[u8]) -> Option<&V>` with byte-at-a-time inner descent
- [x] 1.4 Implement compact `insert` (lone leaf on first key; split leaf into inner+leaves on divergence; replace on duplicate key)
- [x] 1.5 Implement persistent `remove` returning new map (copy-on-write path only; collapse inner nodes when reduced to one child where applicable)
- [x] 1.6 Implement `len`, `is_empty`, and shallow `Clone`
- [x] 1.7 Unit tests: empty map, single leaf, shared-prefix split, replace, remove, rank/select correctness

## 2. Prefix iteration and diff

- [x] 2.1 Implement `iter_prefix(&[u8]) -> impl Iterator<Item = (&[u8], &V)>` (DFS from prefix boundary, lexicographic key order)
- [x] 2.2 Implement `iter` as `iter_prefix(&[])` and `winner_under_prefix(&[u8]) -> Option<(&[u8], &V)>` (prefix descent + `last_child` helper on highest set bit; O(trie depth))
- [x] 2.3 Define `DiffKind`, `TrieDiffEntry<'a, V>`, and `diff(&self, &other) -> impl Iterator<Item = TrieDiffEntry<'_, V>>` using `Arc::ptr_eq` short-circuit (no upfront `Vec` collection)
- [x] 2.4 Unit tests: prefix filtering, empty prefix = all entries, `winner_under_prefix` winner selection and depth bound, diff iterator on identical/shared-root maps (zero entries), diff on single-key change, early-exit after first entry

## 3. Key encoding helpers

- [x] 3.1 Add `EdgeType` as `#[repr(u8)]` enum with `Parent = 0` and `TryFrom<&str>` / `as_str()` for API boundary conversion
- [x] 3.2 Add `CrdtKeySuffix::write_into` (version BE u64 + digest + previous_digest with zero sentinel) and parse helpers
- [x] 3.3 Add `block_version_key(id, version, digest, previous_digest) -> [u8; 88]` and `block_entity_prefix(id) -> [u8; 16]`
- [x] 3.4 Add `edge_version_key(target, EdgeType, source, version, digest, previous_digest) -> [u8; 105]`, `edge_entity_prefix(...) -> [u8; 33]`, and `edge_nav_prefix(target, EdgeType) -> [u8; 17]`
- [x] 3.5 Unit tests: key length, BE version ordering, digest tie-break order, zero sentinel for missing previous_digest

## 4. Snapshot integration

- [x] 4.1 Replace `Snapshot` fields with `RadixTrieMap<BlockVersion>` and `RadixTrieMap<EdgeVersion>`
- [x] 4.2 Refactor `materialize` to insert every version record with full CRDT keys (remove separate winner `HashMap` pass)
- [x] 4.3 Implement winner resolution: `winner_under_prefix(entity_prefix)` + tombstone check; expose `Block` / `Edge` via existing public methods
- [x] 4.4 Refactor `children_of` to scan 17-byte navigation prefix and resolve per-source winners under 33-byte entity prefixes
- [x] 4.5 Refactor invariant filtering on active winners (drop invalid edges/blocks; recompute root)
- [x] 4.6 Add `edges_with_prefix(&[u8]) -> impl Iterator<Item = &Edge>` on `Snapshot` (active edges only)
- [x] 4.7 Verify all existing `snapshot.rs` tests pass unchanged

## 5. Snapshot structural diff

- [x] 5.1 Define `SnapshotDiffEntry` (entity identity + `DiffKind` + optional old/new active values)
- [x] 5.2 Implement `Snapshot::diff(&self, &other) -> impl Iterator<Item = SnapshotDiffEntry>` chaining block and edge trie diff iterators with entity-prefix decoding
- [x] 5.3 Tests: identical histories → empty iterator + shared trie roots; one property change → single yielded entry; early exit after first difference

## 6. Export and incremental rematerialize (optional)

- [x] 6.1 Re-export `RadixTrieMap`, `DiffKind`, `TrieDiffEntry`, and key helpers from `crates/graph/src/lib.rs`
- [x] 6.2 Add incremental materialize: append new version records to prior snapshot tries; fall back to full rebuild when no prior snapshot exists
- [x] 6.3 Wire incremental path in `Session` rematerialize; test incremental vs full rebuild equivalence

## 7. Verification

- [x] 7.1 Run `cargo test -p graph` (and workspace tests if affected)
- [x] 7.2 Confirm no storage format or public CLI behavior changes beyond new optional snapshot APIs
