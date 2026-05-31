## MODIFIED Requirements

### Requirement: Create block

The system MUST support creating a new block with a required parent and an optional `PositionHint` by appending block and edge version records. Creating a block without a parent MUST be rejected. The `PositionHint` defaults to `Last` when omitted. The new parent edge MUST have the `"order"` property set to the fractional-index string computed from the resolved sibling position (see child-ordering spec). `PositionHint::Before(id)` or `PositionHint::After(id)` referencing a block that is not a child of the given parent MUST be rejected.

#### Scenario: Create without parent is forbidden

- **WHEN** a block is created without a parent
- **THEN** the system returns an error and no version records are appended

#### Scenario: Create child block with default position

- **WHEN** a block is created with a parent id present in the snapshot and no position hint
- **THEN** block and parent edge version records are appended
- **AND** the parent edge version record has `"order"` set to a valid fractional-index string that sorts after all existing children of that parent

#### Scenario: Create child block with explicit position

- **WHEN** a block is created with a parent id present in the snapshot and a `PositionHint`
- **THEN** block and parent edge version records are appended
- **AND** the parent edge version record has `"order"` set to a value consistent with the requested position (see child-ordering: PositionHint for sibling placement)

#### Scenario: Create with nonexistent parent

- **WHEN** a block is created with a parent id not present in the snapshot
- **THEN** the system returns an error and no version records are appended

#### Scenario: Create with Before/After referencing non-child

- **WHEN** a block is created with `PositionHint::Before(id)` or `PositionHint::After(id)` where `id` is not a child of the target parent
- **THEN** the system returns an error and no version records are appended

### Requirement: Move block

The system MUST support reparenting a block by appending new edge version records (tombstone old parent edge, add new parent edge) with an optional `PositionHint` for the target position among the new parent's children. A block MUST NOT be moved to root. The `PositionHint` defaults to `Last` when omitted. The new parent edge MUST have the `"order"` property set to the fractional-index string computed from the resolved sibling position. `PositionHint::Before(id)` or `PositionHint::After(id)` referencing a block that is not a child of the target parent MUST be rejected.

#### Scenario: Move to new parent

- **WHEN** move is applied to block `<id>` with a target parent `<parent>` and no position hint
- **THEN** edge version records are appended such that the active view has a `parent` edge for `<id>` targeting `<parent>`
- **AND** the new parent edge has `"order"` set to a value that sorts after all existing children of `<parent>`

#### Scenario: Move to new parent with explicit position

- **WHEN** move is applied to block `<id>` with target parent `<parent>` and a `PositionHint`
- **THEN** the new parent edge has `"order"` consistent with the requested position

#### Scenario: Move to root is forbidden

- **WHEN** move is applied to block `<id>` with no parent specified
- **THEN** the system returns an error and no version records are appended

#### Scenario: Move creates cycle

- **WHEN** move would make a block an ancestor of itself
- **THEN** the system returns an error and no version records are appended

#### Scenario: Move nonexistent block

- **WHEN** move is applied to an id not present in the snapshot
- **THEN** the system returns an error

#### Scenario: Move block without existing parent edge

- **WHEN** move is applied to a block with no active parent edge
- **THEN** only a new parent edge version record is appended (no tombstone of a prior edge)
- **AND** the new parent edge has `"order"` set appropriately for the given position hint
- **AND** after append the active view has a `parent` edge for the block targeting the new parent

#### Scenario: Move with Before/After referencing non-child

- **WHEN** move is applied with `PositionHint::Before(id)` or `PositionHint::After(id)` where `id` is not a child of the target parent
- **THEN** the system returns an error and no version records are appended
