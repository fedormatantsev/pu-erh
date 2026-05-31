## 1. Fractional-Index Module

- [x] 1.1 Add `crates/graph/src/fractional_index.rs` implementing `generate_key_between(left: Option<&str>, right: Option<&str>) -> Result<String, OrderError>` based on the jitl algorithm
- [x] 1.2 Write unit tests for `generate_key_between`: key between two keys, before first, after last, first-only child, and invalid-input error
- [x] 1.3 Export the module from `crates/graph/src/lib.rs` (`pub use fractional_index::{generate_key_between, OrderError}`)

## 2. PositionHint Type

- [x] 2.1 Add `PositionHint` enum in `crates/graph/src/model.rs` with variants `First`, `Last`, `Before(Uuid)`, `After(Uuid)`
- [x] 2.2 Export `PositionHint` from `crates/graph/src/lib.rs`

## 3. Graph Layer: children_ordered and edge-order helpers

- [x] 3.1 Add `parent_edge(&self, child: Uuid) -> Option<Edge>` (or equivalent) to `KnowledgeBase` in `crates/graph/src/snapshot.rs` to expose the active parent edge with its properties
- [x] 3.2 Add `child_order(&self, child: Uuid) -> &str` helper that reads `"order"` from the child's parent-edge properties, returning `""` when absent
- [x] 3.3 Add `children_ordered(&self, parent: Uuid) -> Vec<Uuid>` to `KnowledgeBase`: call `children_of(parent)`, sort by `(child_order(c), c)` ascending, return sorted ids
- [x] 3.4 Write unit tests for `children_ordered`: ordered by `order` property, legacy blocks (no `order`) sort first, tie-breaking by UUID

## 4. Graph Layer: resolve_position helper

- [x] 4.1 Add `resolve_position(&self, parent: Uuid, hint: PositionHint) -> Result<(Option<String>, Option<String>), GraphError>` to `KnowledgeBase`: returns the `(left_order, right_order)` pair for `generate_key_between` based on the hint and the parent's current ordered children; returns `GraphError` for `Before`/`After` referencing a non-child

## 5. Update Mutations

- [x] 5.1 Update `create_block` in `crates/core/src/mutation.rs` to accept `position: PositionHint`; resolve the position via `resolve_position`, call `generate_key_between`, and set `"order"` on the new parent edge's `Properties`
- [x] 5.2 Update `move_block` in `crates/core/src/mutation.rs` to accept `position: PositionHint`; compute and set `"order"` on the new parent edge the same way
- [x] 5.3 Add error variant to `CoreError` for invalid position hint (e.g. `PositionSiblingNotFound(Uuid)`) if not already covered by `GraphError`
- [x] 5.4 Update existing mutation tests to pass `PositionHint::Last` where `create_block` and `move_block` are called; add new tests covering `First`, `Before`, `After`, and invalid sibling rejection

## 6. Update Session

- [x] 6.1 Update `Session::create_block` signature in `crates/core/src/session.rs` to accept `position: PositionHint` and thread it through to `mutation::create_block`
- [x] 6.2 Update `Session::move_block` signature in `crates/core/src/session.rs` to accept `position: PositionHint` and thread it through to `mutation::move_block`

## 7. Update CLI

- [x] 7.1 Add `--before <UUID>`, `--after <UUID>`, `--first`, `--last` flags to the `Create` command in `crates/cli/src/main.rs`; parse them into a `PositionHint` (default `Last`); pass to `session.create_block`
- [x] 7.2 Add the same position flags to the `Move` command; pass to `session.move_block`
- [x] 7.3 Return an error if conflicting position flags are provided (e.g. `--first --last`)
