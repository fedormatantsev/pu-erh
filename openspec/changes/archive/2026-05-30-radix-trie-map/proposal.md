## Why

The materialized `Snapshot` currently stores blocks and edges in `HashMap`s. That works for point lookups but wastes memory when many snapshots share most of their content (e.g. after each mutation rematerializes), offers no efficient prefix-based access for tree navigation, and requires full map comparison to detect unchanged subtrees. A compact, persistent radix trie with fixed binary keys enables structural sharing, O(key-length) lookups by full key or prefix, and cheap snapshot diffs via pointer equality on shared inner nodes.

## What Changes

- Introduce a generic **radix trie map** (`RadixTrieMap<K, V>`) with inner nodes (256-way sparse child array + occupancy bitmask) and leaf nodes (key bytes + value).
- Keep the trie **compact**: a single entry does not allocate a chain of inner nodes; inner nodes appear only when multiple leaves share a byte prefix.
- Replace `Snapshot`'s `HashMap<Uuid, Block>` and `HashMap<EdgeKey, Edge>` with trie-backed maps holding version records keyed by entity identity plus CRDT suffix (`version`, `digest`, `previous_digest` as fixed bytes).
- Introduce `EdgeType` as a `#[repr(u8)]` enum for trie keys and prefix helpers (v0: `Parent`); string edge types remain at the API/storage boundary with conversion at materialization.
- **Fuse CRDT conflict resolution into trie ordering**: under a fixed entity prefix, the winner is found by prefix descent then last-child descent in O(trie depth); active reads skip tombstoned winners.
- Implement **persistent (copy-on-write) updates** so rematerialization shares unchanged subtrees with prior snapshots.
- Add **prefix query** APIs on the trie (and expose graph-relevant ones on `Snapshot`, e.g. edges by `{target}{type}` prefix, children via `parent` prefix).
- Encode selected **graph invariants in trie layout** where possible (e.g. winner-by-prefix-last replaces separate winner selection; prefix scans replace linear edge iteration for tree navigation).
- Add **structural diff** as lazy iterators over two tries (or two snapshots), walking only divergent branches via `ptr_eq` on shared nodes.

## Capabilities

### New Capabilities

- `radix-trie-map`: Generic compact persistent radix trie with inner/leaf nodes, insert/get/remove, prefix iteration, structural sharing, and lazy pointer-equality diff iteration.

### Modified Capabilities

- `immutable-snapshot`: Snapshot block/edge storage and navigation APIs backed by radix tries; prefix-based edge queries; structural diff between snapshots.

## Impact

- **`crates/graph`**: New `radix_trie` (or `trie`) module; refactor `Snapshot` internals and materialization path; possible new public query methods.
- **`crates/core` / `crates/cli`**: May adopt prefix queries or snapshot diff if exposed; otherwise unchanged behavior.
- **Tests**: New trie unit tests; update snapshot integration tests for equivalence with prior HashMap behavior.
- **No storage format change**: Version history persistence unchanged; only in-memory snapshot representation changes.
