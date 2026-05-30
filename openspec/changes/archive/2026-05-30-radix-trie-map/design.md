## Context

`Snapshot::materialize` currently builds two `HashMap`s — `HashMap<Uuid, Block>` and `HashMap<EdgeKey, Edge>` — then runs invariant filtering (drop bad edges, recompute root). Edge keys today use a string composite `{target}{type}{source}` (`EdgeKey`); this change replaces that with fixed binary keys for trie storage. Every mutation rematerializes the full snapshot, duplicating unchanged entries even when history append only touched a few entities.

The graph domain maps naturally to byte-keyed tries. Each version record is keyed by entity identity followed by CRDT metadata (`version`, `digest`, `previous_digest`) so lexicographic order matches the read-time winner rule (max version, then max digest). Winner lookup under an entity prefix is a bounded-depth walk (prefix descent, then repeated last-child descent) — not a full prefix scan. Prefix scans on the identity portion support tree navigation (`children_of` via the 17-byte `{target, type}` prefix).

## Goals / Non-Goals

**Goals:**

- Generic `RadixTrieMap<V>` keyed by byte slices (`&[u8]`) with inner/leaf node layout
- Compact representation: no inner-node chain for a lone entry; inner nodes appear only at byte positions where keys diverge
- Persistent (structural sharing): copy-on-write via `Arc`; unchanged subtrees reused across updates and snapshot versions
- O(key length) exact lookup; prefix iteration for edge queries aligned with binary edge key layout
- Lazy structural diff iteration between two tries using pointer equality (`Arc::ptr_eq`) to skip identical subtrees
- Fuse CRDT ordering into trie keys: `version`, `digest`, and `previous_digest` suffix bytes; winner selection via bounded-depth prefix + last-child descent
- Replace `Snapshot` block/edge `HashMap`s with trie-backed version-record storage without changing external graph semantics
- Preserve all existing invariant filtering behavior (silent drop of invalid entities)

**Non-Goals:**

- Changing version history storage format or conflict resolution rules
- Suffix/reverse-index queries (e.g. efficient global `parent_of` scan without point key)
- Hash-consing / intern pool across unrelated snapshots (follow-up optimization)
- Exposing trie diff in CLI in v1
- DataFusion integration
- Serialization of trie nodes to disk

## Decisions

### 1. Node layout

**Choice:**

```text
enum Node<V> {
    Inner {
        children_mask: u256,           // 256-bit occupancy bitmask
        children: Vec<Arc<Node<V>>>,   // sparse: only set bit indices, in ascending order
    },
    Leaf {
        key: Box<[u8]>,                // full key bytes (not suffix-only)
        value: V,
    },
}
```

Root is `Option<Arc<Node<V>>>` (empty map = `None`).

**Rationale:** Matches user spec: 256-way branching, sparse child storage, explicit leaf keys. Storing the full key in each leaf simplifies iteration, diff reporting, and correctness checks without walking from root.

**Alternatives considered:**
- Suffix-only leaves (Patricia-style) — smaller but harder diff/iteration; rejected for v1 clarity
- Fixed `[Option<Arc<Node>>; 256]` — wastes memory on sparse maps; rejected

### 2. Compact insertion (no unnecessary inner chains)

**Choice:** On insert into empty trie, attach a single leaf at root. On insert that shares a prefix with an existing leaf, split at the first differing byte: create one inner node at that depth with two leaf children (and further inner nodes only as needed). Never pre-allocate inner nodes for unshared prefix bytes.

**Rationale:** A lone block or edge costs one leaf node, not 16+ inner nodes for UUID length.

### 3. Child indexing in inner nodes

**Choice:** `children_mask` is a 256-bit bitmask (e.g. two `u128`s or `[u64; 4]`). `children` vec holds `popcount(mask)` entries in ascending byte-index order. Child at byte `b` is at `children[rank(b, mask)]`.

**Rationale:** O(1) bit test, cache-friendly sparse storage, standard radix trie layout.

### 4. Persistence model

**Choice:** All nodes wrapped in `Arc`. `insert`, `remove`, and `insert_batch` return a new `RadixTrieMap` sharing unchanged `Arc` subtrees. `Clone` is shallow (increments root `Arc` only).

**Rationale:** Enables structural sharing for snapshot rematerialization and cheap snapshot clones.

**Session integration:** After validating a mutation, build the next snapshot trie by cloning the prior trie and applying only changed keys (block/edge winners that differ from previous snapshot). Fall back to full rebuild from winners when no prior snapshot exists.

### 5. Edge type enum

**Choice:** Introduce `EdgeType` as a `#[repr(u8)]` enum (v0: `Parent = 0`). Edge records in version history and the materialized `Edge` struct continue to expose type as a string at the API/storage boundary; trie keys and prefix helpers use the enum byte directly.

**Rationale:** A single-byte type tag keeps edge keys fixed-width and avoids UTF-8 string allocation or parsing on the trie hot path. Unknown type strings from history fail conversion at materialization and are dropped (consistent with silent invariant filtering).

