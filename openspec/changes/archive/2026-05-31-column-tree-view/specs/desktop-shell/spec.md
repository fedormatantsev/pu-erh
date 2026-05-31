## MODIFIED Requirements

### Requirement: Anti-default UI shell

The desktop window's main content surface MUST be the Block View defined by the `block-view` capability, rendering the current selected block. The Default Block View renders the current selected block as a three-column TreeView (parent, current block with siblings, children) with arrow-key navigation, as defined by `block-view`. Arrow-key navigation within the Block View is the TreeView's selection policy and is permitted; it is not a global keyboard shortcut menu. The desktop window MUST NOT include separate navigation-tree or sidebar chrome, file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy. Any surface beyond the Block View and its associated Properties View (per `block-view`) MUST be introduced only by an explicit future requirement.

#### Scenario: Main surface is the Block View

- **WHEN** the application renders its main content surface
- **THEN** it shows the Block View rendering the current selected block as defined by `block-view`
- **AND** the Default Block View's three-column TreeView appears as Block View content, not as a separate sidebar or window chrome

#### Scenario: No invented chrome

- **WHEN** the application renders
- **THEN** it does not add file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy

### Requirement: Block-view IPC commands

The desktop shell MUST expose Tauri invoke commands sufficient for the Block View framework, each a thin wrapper over `core::Session`: read a block, read a block's parent, read a block's children, set a block property, and save. Read commands MUST mirror the session read APIs and the `parent:` and `children:` queries. Errors MUST be derived from `CoreError` without friendly rewriting, consistent with the existing commands.

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

#### Scenario: Read block command

- **WHEN** the frontend invokes the read-block command with a block id present in the session
- **THEN** the backend returns that block's id and properties

#### Scenario: Set property command

- **WHEN** the frontend invokes the set-property command with a block id, key, and value
- **THEN** the backend calls `Session::set_property` and the in-memory block reflects the new property
- **AND** an invalid request fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Save command

- **WHEN** the frontend invokes the save command
- **THEN** the backend calls `Session::save`
