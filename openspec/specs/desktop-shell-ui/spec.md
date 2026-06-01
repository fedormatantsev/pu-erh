# desktop-shell-ui Specification

## Purpose

Desktop app navigation framework: current selected block, Block/Properties mode exclusivity, unified renderer registry keyed by `display` (primary and inline dispatch, primary-only fallback, default inline renderer), and View Router.
## Requirements
### Requirement: Current selected block

There MUST always be exactly one current selected block, and it MUST resolve to a valid (non-tombstoned) block in the active view. The current selected block dictates what the UI renders. On application open with no prior selection, the current selected block MUST resolve to the root block.

#### Scenario: Selection always present on open

- **WHEN** the application opens a session with a persisted root block and no prior selection
- **THEN** the current selected block is the root block

#### Scenario: Exactly one current block

- **WHEN** the UI is rendering
- **THEN** exactly one block is the current selected block
- **AND** the rendered surface is determined by that block

#### Scenario: Selection must reference a valid block

- **WHEN** the current selected block id no longer resolves to an active (non-tombstoned) block
- **THEN** the UI MUST NOT render a stale or missing block as current
- **AND** the current selected block MUST be re-resolved to a valid block (the root block when no other selection is valid)

### Requirement: Primary surface renders current block

The primary UI surface MUST render the current selected block. The component MUST be chosen by that block's `display` property through the renderer registry per Requirement: **Renderer registry**.

#### Scenario: Primary surface renders the current block

- **WHEN** the primary surface (Block View mode) is shown
- **THEN** it renders the current selected block using the component registered for that block's `display` value

#### Scenario: Renderer chosen by display

- **WHEN** the current selected block's `display` property names a registered renderer
- **THEN** the primary surface renders that block with the registered component

### Requirement: Mode exclusivity between Block View and Properties View

The user MUST be able to switch between the Block View and the Properties View. Exactly one MUST be shown at a time. The active mode is ephemeral UI state and MUST NOT be persisted unless a future change specifies persistence.

#### Scenario: Only one surface visible

- **WHEN** the user switches to the Properties View
- **THEN** the Properties View is shown
- **AND** the Block View is not shown at the same time

#### Scenario: Switch back to Block View

- **WHEN** the user switches from the Properties View to the Block View
- **THEN** the Block View is shown
- **AND** the Properties View is not shown at the same time

### Requirement: Renderer-specific selection policy

Changing the current selected block MUST be delegated to the active primary renderer implementation. This capability MUST NOT mandate a global selection mechanism, and an inline block MUST NOT be assumed to become the current selected block on activation unless the active renderer's policy specifies it.

#### Scenario: No global navigation rule

- **WHEN** the user interacts with an inline block in a primary renderer
- **THEN** whether that interaction changes the current selected block is determined by that renderer's selection policy per **`tree-view`** when the Tree View is active
- **AND** no global "activate inline block selects it" behavior is assumed

### Requirement: Renderer registry

The application MUST maintain a renderer registry mapping each registered `display` string to a view component and a mode set (`primary`, `inline`, or both). Registration MUST be the single source of truth for which components implement which `display` values. The registry MUST be consulted for both primary and inline rendering.

#### Scenario: Primary dispatch uses registry

- **WHEN** the current selected block's `display` names a registered value with primary in its mode set
- **THEN** the primary surface renders that block with the registered component

#### Scenario: Inline dispatch uses registry when inline is supported

- **WHEN** a block rendered inline has a `display` value registered with inline in its mode set
- **THEN** the registered component renders in inline rendering context per Requirement: **Primary vs inline rendering context**

#### Scenario: Primary-only registry entry falls back on inline dispatch

- **WHEN** a block rendered inline has a `display` value registered as primary-only
- **THEN** dispatch MUST NOT use that component
- **AND** MUST use the default inline renderer per Requirement: **Default renderer**

#### Scenario: Display dropdown populated from registry

- **WHEN** the Properties View renders the `display` dropdown per **`properties-view`**
- **THEN** options are the registered `display` string keys from this registry

### Requirement: Renderer dispatch fallback

When a block's `display` is unset, or set to a value with no registered component, dispatch MUST select the default renderer per Requirement: **Default renderer**. When `display` is unrecognized, the system MUST surface the unrecognized value and THEN select the default renderer. Unset and unrecognized handling MUST be identical for primary and inline dispatch.

