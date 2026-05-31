# action-bar Specification

## Purpose

Defines the action bar overlay: a compact floating panel in the upper-left corner exposing shell-level actions (view-mode toggle and create-child), with presentational parts session-agnostic in the design system.
## Requirements
### Requirement: Action bar overlay surface

The application MUST present an **action bar**: a compact floating panel pinned to the upper-left corner of the window that overlays the active content surface (the Block View or the Properties View). The action bar MUST be subordinate chrome per the `ui-direction` capability — low contrast, compact, with no decorative border or background that competes with content — and MUST NOT displace or resize the content beneath it. The action bar MUST expose exactly the actions defined by this capability and MUST NOT introduce navigation trees, sidebars, menus, or other unspecified surfaces.

#### Scenario: Action bar overlays the content in the upper-left

- **WHEN** the application renders its content surface
- **THEN** the action bar is shown as a floating panel in the upper-left corner over the content
- **AND** the content surface fills the available width beneath it without being displaced

#### Scenario: Action bar exposes only the defined actions

- **WHEN** the action bar is rendered
- **THEN** it shows only the view-mode toggle action and the create-child action
- **AND** it adds no navigation tree, sidebar, menu, or other surface

### Requirement: Action bar is presentational with shell-owned state

The presentational action bar component MUST live in the design-system package and MUST be session-agnostic: it MUST receive its state and action handlers as props/callbacks, and it MUST NOT call IPC/Tauri APIs or read shell state directly. All action bar state — the active view mode and whether a current selected block exists — MUST be held in the application shell, which wires the handlers to `core::Session`.

#### Scenario: Presentational component stays session-agnostic

- **WHEN** the action bar component is added to the design-system package
- **THEN** it receives the active mode, an enabled/disabled flag for create, and action callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Shell owns action bar state and wiring

- **WHEN** the view mode changes or a current selected block becomes available
- **THEN** that state is held in the application shell
- **AND** the shell passes the resulting state and handlers to the presentational action bar

### Requirement: Toggle view mode action

The action bar MUST provide a single action that toggles the active view mode between the Block View and the Properties View, consistent with the mode-exclusivity requirement of the `block-view` capability. The action MUST reflect the current mode. This action MUST be the only view-mode toggle surface; no separate inline toggle MUST be shown alongside it.

#### Scenario: Toggle switches the active mode

- **WHEN** the user activates the toggle action while the Block View is shown
- **THEN** the active mode changes to the Properties View
- **AND** exactly one of the two views is shown, per `block-view` mode exclusivity

#### Scenario: Toggle reflects the current mode

- **WHEN** the action bar is rendered
- **THEN** the toggle action reflects which view is currently active

#### Scenario: Single toggle surface

- **WHEN** the action bar is shown
- **THEN** no separate inline view-mode toggle is rendered outside the action bar

### Requirement: Create child block action

The action bar MUST provide an action that creates a new block as a child of the current selected block by calling the `create_block` mutation exposed by `core::Session` with the current selected block as the parent (see the `mutations` capability). The action MUST be disabled when there is no current selected block. Creating a child MUST NOT change the current selected block and MUST NOT trigger a save (consistent with the desktop shell's no-save policy); the created block is held in memory until the user invokes the explicit Save control. After a successful create, the Block View MUST re-read the current selected block's children so the new child appears in the children column. The action bar MUST NOT introduce any block creation, validation, or selection logic of its own beyond invoking the shell-provided handler. Any error from the mutation MUST be surfaced as the `CoreError`-derived value, without friendly rewriting.

#### Scenario: Create appends a child of the current block

- **WHEN** the user activates the create-child action while a block is the current selected block
- **THEN** the shell calls `core::Session::create_block` with that block as the parent
- **AND** a new block and parent edge version record are appended in memory per the `mutations` capability

#### Scenario: Create is disabled without a current block

- **WHEN** there is no current selected block (for example the root could not be resolved)
- **THEN** the create-child action is disabled

#### Scenario: Create does not change selection

- **WHEN** a child is successfully created
- **THEN** the current selected block does not change

#### Scenario: Create does not save

- **WHEN** a child is successfully created
- **THEN** no save occurs and the new block is held in memory
- **AND** it is persisted only when the user invokes the explicit Save control

#### Scenario: New child appears after create

- **WHEN** a child is successfully created under the current selected block
- **THEN** the Block View re-reads the current selected block's children
- **AND** the new child appears in the children column

#### Scenario: Create error surfaced as returned

- **WHEN** the create-child mutation returns an error
- **THEN** the error is surfaced as the `CoreError`-derived value without friendly rewriting
