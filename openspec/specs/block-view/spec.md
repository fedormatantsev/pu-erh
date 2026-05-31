# block-view Specification

## Purpose
TBD - created by archiving change block-view-principles. Update Purpose after archive.
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

### Requirement: Block View is the primary surface

The Block View MUST be the primary UI surface, and it MUST render the current selected block. The Block View decides how to represent the block's contents through a renderer (for example a Document Editor, Chart, or Calendar). The renderer for the current selected block MUST be chosen by that block's `display` property.

#### Scenario: Block View renders the current block

- **WHEN** the Block View is shown
- **THEN** it renders the current selected block using the renderer named by that block's `display` property

#### Scenario: Renderer chosen by display

- **WHEN** the current selected block's `display` property names a renderer the Block View supports
- **THEN** the Block View represents the block using that renderer

### Requirement: Display property

`display` MUST be a reserved property key on a block. Its value MUST be a `PropertyValue::String` naming the renderer used to represent the block as the current selected block. `display` MUST be stored in the existing block properties map and MUST be changed only through the Properties View. Writes to `display` MUST go through the `set_property` block-property mutation (see the `mutations` capability) exposed by `core::Session`; the UI layer MUST NOT introduce Block-View-specific mutation or validation logic.

#### Scenario: Display is a reserved string property

- **WHEN** a block has a renderer configured
- **THEN** the block's properties map contains the key `display` with a `PropertyValue::String` value naming the renderer

#### Scenario: Display changed via the set_property mutation

- **WHEN** the Properties View changes the current selected block's `display`
- **THEN** the change is applied through the `core::Session` `set_property` mutation
- **AND** no Block-View-specific mutation or validation logic is introduced in the UI layer

### Requirement: Properties View

The Properties View MUST contain the settings of the current Block View, including the current selected block's `display` property. The Properties View MUST be the surface through which `display` is changed. The `display` property MUST be rendered in a dedicated, fixed-position layout slot (before the generic properties list) as a constrained one-of selector (dropdown) populated with the registered block view names available in the app; it MUST NOT appear as an item in the generic properties list and MUST NOT be presented as a free-text input. No property title label MUST be shown for `display` — only the selector control is rendered in its slot. When the block has no `display` property or its stored value is not a recognized view name, the selector MUST implicitly resolve to the default view and MUST overwrite the stored value with the resolved default on the next save.

#### Scenario: Properties View exposes display as a dropdown

- **WHEN** the Properties View is shown for the current selected block
- **THEN** it presents the block's `display` property as a dropdown populated with registered view names
- **AND** no free-text input is shown for `display`
- **AND** no property title label is shown for `display`

#### Scenario: Absent display resolves to default in selector

- **WHEN** the Properties View loads a block that has no `display` property
- **THEN** the selector shows the default view as the selected value
- **AND** no write to storage occurs at load time

#### Scenario: Unrecognized display resolves to default in selector

- **WHEN** the Properties View loads a block whose `display` value is not a registered view name
- **THEN** the selector shows the default view as the selected value
- **AND** no error or warning is surfaced to the user

#### Scenario: Absent or unrecognized display overwritten on save

- **WHEN** a block's `display` was absent or unrecognized at load time
- **AND** the user saves
- **THEN** the resolved default value is written to the block's `display` property before persisting
- **AND** the stored value after save is the default view name

### Requirement: Mode exclusivity between Block View and Properties View

The user MUST be able to switch between the Block View and the Properties View. Exactly one of them MUST be shown at a time; they MUST NOT be shown simultaneously. The active mode is ephemeral UI state and MUST NOT be persisted unless a future change specifies persistence.

#### Scenario: Only one surface visible

- **WHEN** the user switches to the Properties View
- **THEN** the Properties View is shown
- **AND** the Block View is not shown at the same time

#### Scenario: Switch back to Block View

