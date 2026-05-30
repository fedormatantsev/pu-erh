# mutations Specification

## Purpose
TBD - created by archiving change walking-skeleton. Update Purpose after archive.
## Requirements
### Requirement: Create block

The system MUST support creating a new block with a required parent. Creating a block without a parent MUST be rejected.

#### Scenario: Create without parent is forbidden

- **WHEN** a block is created without a parent
- **THEN** the system returns an error and no block or edge is created

#### Scenario: Create child block

- **WHEN** a block is created with a parent id that exists in the graph
- **THEN** a new block is added
- **AND** a `parent` edge is created with source set to the new block and target set to the parent id

#### Scenario: Create with nonexistent parent

- **WHEN** a block is created with a parent id that does not exist
- **THEN** the system returns an error and no block or edge is created

### Requirement: Move block

The system MUST support reparenting an existing block by updating its `parent` edge. A block MUST NOT be moved to root (parent edge removed).

#### Scenario: Move to new parent

- **WHEN** move is applied to block `<id>` with an existing parent `<parent>`
- **THEN** the `parent` edge for `<id>` has target `<parent>` (replacing any previous parent edge for `<id>`)

#### Scenario: Move to root is forbidden

- **WHEN** move is applied to block `<id>` with no parent specified (or explicit root)
- **THEN** the system returns an error and the graph is unchanged

#### Scenario: Move creates cycle

- **WHEN** move would make a block an ancestor of itself (directly or indirectly)
- **THEN** the system returns an error and the graph is unchanged

#### Scenario: Move nonexistent block

- **WHEN** move is applied to an id that does not exist
- **THEN** the system returns an error

### Requirement: Delete block

The system MUST support deleting a block by id. The root block MUST NOT be deletable.

#### Scenario: Delete leaf block

- **WHEN** delete is applied to a block with no children
- **THEN** the block is removed from the graph
- **AND** all edges involving that block (as source or target) are removed

#### Scenario: Delete block with children

- **WHEN** delete is applied to a block that has direct children (incoming `parent` edges)
- **THEN** the system returns an error and the block remains in the graph

#### Scenario: Delete root block

- **WHEN** delete is applied to the root block
- **THEN** the system returns an error and the root block remains in the graph

#### Scenario: Delete nonexistent block

- **WHEN** delete is applied to an id that does not exist
- **THEN** the system returns an error

