## MODIFIED Requirements

### Requirement: Block-view IPC commands

The desktop shell MUST expose Tauri invoke commands sufficient for the Block View framework, each a thin wrapper over `core::Session`: read a block, read a block's parent, read a block's children, read a block's children in `child-ordering` order, create a child block, set a block property, delete a block, move a block, and save. Read commands MUST mirror the session read APIs and the `parent:` and `children:` queries. The ordered-children command MUST return the block's children in the order defined by the `child-ordering` `order` property (mirroring `KnowledgeBase::children_ordered`), since the unordered `children` command and `children:` query do not expose edge order to the frontend. The create-child command MUST call `core::Session::create_block` with a required parent; it MUST accept an optional sibling-relative position so a child can be created immediately after a given sibling (defaulting to last when no position is given) and MUST NOT save. The delete command MUST call `core::Session::delete_block` and MUST NOT save. The move command MUST call `core::Session::move_block` with a sibling-relative position and MUST NOT save. Errors MUST be derived from `CoreError` without friendly rewriting, consistent with the existing commands.

#### Scenario: Read parent command

- **WHEN** the frontend invokes the parent command with a block id that has a parent in the session
- **THEN** the backend returns that block's parent, carrying its id, properties, and whether it has children
- **AND** the result matches the `parent:` query over the in-process session

#### Scenario: Read parent command at the root

- **WHEN** the frontend invokes the parent command with the root block id, which has no parent
- **THEN** the backend returns no parent block, matching the `parent:` query result for a block with no parent

#### Scenario: Read children command

- **WHEN** the frontend invokes the children command with a block id present in the session
- **THEN** the backend returns that block's children, each carrying its id, properties, and whether it has children
- **AND** the result matches the `children:` query over the in-process session

#### Scenario: Read ordered children command

- **WHEN** the frontend invokes the ordered-children command with a block id present in the session
- **THEN** the backend returns that block's children sorted by the `child-ordering` `order` property
- **AND** each child carries its id, properties, and whether it has children

#### Scenario: Read block command

- **WHEN** the frontend invokes the read-block command with a block id present in the session
- **THEN** the backend returns that block's id and properties

#### Scenario: Create child command

- **WHEN** the frontend invokes the create-child command with a parent block id present in the session
- **THEN** the backend calls `Session::create_block` with that parent and appends block and parent edge version records in memory
- **AND** no save occurs
- **AND** an invalid request (for example a nonexistent parent) fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Create child at a sibling-relative position

- **WHEN** the frontend invokes the create-child command with a parent and a position immediately after a given sibling
- **THEN** the backend calls `Session::create_block` with that parent and the corresponding `PositionHint`
- **AND** the new child's `child-ordering` `order` places it immediately after the given sibling
- **AND** no save occurs

#### Scenario: Delete block command

- **WHEN** the frontend invokes the delete command with a block id present in the session
- **THEN** the backend calls `Session::delete_block` and the block is removed in memory
- **AND** no save occurs
- **AND** an invalid request fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Move block command

- **WHEN** the frontend invokes the move command with a block id and a sibling-relative position
- **THEN** the backend calls `Session::move_block` with the corresponding `PositionHint` and the block's order reflects the new position in memory
- **AND** no save occurs
- **AND** an invalid request fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Set property command

- **WHEN** the frontend invokes the set-property command with a block id, key, and value
- **THEN** the backend calls `Session::set_property` and the in-memory block reflects the new property
- **AND** an invalid request fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Save command

- **WHEN** the frontend invokes the save command
- **THEN** the backend calls `Session::save`
