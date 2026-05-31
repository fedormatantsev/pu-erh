## Context

Children of a block currently have no defined sibling order. The `children()` method returns siblings in an order determined by trie-key insertion sequence, which is deterministic within a single session but may differ between peers after replication merge. There is no user-controlled or CRDT-stable ordering mechanism.

Parent edges already carry a `properties: BTreeMap<String, PropertyValue>` map. That map participates in BLAKE3 digest computation and round-trips through storage format v2 unchanged. The infrastructure for storing an `order` property on parent edges already exists at zero schema cost.

## Goals / Non-Goals

**Goals:**
- Stable, user-controlled sibling order for children of any block.
- CRDT-safe: concurrent inserts from different peers produce non-conflicting `order` values that merge without coordination.
- No storage-format version bump — `order` is a regular `PropertyValue::String` edge property.
- A `children_ordered(parent_id)` API at the graph layer, returning children sorted by their parent-edge `order` value.

**Non-Goals:**
- UI for drag-and-drop or visual reordering (out of scope for this change).
- Ordering of non-`Parent` edges.
- Retroactive migration of existing blocks to assigned `order` values (empty-string sentinel handles legacy records).
- Automatic CRDT conflict resolution beyond tie-breaking by UUID.

## Decisions

### Decision 1: `order` stored as `PropertyValue::String` on the parent edge

The parent edge `properties` map already accepts `PropertyValue::String`. Using it means:
- No changes to the block model, edge model, version record format, or storage schema.
- BLAKE3 digest computation already covers edge properties, so `order` is authenticated at no extra cost.
- Storage format v2 round-trips the value unchanged.

Alternative: a dedicated `order` field on the edge struct. Rejected — requires a storage format bump and invasive model changes throughout the stack.

### Decision 2: jitl fractional-indexing string algorithm

The algorithm (originally by figma/jitl) produces compact, lexicographically sortable strings such as `"a0"`, `"V"`, `"Zz|"` with the invariant `key(left) < key(midpoint) < key(right)` under string comparison. Keys grow logarithmically in the number of successive insertions between two fixed neighbors — manageable in practice.

Alternatives considered:
- Rational number strings (e.g. `"1/3"`): unbounded length on repeated subdivision.
- Timestamps: clock drift causes ordering conflicts between peers.
- Lamport counters: conflict on concurrent inserts from different peers at the same position.

Implementation: a minimal `fractional_index` module in `crates/graph` (or a workspace crate `crates/fractional-index`) exposing `generate_key_between(a: Option<&str>, b: Option<&str>) -> Result<String, OrderError>`. If a well-maintained Rust crate exists on crates.io, adopt it instead.

### Decision 3: `PositionHint` enum threads through create/move

```rust
pub enum PositionHint {
    First,
    Last,           // default when caller omits position
    Before(BlockId),
    After(BlockId),
}
```

The graph layer resolves the hint: looks up the adjacent sibling's `order` property from the active view, then calls `generate_key_between` with the appropriate `(left, right)` pair.

CLI: `--before <uuid>`, `--after <uuid>`, `--first`, `--last` (default `--last`).

### Decision 4: Missing `order` treated as `""` (empty-string sentinel)

Blocks whose parent edge has no `order` property (created before this change) are sorted as if `order = ""`. The empty string sorts before all valid fractional-index keys (the algorithm's character space begins with printable ASCII above `"`), placing legacy blocks at the start. This is deterministic and requires no mutation of existing version records.

Alternative: assign `order` values via a migration mutation on first open. Rejected for v0 — unnecessary complexity; legacy blocks at the start is an acceptable temporary state.

### Decision 5: Tie-breaking by child block UUID

Two concurrent peers inserting at the same position independently call `generate_key_between` with identical `(left, right)` arguments and may produce the same key. Tie-breaking sorts ascending by child `BlockId` bytes. This is deterministic, requires no coordination, and is acceptable for v0.

## Risks / Trade-offs

- **Algorithmic panic on invalid input** → `generate_key_between` validates that `a < b` and both are valid fractional-index strings; returns `Err(OrderError)` on invalid input rather than panicking.
- **String length growth** → The jitl algorithm keeps keys short under typical usage; document the O(log n) length growth bound in module comments.
- **Legacy blocks sort first** → A user who opens an existing knowledge base will see un-ordered blocks at the top. Acceptable for v0; a future migration mutation or explicit reorder is the remedy.
- **Concurrent same-position inserts** → Two peers insert at the same slot; their keys may collide. UUID tie-breaking produces stable but potentially surprising order. Acceptable for v0.

## Migration Plan

1. Deploy the change. No storage file is modified; existing `order`-less parent edges continue to work via the empty-string sentinel.
2. New `create` and `move` calls write `order` onto the parent edge. The knowledge base becomes gradually ordered as blocks are created or moved.
3. No rollback concern: `order` is an additive edge property. An older binary without this change simply ignores the `order` property and returns children in trie order.

## Open Questions

- Is there a well-maintained Rust fractional-indexing crate worth adopting, or should we implement the ~100-line jitl core ourselves?
- Should `children_ordered` be the only public children API, or should the unordered `children()` be retained for cases where order is irrelevant?
