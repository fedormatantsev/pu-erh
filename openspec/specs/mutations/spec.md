# mutations Specification

## Purpose

Defines user-facing graph mutations (create, move, delete) as append-only version-record operations validated against the current active view before trie insertion.
## Requirements
### Requirement: Create block

The system MUST support creating a new block with a required parent by appending block and edge version records. Creating a block without a parent MUST be rejected.

#### Scenario: Create without parent is forbidden

- **WHEN** a block is created without a parent
- **THEN** the system returns an error and no version records are appended

#### Scenario: Create child block

- **WHEN** a block is created with a parent id present in the snapshot
- **THEN** block and parent edge version records are appended
- **AND** after append the active view has a `parent` edge from the new block to the parent id via per-call CRDT winner selection

#### Scenario: Create with nonexistent parent

- **WHEN** a block is created with a parent id not present in the snapshot
- **THEN** the system returns an error and no version records are appended

### Requirement: Move block

The system MUST support reparenting by appending new edge version records (tombstone old parent edge, add new parent edge). A block MUST NOT be moved to root.

#### Scenario: Move to new parent

- **WHEN** move is applied to block `<id>` with an existing parent `<parent>`
- **THEN** edge version records are appended such that the snapshot has a `parent` edge for `<id>` targeting `<parent>`

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
- **AND** after append the active view has a `parent` edge for the block targeting the new parent

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

