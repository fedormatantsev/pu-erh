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

The desktop window's main content surface MUST be the Block View defined by the `block-view` capability, rendering the current selected block. The Default Block View MAY render the current block's children as a recursive tree with expand/collapse controls within the Block View content. The desktop window MUST NOT include separate navigation-tree or sidebar chrome, file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy. Any surface beyond the Block View and its associated Properties View (per `block-view`) MUST be introduced only by an explicit future requirement.

#### Scenario: Main surface is the Block View

- **WHEN** the application renders its main content surface
- **THEN** it shows the Block View rendering the current selected block as defined by `block-view`
- **AND** the Default Block View's children tree appears as Block View content, not as a separate sidebar or window chrome

#### Scenario: No invented chrome

- **WHEN** the application renders
- **THEN** it does not add file-open dialogs, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy

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