- **WHEN** the user switches from the Properties View to the Block View
- **THEN** the Block View is shown
- **AND** the Properties View is not shown at the same time

### Requirement: Primary and inline block rendering

A block MUST be rendered in one of two modes. A block MUST be rendered as **primary** when it is the current selected block, using the Block View renderer named by its `display`. A block MUST be rendered as **inline** when it is shown as a child of the current selected block (when the Block View displays hierarchy) or as a member of a collection such as a query result. The inline representation MUST be chosen by the containing Block View and is not required to match the block's own `display`.

#### Scenario: Primary rendering of current block

- **WHEN** a block is the current selected block
- **THEN** it is rendered as primary within the Block View using its `display` renderer

#### Scenario: Inline rendering as a child

- **WHEN** a block is shown as a child of the current selected block in a Block View that displays hierarchy
- **THEN** it is rendered inline using a representation chosen by the containing Block View

#### Scenario: Inline rendering in a collection

- **WHEN** a block is shown as a member of a query result or other collection-of-blocks context
- **THEN** it is rendered inline using a representation chosen by the containing Block View

### Requirement: Block-View-specific selection policy

Changing the current selected block MUST be delegated to the active Block View implementation; each Block View defines its own selection and navigation policy. This capability MUST NOT mandate a global selection mechanism, and an inline block MUST NOT be assumed to become the current selected block on activation unless the active Block View's policy specifies it.

#### Scenario: No global navigation rule

- **WHEN** the user interacts with an inline block in a Block View
- **THEN** whether that interaction changes the current selected block is determined by the active Block View's selection policy
- **AND** no global "activate inline block selects it" behavior is assumed

### Requirement: Block View surfaces are thin adapters

The Block View, the Properties View, and renderer dispatch MUST live in the application adapter layer (for example `apps/desktop`), not in the presentational design system. They MUST coordinate `core::Session` only and MUST NOT duplicate graph, CRDT, trie, or mutation-validation logic.

#### Scenario: Dispatch coordinates the session only

- **WHEN** the Block View resolves the current selected block and its `display` renderer
- **THEN** it reads block state through `core::Session`
- **AND** it does not reimplement domain logic in the UI layer

### Requirement: Block View router

The Block View MUST dispatch the current selected block to a primary renderer by the block's `display` property through a Block View router. The router MUST provide a default renderer. When `display` is unset, the router MUST select the default renderer. When `display` is set to a value no registered renderer recognizes, the router MUST surface the unrecognized value rather than silently substituting a renderer, and MUST then select the default renderer.

#### Scenario: Display selects a registered renderer

- **WHEN** the current selected block's `display` names a renderer registered with the Block View router
- **THEN** the Block View renders the block with that renderer

#### Scenario: Unset display selects the default renderer

- **WHEN** the current selected block has no `display` property
- **THEN** the Block View router selects the default renderer

#### Scenario: Unrecognized display value surfaced then defaulted

- **WHEN** the current selected block's `display` value is not registered with the Block View router
- **THEN** the Block View surfaces the unrecognized `display` value
- **AND** the router selects the default renderer

### Requirement: Inline View router

When a block is rendered inline (as a child or collection member inside a Block View), the inline representation MUST be dispatched by the block's `display` property through an Inline View router that provides a default inline renderer. The same unset and unrecognized-value handling as the Block View router MUST apply: unset selects the default; an unrecognized value is surfaced and then the default is selected.

#### Scenario: Inline default renderer

- **WHEN** a block is rendered inline and its `display` names no registered inline renderer
- **THEN** the Inline View router selects the default inline renderer

### Requirement: Default Block View

The default Block View renderer MUST display the current selected block within a three-column TreeView that communicates the block's position in the hierarchy. The columns MUST be, left to right: (1) the **single parent** of the current selected block; (2) the current selected block together with its sibling blocks (those ordered before and after it), with the current selected block visibly distinguished; (3) the **children** of the current selected block. Every block shown in any column MUST be rendered inline via the Inline View router.

