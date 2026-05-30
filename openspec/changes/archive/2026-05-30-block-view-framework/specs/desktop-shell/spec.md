## ADDED Requirements

### Requirement: Block-view IPC commands

The desktop shell MUST expose Tauri invoke commands sufficient for the Block View framework, each a thin wrapper over `core::Session`: read a block, read a block's children, set a block property, and save. Read commands MUST mirror the session read APIs and the `children:` query. Errors MUST be derived from `CoreError` without friendly rewriting, consistent with the existing commands.

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

## MODIFIED Requirements

### Requirement: No unspecified save policy

The desktop shell MUST NOT invoke `Session::save` on a timer, on mutation, or on window close. The shell MAY expose a single explicit Save control (a button or invoke command) that calls `Session::save` only when the user invokes it. No automatic or implicit save trigger is permitted.

#### Scenario: Window close without save

- **WHEN** the user closes the Tauri window
- **THEN** the process exits without an automatic save-on-close step

#### Scenario: Explicit save only

- **WHEN** the user changes a block property in the Properties View
- **THEN** the change is held in memory and is not persisted to disk
- **AND** it is written only when the user invokes the explicit Save control calling `Session::save`

### Requirement: Anti-default UI shell

The desktop window's main content surface MUST be the Block View defined by the `block-view` capability, rendering the current selected block. The Default Block View MAY render the current block's children as a recursive tree with expand/collapse controls within the Block View content. The desktop window MUST NOT include separate navigation-tree or sidebar chrome, file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy. Any surface beyond the Block View and its associated Properties View (per `block-view`) MUST be introduced only by an explicit future requirement.

#### Scenario: Main surface is the Block View

- **WHEN** the application renders its main content surface
- **THEN** it shows the Block View rendering the current selected block as defined by `block-view`
- **AND** the Default Block View's children tree appears as Block View content, not as a separate sidebar or window chrome

#### Scenario: No invented chrome

- **WHEN** the application renders
- **THEN** it does not add file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy
