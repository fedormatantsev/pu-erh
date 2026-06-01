# desktop-shell Specification

## Purpose

Defines the Tauri desktop shell: `crates/desktop` adapter, in-process `core::Session`, minimal IPC commands, and anti-default constraints (no auto-save, no invented product UX).
## Requirements
### Requirement: Desktop crate in Cargo workspace

The repository MUST include a `crates/desktop` library crate as a member of the root Cargo workspace.

#### Scenario: Workspace builds desktop crate

- **WHEN** a developer runs `cargo build -p desktop`
- **THEN** the desktop crate compiles successfully against `pu-erh-core`

### Requirement: Tauri application host

The repository MUST include a Tauri 2 application under `apps/desktop/src-tauri` that launches a webview window and loads the React frontend built by Vite.

#### Scenario: Dev mode launches window

- **WHEN** a developer runs the documented desktop dev command from the repo root
- **THEN** a Tauri window opens displaying the React frontend
- **AND** the webview loads assets from the Vite dev server or built output as configured in `tauri.conf.json`

### Requirement: In-process session ownership

The Tauri process MUST hold exactly one `core::Session` for the application lifetime, opened at startup via the desktop adapter.

#### Scenario: Session opened at startup

- **WHEN** the Tauri application starts
- **THEN** the desktop adapter calls `Session::open` with a deterministic storage path under the application data directory
- **AND** the session remains available until process exit

#### Scenario: No duplicated domain logic

- **WHEN** the desktop adapter performs session operations
- **THEN** it MUST call `core::Session` (or shared helpers in `crates/desktop`) only
- **AND** MUST NOT reimplement mutation validation, trie logic, or storage format handling in the Tauri crate

### Requirement: Minimal IPC commands

The desktop shell MUST expose scaffold Tauri invoke commands that prove Rust-to-frontend wiring without implementing unspecified product flows.

#### Scenario: Ping command

- **WHEN** the frontend invokes the `ping` command
- **THEN** the backend returns a non-empty fixed string
- **AND** no knowledge base mutation occurs

#### Scenario: Root id command

- **WHEN** the frontend invokes the `root_id` command and the session has a persisted root block
- **THEN** the backend returns the root block UUID as a string
- **AND** the value matches `Session::root_id()` from the in-process session

#### Scenario: Root id before root exists

- **WHEN** the frontend invokes `root_id` and the session has no root block version record yet
- **THEN** the command fails with an error derived from `CoreError` without friendly rewriting

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

The desktop window's main content surface MUST be the primary Block View surface per **`desktop-shell-ui`**, rendering the current selected block. The default primary renderer is the Tree View per **`tree-view`**. Arrow-key navigation within the Tree View is the Tree View's selection policy and is permitted; it is not a global keyboard shortcut menu. The desktop window MUST NOT include separate navigation-tree or sidebar chrome, file-open dialogs, auto-save on mutation or close, keyboard shortcut menus, themes, or welcome/marketing copy. The action bar overlay per **`action-bar`** is an allowed surface in addition to the Block View and Properties View per **`desktop-shell-ui`**. Any other surface MUST be introduced only by an explicit future requirement.

#### Scenario: Main surface is the Block View

- **WHEN** the application renders its main content surface
- **THEN** it shows the primary surface rendering the current selected block per **`desktop-shell-ui`**
- **AND** the Tree View appears as primary content when the default primary renderer is active, not as separate sidebar chrome

#### Scenario: No invented chrome

- **WHEN** the application renders
- **THEN** it does not add file-open dialogs, auto-save on mutation or close, keyboard shortcut menus, themes, or welcome/marketing copy

#### Scenario: Action bar overlay is permitted

- **WHEN** the application renders its content surface
- **THEN** the action bar overlay per **`action-bar`** MAY be shown over the content
- **AND** it is not treated as invented chrome

### Requirement: Block-view IPC commands

The desktop shell MUST expose Tauri invoke commands sufficient for the desktop UI framework, each a thin wrapper over `core::Session`: read a block, read a block's parent, read a block's children, read a block's children in **`child-ordering`** order, create a child block, set a block property, delete a block, move a block, and save. Read commands MUST mirror session read APIs and the `parent:` and `children:` queries per **`query-language`**. The ordered-children command MUST return children sorted by **`child-ordering`**. Create, delete, and move commands MUST NOT save. Errors MUST be derived from `CoreError` without friendly rewriting per **`agent-anti-default`**, Requirement: **Error presentation without friendly rewriting**.

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
- **THEN** the backend returns that block's children sorted by the **`child-ordering`** `order` property
- **AND** each child carries its id, properties, and whether it has children

#### Scenario: Read block command

- **WHEN** the frontend invokes the read-block command with a block id present in the session
- **THEN** the backend returns that block's id and properties

#### Scenario: Create child command

- **WHEN** the frontend invokes the create-child command with a parent block id present in the session
- **THEN** the backend calls `Session::create_block` with that parent and appends block and parent edge version records in memory
- **AND** no save occurs
- **AND** an invalid request fails with an error derived from `CoreError` without friendly rewriting

#### Scenario: Create child at a sibling-relative position

- **WHEN** the frontend invokes the create-child command with a parent and a position immediately after a given sibling
- **THEN** the backend calls `Session::create_block` with that parent and the corresponding `PositionHint`
- **AND** the new child's **`child-ordering`** `order` places it immediately after the given sibling
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

### Requirement: Desktop open policy (interim)

Until a storage-engine capability with autosave is specified, when `AppState` is opened at a path where no knowledge base storage directory exists, the desktop adapter MUST automatically save the session so the root block is materialized before any frontend calls are served. After `open_at` returns successfully, `root_id()` MUST succeed. This policy is adapter-specific and MUST NOT be assumed by **`session`**, **`cli`**, or future REPL mode.

The deterministic storage path under application data MUST be the directory `{app_data_dir}/pu-erh/kb/` (not a `kb.json` file).

#### Scenario: First launch creates root block

- **WHEN** `AppState::open_at` is called with a storage directory path that does not exist on disk
- **THEN** `open_at` saves the session automatically
- **AND** `root_id()` succeeds immediately after `open_at` returns

#### Scenario: Existing knowledge base is not re-saved on open

- **WHEN** `AppState::open_at` is called with a path to an existing knowledge base storage directory
- **THEN** no additional save is performed during `open_at`
- **AND** the storage directory on disk is not modified by the open operation alone