The left (parent) column MUST be empty when the current selected block is the root block (it has no parent). The right (children) column MUST be empty when the current selected block has no children.

The TreeView's selection policy MUST support two ways to change the current selected block:

- **Activation**: activating (for example clicking) a block's inline preview in any column MUST make that block the current selected block.
- **Arrow-key navigation**: `←` MUST select the parent of the current selected block; `→` MUST select the first child of the current selected block; `↑` MUST select the previous sibling and `↓` MUST select the next sibling within the center column. Each of these key actions MUST be a no-op when the target does not exist: `←` at the root, `→` when there are no children, and `↑`/`↓` past the first/last sibling.

Whenever the current selected block changes (by activation or arrow key), the Block View MUST re-render so that the columns re-center on the newly selected block. The TreeView MUST NOT introduce any block-property mutation; selection is ephemeral UI state and changes only the current selected block.

#### Scenario: Three columns reflect the hierarchy

- **WHEN** the default Block View renders the current selected block
- **THEN** the left column shows that block's parent, the center column shows that block and its siblings with the current block distinguished, and the right column shows that block's children
- **AND** each shown block is rendered inline via the Inline View router

#### Scenario: Root hides the parent column content

- **WHEN** the current selected block is the root block
- **THEN** the left (parent) column shows no block

#### Scenario: No children hides the children column content

- **WHEN** the current selected block has no children
- **THEN** the right (children) column shows no block

#### Scenario: Activating a block selects it

- **WHEN** the user activates a block's inline preview in any column
- **THEN** that block becomes the current selected block
- **AND** the Block View re-centers the columns on the newly selected block

#### Scenario: Arrow keys move the selection and re-center

- **WHEN** the user presses `←`, `→`, `↑`, or `↓`
- **THEN** the current selected block becomes the parent, the first child, the previous sibling, or the next sibling respectively
- **AND** the Block View re-centers the columns on the newly selected block

#### Scenario: Arrow keys are no-ops at hierarchy edges

- **WHEN** the user presses `←` while the current selected block is the root, `→` while it has no children, or `↑`/`↓` while it is the first/last sibling
- **THEN** the current selected block does not change

### Requirement: Default Inline view and title property

The default inline renderer MUST show the block's `title`, where `title` is a reserved property key whose value, when present, MUST be a `PropertyValue::String`. When the block has no `title` property (or its value is not a string), the default inline renderer MUST show the block id as a neutral placeholder. The default inline renderer MUST NOT invent descriptive or marketing placeholder copy.

#### Scenario: Title shown when present

- **WHEN** a block with a `title` string property is rendered by the default inline renderer
- **THEN** the rendered label is the `title` value

#### Scenario: Id placeholder when title absent

- **WHEN** a block with no `title` property is rendered by the default inline renderer
- **THEN** the rendered label is the block id

### Requirement: Block View component and state placement

Presentational Block View building blocks (column layout, inline block label, view-mode toggle, properties panel layout) MUST live in the design-system package and MUST remain session-agnostic: they MUST NOT call IPC/Tauri APIs and MUST NOT assume a current selected block. All Block View state — the current selected block and the active view mode — MUST be held in the application shell, which wires session reads and mutations to the presentational components. The TreeView's columns (parent, siblings, children) MUST be derived from the current selected block; the shell MUST NOT hold separate per-node expand/collapse state.

#### Scenario: Presentational components stay session-agnostic

- **WHEN** a Block View component is added to the design-system package
- **THEN** it receives data and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Shell owns Block View state

- **WHEN** the current selected block or the active view mode changes
- **THEN** that state is held and updated in the application shell
- **AND** the shell passes the resulting data to the presentational components

#### Scenario: Columns derive from the current selected block

- **WHEN** the TreeView renders
- **THEN** the parent, sibling, and children columns are derived from the current selected block
- **AND** the shell holds no separate per-node expand/collapse state