#### Scenario: Unset display selects default on primary

- **WHEN** the current selected block has no `display` property
- **THEN** the default primary renderer is selected

#### Scenario: Unrecognized display surfaced then defaulted on primary

- **WHEN** the current selected block's `display` is not registered
- **THEN** the unrecognized `display` value is surfaced
- **AND** the default primary renderer is selected

#### Scenario: Unset display selects default inline renderer

- **WHEN** an inline block has no `display` property
- **THEN** the default inline renderer is selected

### Requirement: Default renderer

The default primary renderer MUST be the Tree View per **`tree-view`**.

The default inline renderer MUST show the block's `title` per **`property-registry`**, Requirement: **title property**, when present. When `title` is absent or non-string, the default inline renderer MUST show the block id as a neutral technical label. The default inline renderer MUST NOT show marketing or descriptive placeholder copy.

#### Scenario: Title shown when present in default inline renderer

- **WHEN** a block with a string `title` property is rendered by the default inline renderer
- **THEN** the rendered label is the `title` value

#### Scenario: Block id when title absent in default inline renderer

- **WHEN** a block with no `title` property is rendered by the default inline renderer
- **THEN** the rendered label is the block id

### Requirement: Primary vs inline rendering context

Primary dispatch MUST render the current selected block as the main application surface. Inline dispatch MUST render a block embedded within a host view (for example Tree View columns or a query-result collection).

Both paths MUST resolve the component from the same renderer registry keyed by that block's `display` property. Registered components MUST accept a rendering context (`primary` | `inline`) when inline is supported. Primary-only entries MUST NOT receive inline context — inline dispatch MUST fall back before invoking them.

#### Scenario: Primary rendering of current block

- **WHEN** a block is the current selected block
- **THEN** it is rendered as primary using its `display` registry entry in primary rendering context

#### Scenario: Inline rendering as a child

- **WHEN** a block is shown as a child of the current selected block in a host view that displays hierarchy
- **THEN** it is rendered inline via the registry lookup for that block's own `display` property

#### Scenario: Inline rendering in a collection

- **WHEN** a block is shown as a member of a query result or other collection-of-blocks context
- **THEN** it is rendered inline via the registry lookup for that block's own `display` property

### Requirement: View Router

The application MUST dispatch the active view through a named View Router function with the signature `(blockId: string, mode: ViewMode) → ViewDescriptor`. The View Router MUST be the single authoritative place where `ViewMode` values are mapped to concrete view components and their associated action descriptor factories.

A `ViewDescriptor` MUST carry:
- `View`: the React component that renders the active view for the given block.
- `actions`: a factory function `(ctx: ViewActionContext) → ActionBarAction[]`.

`ViewActionContext` MUST carry at minimum `setViewMode`, `createChild`, and `canCreateChild`.

#### Scenario: ViewMode maps to Tree View component and actions

- **WHEN** the View Router is called with `mode === "block"`
- **THEN** it returns a `ViewDescriptor` whose `View` is the Tree View component per **`tree-view`**
- **AND** whose `actions` factory returns toggle-to-properties and create-child descriptors

#### Scenario: ViewMode maps to Properties View component and actions

- **WHEN** the View Router is called with `mode === "properties"`
- **THEN** it returns a `ViewDescriptor` whose `View` is the Properties View component per **`properties-view`**
- **AND** whose `actions` factory returns only the toggle-to-block-view descriptor

#### Scenario: View Router is the single dispatch point

- **WHEN** the application host renders the active view
- **THEN** it calls the View Router to obtain both the view component and the action list
- **AND** does not contain a separate inline conditional that branches on view mode

### Requirement: Shell UI state and presentational placement

Presentational building blocks (column layout, inline block label, properties panel layout) MUST live in the design-system package and MUST remain session-agnostic per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**. All shell UI state — the current selected block and the active view mode — MUST be held in the application shell (`apps/desktop`), which wires session reads and mutations to presentational components.

#### Scenario: Presentational components stay session-agnostic

- **WHEN** a shell UI presentational component is added to the design-system package
- **THEN** it receives data and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Shell owns selection and view mode state

- **WHEN** the current selected block or the active view mode changes
- **THEN** that state is held and updated in the application shell
- **AND** the shell passes the resulting data to the presentational components