**Alternatives considered:**
- String type in key (current `EdgeKey` layout) — variable length, heap allocation, multibyte trie branching; rejected
- `as_str().as_bytes()` without retaining the string — still allocates the composite string; rejected

### 6. Key encodings for graph entities

**Choice:** Write keys directly into fixed byte buffers — no intermediate `String` or `str`. Every trie entry corresponds to one version record. Keys have two parts: **entity identity prefix** (for grouping and navigation queries) and **CRDT suffix** (for branch ordering and uniqueness).

**CRDT suffix** (shared by blocks and edges, 72 bytes):

| Field | Encoding | Length |
|-------|----------|--------|
| `version` | big-endian `u64` | 8 |
| `digest` | raw BLAKE3 bytes | 32 |
| `previous_digest` | raw BLAKE3 bytes; 32 zero bytes when `None` | 32 |

Big-endian `version` ensures lexicographic key order matches numeric version order. Appending `digest` after `version` implements the existing tie-break (lexicographic max digest wins). `previous_digest` is included for causal identity and diffing branches; it does not participate in winner ordering beyond key uniqueness.

**Full keys:**

| Entity | Layout | Length |
|--------|--------|--------|
| Block | `id.as_bytes()` ∥ CRDT suffix | 88 |
| Edge | `target.as_bytes()` ∥ `edge_type as u8` ∥ `source.as_bytes()` ∥ CRDT suffix | 105 |

**Entity identity prefixes** (CRDT suffix omitted — used for prefix scans only):

| Prefix | Layout | Length |
|--------|--------|--------|
| Block entity | `id.as_bytes()` | 16 |
| Edge entity | `target.as_bytes()` ∥ `edge_type as u8` ∥ `source.as_bytes()` | 33 |
| Edge navigation | `target.as_bytes()` ∥ `edge_type as u8` | 17 |

```rust
// block_version_key: [u8; 88]
key[0..16].copy_from_slice(id.as_bytes());
write_be_u64(&mut key[16..24], version);
key[24..56].copy_from_slice(&digest);
key[56..88].copy_from_slice(previous_digest.as_deref().unwrap_or(&[0u8; 32]));

// edge_version_key: [u8; 105]
key[0..16].copy_from_slice(target.as_bytes());
key[16] = edge_type as u8;
key[17..33].copy_from_slice(source.as_bytes());
write_be_u64(&mut key[33..41], version);
key[41..73].copy_from_slice(&digest);
key[73..105].copy_from_slice(previous_digest.as_deref().unwrap_or(&[0u8; 32]));
```

**Winner selection:** Under a fixed entity identity prefix, the winning version record is the leaf with the lexicographically greatest full key (max `version`, then max `digest`). Because CRDT fields are appended after the identity prefix in sort order, the winner can be found without scanning all entries under the prefix:

1. **Prefix descent** — Starting at the root, follow one child per byte of the entity prefix. This takes at most `prefix.len()` steps and lands on either an inner node (typical: multiple version records share the identity) or a lone leaf (single version for that entity).
2. **Last-child descent** — From that node, while the current node is inner, follow the **last child** (highest occupied byte index in `children_mask`). Repeat until a leaf is reached.
3. **Result** — The leaf reached is the winner. Total work is at most **trie depth** node lookups (≤ full key length: 88 for blocks, 105 for edges), not proportional to the number of version records under the prefix.

Active snapshot reads take that record unless it is tombstoned.

```text
winner_under_prefix(prefix):
  node ← root
  for b in prefix:                        // ≤ prefix.len steps
    node ← child(node, b) ?? return None
  while node is Inner:                     // ≤ (key_len - prefix.len) steps
    node ← last_child(node)
  return leaf value if node is Leaf else None
```

`last_child(inner)` selects the child at the highest set bit in `children_mask`. This is equivalent to the last entry in `iter_prefix(entity_prefix)` order but avoids enumerating intermediate leaves.

**Rationale:** Embedding CRDT fields in the key lets the trie hold branched history while read-time conflict resolution is O(trie depth) instead of a separate `HashMap` winner pass or O(k) prefix scan over k version records.

**Alternatives considered:**
- Identity-only keys with winner chosen out-of-band — reintroduces a separate selection pass; rejected
- JSON or string encoding of CRDT fields — variable length, allocation; rejected

### 7. Snapshot API changes

**Choice:** Replace internal `HashMap`s with `RadixTrieMap<BlockVersion>` and `RadixTrieMap<EdgeVersion>` (or equivalent version-record values). `materialize` inserts **every** version record from history using full CRDT keys — no separate winner `HashMap` pass. Active-entity reads resolve the winner via `winner_under_prefix(entity_prefix)` (prefix descent + last-child descent, O(trie depth)):

