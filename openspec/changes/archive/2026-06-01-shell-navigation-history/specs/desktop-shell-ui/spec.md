## ADDED Requirements

### Requirement: Shell records navigation history

The application shell MUST record navigation history per **`navigation-history`** each time `selectBlock` is invoked by a user action. The shell MUST expose `navigateBack`, `navigateForward`, `canGoBack`, and `canGoForward` as part of its state. The initial root-block resolution on application open MUST NOT create a back stack entry. History state MUST be held in the application shell alongside the current selected block.

#### Scenario: selectBlock records history

- **WHEN** `selectBlock` is called with a block id different from the current selected block
- **THEN** the previous current block id is pushed onto the back stack
- **AND** the forward stack is cleared
- **AND** the new block becomes the current selected block

#### Scenario: Root resolution does not push history

- **WHEN** the application opens and resolves the root block as the initial current selected block
- **THEN** the back stack remains empty

#### Scenario: Shell exposes navigation capabilities

- **WHEN** the shell state is consumed
- **THEN** `navigateBack`, `navigateForward`, `canGoBack`, and `canGoForward` are available
- **AND** they reflect the current state of the navigation history stacks

### Requirement: Shell UI state and presentational placement

Presentational building blocks (column layout, inline block label, properties panel layout) MUST live in the design-system package and MUST remain session-agnostic per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**. All shell UI state — the current selected block, the active view mode, and the navigation history stacks — MUST be held in the application shell (`apps/desktop`), which wires session reads and mutations to presentational components.

#### Scenario: Presentational components stay session-agnostic

- **WHEN** a shell UI presentational component is added to the design-system package
- **THEN** it receives data and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Shell owns selection, view mode, and navigation history state

- **WHEN** the current selected block, the active view mode, or the navigation history changes
- **THEN** that state is held and updated in the application shell
- **AND** the shell passes the resulting data to the presentational components
