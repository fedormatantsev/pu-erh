## Why

Children of a block have no defined order: the active view returns siblings in arbitrary (trie-insertion) order, making any rendered list non-deterministic and non-editable. A fractional-index string stored as a property on each parent edge provides a stable, CRDT-safe total order for children without integer counters that conflict under concurrent edits.

## What Changes

- Add a well-known `order` property to every parent edge, holding a fractional-index string (e.g. `"a0"`, `"Zz|"`) that encodes sibling position.
- When a block is created, the caller supplies an optional position (before/after a sibling or at start/end); the system computes and stores the `order` value on the new parent edge.
- When a block is moved, the new parent edge carries a freshly computed `order` value for the target sibling slot.
- The graph API exposes `children_ordered(parent_id)` returning children sorted ascending by their parent-edge `order` value. Ties (identical `order` on concurrent inserts) are broken by child block id for determinism.
- The `order` value is a pure string `PropertyValue::String` — it participates in digest computation automatically and requires no schema changes to edges or version records.

## Capabilities

### New Capabilities

- `child-ordering`: Defines the `order` well-known property on parent edges, fractional-index generation rules, ordered-children query semantics, and tie-breaking behavior.

### Modified Capabilities

- `mutations`: `create` and `move` MUST accept a sibling-position argument and write the computed `order` string onto the parent edge properties.
- `well-known-properties`: Add `order` to the well-known edge property registry (parent edges only).

## Impact

- `crates/graph`: new `children_ordered` method; `create` and `move` edge construction gains `order` property.
- `crates/core` / `Session`: mutation call sites updated to thread position argument through to graph layer.
- `crates/cli`: mutation commands (`create`, `move`) gain optional `--before`/`--after`/`--first`/`--last` flags; default is `--last`.
- New dependency: a fractional-indexing crate (e.g. `fractional-indexing` or equivalent) added to the workspace.
- No storage-format change — `order` is a regular edge property, already round-tripped under format version 2.