- `block(id)`: `winner_under_prefix` on 16-byte id prefix; omit if tombstoned
- `get_edge(...)`: `winner_under_prefix` on 33-byte edge entity prefix; omit if tombstoned
- `children_of(parent)`: enumerate distinct source ids under 17-byte navigation prefix (prefix iteration); for each, `winner_under_prefix` on the corresponding 33-byte entity prefix; keep non-tombstoned parent edges

Keep existing public method signatures returning `Block` / `Edge` (not version records).

Add optional v1 methods:
- `Snapshot::edges_with_prefix(&[u8]) -> impl Iterator<Item = &Edge>` (active edges only)
- `RadixTrieMap::diff(&self, &other) -> impl Iterator<Item = TrieDiffEntry<'_, V>>` — lazy iterator over diverged entries (added / removed / changed); no upfront collection
- `RadixTrieMap::winner_under_prefix(&[u8]) -> Option<(&[u8], &V)>` — O(trie depth) winner lookup used by snapshot reads

**Rationale:** Prefix iteration replaces linear scan over all edges for children queries. A diff iterator avoids allocating vectors for unchanged maps and lets callers stop early. External behavior unchanged for existing callers.

### 8. Invariant filtering with tries

**Choice:** Keep filtering as a post-materialization pass (same logic as today), mutating by removing entries from tries (persistent remove returning new snapshot). Do not yet encode cycle detection into trie structure.

**Parent uniqueness:** Still enforced by dropping conflicting parent edges after winner resolution (same as current `drop_conflicting_parent_edges`). The 33-byte edge entity prefix ensures at most one winning edge per `(target, type, source)` triple.

**Rationale:** Correctness first; trie layout does not eliminate need for cycle/root checks in v1. User's "fuse invariants" goal applies to key-uniqueness and prefix-scoped navigation; full invariant fusion is incremental.

### 9. Structural diff algorithm

**Choice:** Expose diff as a lazy iterator, not a materialized diff struct. Walk two `Arc<Node>` roots depth-first; yield `TrieDiffEntry` items only at divergence points:

```rust
enum DiffKind { Added, Removed, Changed }

struct TrieDiffEntry<'a, V> {
    kind: DiffKind,
    key: &'a [u8],
    old: Option<&'a V>,  // self (left)
    new: Option<&'a V>,  // other (right)
}
```

Algorithm (stack- or generator-driven):

1. If `Arc::ptr_eq(a, b)` → subtrees equal; stop (yield nothing)
2. If node kinds differ → yield all leaves in the absent subtree as `Added` or `Removed`, then stop
3. Inner vs inner: XOR masks; recurse into children present in only one tree (yield whole subtree leaves) or both (recurse)
4. Leaf vs leaf at same path: if keys equal and values differ → yield `Changed`; if keys differ → yield `Removed` + `Added`

`Snapshot::diff` chains the block and edge trie diff iterators, decoding keys to `Uuid` / edge identity at yield time.

**Rationale:** Pointer equality skips unchanged subtrees with zero allocation. Iterator API matches prefix iteration style and supports early exit (e.g. "any change?" without full enumeration).

**Alternatives considered:**
- `TrieDiff { added, removed, changed: Vec<...> }` — forces full walk and allocation even when caller needs one difference; rejected

### 10. Module placement

**Choice:** New module `crates/graph/src/radix_trie.rs` (or `trie/mod.rs`) exported from `graph` crate. `Snapshot` depends on it; no new crate dependency.

**Rationale:** Trie is graph-adjacent but generic enough to reuse; keeps monorepo structure simple.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| `parent_of` still requires winner lookup under full 33-byte edge entity prefix | O(trie depth) via `winner_under_prefix`; suffix index is follow-up |
| Unknown edge type strings in history | Map to `EdgeType` at materialization; drop edges with unmapped types |
| Persistent remove/insert path copying | Only O(depth) nodes copied; depth ≤ key length (≤ 105 for edges) |
| Incremental snapshot update bugs | Comprehensive tests: incremental vs full rebuild must match |
| Leaf stores full key (memory overhead) | Trade memory for simpler API; keys are bounded (88 or 105 bytes) |
| Multiple version records per entity increase trie size | Structural sharing across snapshots and prefix-local winner reads; compaction is follow-up |

## Migration Plan

Internal refactor only. No storage migration. Steps:

1. Implement `RadixTrieMap` with tests
2. Swap `Snapshot` internals; verify all existing snapshot tests pass
3. Add prefix iteration + diff tests
4. Optional: wire incremental trie update in `Session::rematerialize`

Rollback: revert to `HashMap` internals (no persisted format impact).

## Open Questions

- **Incremental rematerialize in v1 or v2?** Recommend v1 full rebuild + persistent node reuse on clone; incremental update in same change if time permits (tasks will include both paths with full rebuild as fallback).
- **Public export of `RadixTrieMap`?** Recommend `pub` in `graph` crate for future query engine use.
- **Ordered iteration?** Lexicographic byte order via DFS; CRDT suffix ordering makes `winner_under_prefix` (last-child descent after prefix) equivalent to max-version/max-digest winner selection in O(trie depth).
