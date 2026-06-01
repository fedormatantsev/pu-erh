# mutations Specification

## Purpose

Defines user-facing graph mutations (create, move, delete) as append-only version-record operations validated against the current active view before trie insertion.
## Requirements
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

### Requirement: Delete block

The system MUST support deleting a block by appending a tombstoned block version record. The root block MUST NOT be deletable.

#### Scenario: Delete leaf block

- **WHEN** delete is applied to a block with no children
- **THEN** a tombstoned block version record is appended
- **AND** if an active parent edge exists, a tombstoned parent edge version record is appended
- **AND** after append the block is absent from the active view via per-call CRDT winner selection

#### Scenario: Delete block with children

- **WHEN** delete is applied to a block with direct children in the snapshot
- **THEN** the system returns an error and no version records are appended

#### Scenario: Delete root block

- **WHEN** delete is applied to the root block
- **THEN** the system returns an error and no version records are appended

#### Scenario: Delete nonexistent block

- **WHEN** delete is applied to an id not present in the snapshot
- **THEN** the system returns an error

### Requirement: Mutations append version records

Successful mutations MUST append new block and/or edge version records directly to the version tries. The active view reflects appended records via per-call CRDT winner selection without a full-history rematerialization pass. Failed mutations MUST NOT append records.

#### Scenario: Successful create appends records

- **WHEN** a block is successfully created
- **THEN** a new block version record and a new parent edge version record are appended

#### Scenario: Successful delete appends tombstone

- **WHEN** a leaf block is successfully deleted
- **THEN** a new tombstoned block version record is appended

#### Scenario: Failed mutation appends nothing

- **WHEN** a mutation fails validation
- **THEN** no version records are appended
- **AND** the snapshot is unchanged

### Requirement: Set block property

The system MUST support setting a single property on an existing block via a generic `set_property(block, key, value)` mutation, where `value` is a `PropertyValue`. The mutation MUST append a new block version record whose properties are the block's current properties with `key` set to `value`. Setting a property on a block not present in the active view MUST be rejected. The mutation MUST validate against the active view before appending, consistent with create/move/delete.

#### Scenario: Set a property on an existing block

- **WHEN** `set_property` is applied to a block present in the snapshot with a key and a `PropertyValue`
- **THEN** a new block version record is appended whose properties equal the block's prior properties with that key set to the value
- **AND** after append the active block reflects the new property via per-call CRDT winner selection

#### Scenario: Overwrite an existing property key

- **WHEN** `set_property` is applied with a key the block already has
- **THEN** the appended block version replaces that key's value with the new value
- **AND** other properties are unchanged

#### Scenario: Set property on nonexistent block

- **WHEN** `set_property` is applied to an id not present in the snapshot
- **THEN** the system returns an error and no version records are appended

### Requirement: Remove block property

The system MUST support removing a single property from an existing block via a `remove_property(block_id, key)` mutation. The mutation MUST append a new block version record whose properties are the block's current properties with `key` absent. Removing a property from a block not present in the active view MUST be rejected. Removing a key that is not present in the block's current properties MUST be rejected. The mutation MUST validate against the active view before appending, consistent with `set_property`.

#### Scenario: Remove an existing property

- **WHEN** `remove_property` is applied to a block present in the snapshot with a key that exists in the block's properties
- **THEN** a new block version record is appended whose properties equal the block's prior properties with that key absent
- **AND** after append the active block no longer has that property via per-call CRDT winner selection

#### Scenario: Remove property on nonexistent block

- **WHEN** `remove_property` is applied to an id not present in the snapshot
- **THEN** the system returns an error and no version records are appended

#### Scenario: Remove nonexistent property key

- **WHEN** `remove_property` is applied to a block present in the snapshot with a key that does not exist in the block's properties
- **THEN** the system returns an error and no version records are appended

