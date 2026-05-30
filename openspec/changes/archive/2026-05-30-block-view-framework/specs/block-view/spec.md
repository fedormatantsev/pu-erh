## ADDED Requirements

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

The default Block View renderer MUST display the current selected block's children as a tree. Each child MUST be rendered inline via the Inline View router. The tree MUST support recursively expanding a node to reveal that node's own children, and collapsing it. Activating (for example clicking) a node in the tree MUST make that node the current selected block; this is the default Block View's selection policy.

#### Scenario: Default view shows children as a tree

- **WHEN** the default Block View renders the current selected block
- **THEN** it displays the block's children, each rendered inline via the Inline View router

#### Scenario: Expand and collapse a node

- **WHEN** the user expands a tree node
- **THEN** that node's children are revealed in the tree
- **AND** collapsing the node hides them again

#### Scenario: Activating a node selects it

- **WHEN** the user activates a node in the default Block View tree
- **THEN** that node becomes the current selected block
- **AND** the Block View re-renders around the newly selected block

### Requirement: Default Inline view and title property

The default inline renderer MUST show the block's `title`, where `title` is a reserved property key whose value, when present, MUST be a `PropertyValue::String`. When the block has no `title` property (or its value is not a string), the default inline renderer MUST show the block id as a neutral placeholder. The default inline renderer MUST NOT invent descriptive or marketing placeholder copy.

#### Scenario: Title shown when present

- **WHEN** a block with a `title` string property is rendered by the default inline renderer
- **THEN** the rendered label is the `title` value

#### Scenario: Id placeholder when title absent

- **WHEN** a block with no `title` property is rendered by the default inline renderer
- **THEN** the rendered label is the block id

### Requirement: Block View component and state placement

Presentational Block View building blocks (tree node, inline block label, view-mode toggle, properties panel layout) MUST live in the design-system package and MUST remain session-agnostic: they MUST NOT call IPC/Tauri APIs and MUST NOT assume a current selected block. All Block View state — current selected block, active view mode, and expand/collapse state — MUST be held in the application shell, which wires session reads and mutations to the presentational components.

#### Scenario: Presentational components stay session-agnostic

- **WHEN** a Block View component is added to the design-system package
- **THEN** it receives data and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Shell owns Block View state

- **WHEN** the current selected block, view mode, or expand/collapse state changes
- **THEN** that state is held and updated in the application shell
- **AND** the shell passes the resulting data to the presentational components

## MODIFIED Requirements

### Requirement: Display property

`display` MUST be a reserved property key on a block. Its value MUST be a `PropertyValue::String` naming the renderer used to represent the block as the current selected block. `display` MUST be stored in the existing block properties map and MUST be changed only through the Properties View. Writes to `display` MUST go through the `set_property` block-property mutation (see the `mutations` capability) exposed by `core::Session`; the UI layer MUST NOT introduce Block-View-specific mutation or validation logic.

#### Scenario: Display is a reserved string property

- **WHEN** a block has a renderer configured
- **THEN** the block's properties map contains the key `display` with a `PropertyValue::String` value naming the renderer

#### Scenario: Display changed via the set_property mutation

- **WHEN** the Properties View changes the current selected block's `display`
- **THEN** the change is applied through the `core::Session` `set_property` mutation
- **AND** no Block-View-specific mutation or validation logic is introduced in the UI layer

## REMOVED Requirements

### Requirement: Raw fallback renderer

**Reason**: Replaced by the children-tree Default Block View. The fallback for an unset or unrecognized `display` is now the default renderer defined by the "Block View router" and "Default Block View" requirements, not a raw properties dump.

**Migration**: Behavior that relied on the unset-`display` block rendering its stored properties as-is now renders the Default Block View (the current block's children as a tree). Surfacing of unrecognized `display` values is preserved by the "Block View router" requirement.
