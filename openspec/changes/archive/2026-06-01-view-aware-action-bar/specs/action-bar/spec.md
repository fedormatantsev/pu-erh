## MODIFIED Requirements

### Requirement: Action bar exposes only the defined actions

The action bar MUST render exactly the actions provided by the currently active view. It MUST NOT have a fixed, hard-coded action set of its own. The presentational `ActionBar` component MUST accept a generic ordered list of action descriptors and MUST render one button per descriptor, in order. It MUST NOT infer, add, or suppress actions based on the active view mode — that is the responsibility of the view that declared the actions.

#### Scenario: Action bar renders the active view's actions

- **WHEN** the active view provides a list of action descriptors
- **THEN** the action bar renders exactly one button per descriptor, in the order provided
- **AND** it does not add or suppress any actions on its own

#### Scenario: Tree View supplies toggle-to-properties and create-child

- **WHEN** the Tree View (Block View mode) is the active view
- **THEN** the action bar shows a toggle-to-properties action and a create-child action
- **AND** it shows no other actions

#### Scenario: Properties View supplies toggle-to-block-view only

- **WHEN** the Properties View is the active view
- **THEN** the action bar shows a toggle-to-block-view action only
- **AND** the create-child action is not shown

### Requirement: Action bar is presentational with shell-owned state

The presentational action bar component MUST live in the design-system package and MUST be session-agnostic: it MUST receive its actions as a list of descriptors (each carrying a label, a press handler, an optional disabled flag, and an optional pressed flag for toggle semantics), and it MUST NOT call IPC/Tauri APIs or read shell state directly. All action bar state — the active view mode and whether a current selected block exists — MUST be held in the application shell. The shell MUST gather the active view's action descriptors and pass them to the presentational action bar.

#### Scenario: Presentational component stays session-agnostic

- **WHEN** the action bar component is rendered
- **THEN** it receives the action list as props
- **AND** it does not call IPC/Tauri APIs or inspect view-mode or block state directly

#### Scenario: Shell gathers actions from the active view and passes them to the action bar

- **WHEN** the view mode changes or the current selected block changes
- **THEN** the shell re-derives the action list from the now-active view's descriptor factory
- **AND** passes the updated list to the presentational action bar

### Requirement: Toggle view mode action

The active view MUST declare a toggle action that switches the active view mode between the Block View and the Properties View. The toggle action MUST communicate the current pressed/active state through its descriptor (a `pressed` flag) so the action bar can render it with correct ARIA semantics. This action MUST be the only view-mode toggle surface; no separate inline toggle MUST be shown alongside it.

#### Scenario: Toggle switches the active mode

- **WHEN** the user activates the toggle action while the Block View is shown
- **THEN** the active mode changes to the Properties View
- **AND** exactly one of the two views is shown

#### Scenario: Toggle reflects the current mode via pressed state

- **WHEN** the action bar renders the toggle action
- **THEN** the toggle action descriptor's pressed flag reflects whether the view being toggled away from is currently active

#### Scenario: Single toggle surface

- **WHEN** the action bar is shown
- **THEN** no separate inline view-mode toggle is rendered outside the action bar

### Requirement: Create child block action

The Tree View MUST declare a create-child action in its action descriptor list. The action MUST be disabled when there is no current selected block. All other behavior (no change to selection, no save, re-reading children, surfacing errors) is unchanged from the existing requirement. The Properties View MUST NOT declare a create-child action.

#### Scenario: Create child declared by Tree View

- **WHEN** the Tree View is the active view
- **THEN** the action descriptor list includes a create-child action

#### Scenario: Create child absent from Properties View actions

- **WHEN** the Properties View is the active view
- **THEN** the action descriptor list does not include a create-child action

#### Scenario: Create is disabled without a current block

- **WHEN** there is no current selected block
- **THEN** the create-child action descriptor has isDisabled set to true

#### Scenario: Create appends a child of the current block

- **WHEN** the user activates the create-child action while a block is the current selected block
- **THEN** the shell calls `core::Session::create_block` with that block as the parent
- **AND** a new block and parent edge version record are appended in memory per the `mutations` capability

#### Scenario: Create does not change selection

- **WHEN** a child is successfully created
- **THEN** the current selected block does not change

#### Scenario: Create does not save

- **WHEN** a child is successfully created
- **THEN** no save occurs and the new block is held in memory

#### Scenario: New child appears after create

- **WHEN** a child is successfully created under the current selected block
- **THEN** the Block View re-reads the current selected block's children
- **AND** the new child appears in the children column

#### Scenario: Create error surfaced as returned

- **WHEN** the create-child mutation returns an error
- **THEN** the error is surfaced as the `CoreError`-derived value without friendly rewriting
