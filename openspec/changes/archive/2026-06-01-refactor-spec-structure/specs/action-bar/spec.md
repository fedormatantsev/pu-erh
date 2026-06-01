## MODIFIED Requirements

### Requirement: Action bar overlay surface

The application MUST present an **action bar**: a compact floating panel pinned to the upper-left corner that overlays the active content surface (Block View or Properties View). The action bar MUST be subordinate chrome per **`ui-direction`**. It MUST NOT displace or resize content beneath it. The action bar MUST render exactly the actions provided by the active view's descriptor factory per **`desktop-shell-ui`**, Requirement: **View Router** — it MUST NOT introduce navigation trees, sidebars, menus, or other unspecified surfaces.

#### Scenario: Action bar overlays the content in the upper-left

- **WHEN** the application renders its content surface
- **THEN** the action bar is shown as a floating panel in the upper-left corner over the content
- **AND** the content surface fills the available width beneath it without being displaced

#### Scenario: Action bar renders only the active view's actions

- **WHEN** the action bar is rendered
- **THEN** it shows exactly the actions from the active view's descriptor factory
- **AND** it adds no navigation tree, sidebar, menu, or other surface

### Requirement: Action bar is presentational with shell-owned state

The presentational `ActionBar` component MUST live in the design-system package and MUST be session-agnostic per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**. It MUST accept an ordered list of action descriptors (label, press handler, optional disabled flag, optional pressed flag). The shell MUST gather descriptors from the active view via **`desktop-shell-ui`**, Requirement: **View Router**, and pass them as props.

#### Scenario: Presentational component stays session-agnostic

- **WHEN** the action bar component is rendered
- **THEN** it receives the action list as props
- **AND** it does not call IPC/Tauri APIs or inspect view-mode or block state directly

#### Scenario: Shell gathers actions from the active view and passes them to the action bar

- **WHEN** the view mode changes or the current selected block changes
- **THEN** the shell re-derives the action list from the now-active view's descriptor factory
- **AND** passes the updated list to the presentational action bar

### Requirement: Toggle view mode action

The active view MUST declare a toggle action switching between Block View and Properties View per **`desktop-shell-ui`**, Requirement: **Mode exclusivity between Block View and Properties View**. The toggle MUST communicate pressed state for ARIA. This MUST be the only view-mode toggle surface.

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

When the Tree View is active per **`tree-view`**, Requirement: **Tree View action descriptors**, the action list MUST include create-child. The Properties View MUST NOT include create-child per **`properties-view`**, Requirement: **Properties View action descriptors**. Create MUST call `Session::create_block` per **`mutations`**. Create MUST NOT change selection or save. Errors MUST surface per **`agent-anti-default`**, Requirement: **Error presentation without friendly rewriting**.

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
- **AND** a new block and parent edge version record are appended in memory per **`mutations`**

#### Scenario: Create does not change selection

- **WHEN** a child is successfully created
- **THEN** the current selected block does not change

#### Scenario: Create does not save

- **WHEN** a child is successfully created
- **THEN** no save occurs and the new block is held in memory

#### Scenario: New child appears after create

- **WHEN** a child is successfully created under the current selected block
- **THEN** the Tree View re-reads the current selected block's children
- **AND** the new child appears in the children column

#### Scenario: Create error surfaced as returned

- **WHEN** the create-child mutation returns an error
- **THEN** the error is surfaced as the `CoreError`-derived value without friendly rewriting

## REMOVED Requirements

### Requirement: Action bar exposes only the defined actions

**Reason**: Duplicated by **Action bar overlay surface** and **Action bar is presentational with shell-owned state**.

**Migration**: Use the merged requirements above; active views declare actions via **`tree-view`** and **`properties-view`**.
