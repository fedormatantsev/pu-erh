## ADDED Requirements

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

The desktop shell MUST NOT invoke `Session::save` on a timer, on mutation, or on window close unless a future change task explicitly requires it.

#### Scenario: Window close without save task

- **WHEN** the user closes the Tauri window
- **THEN** the process exits without an automatic save-on-close step defined in this change

### Requirement: Anti-default UI shell

The desktop window MUST NOT include navigation trees, sidebars, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy as part of this change.

#### Scenario: Bare shell content

- **WHEN** the application renders its initial view
- **THEN** the UI shows only neutral scaffold content (e.g. application name and IPC wiring proof)
- **AND** does not include hierarchical block navigation or file-open dialogs
