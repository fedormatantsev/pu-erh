## MODIFIED Requirements

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
