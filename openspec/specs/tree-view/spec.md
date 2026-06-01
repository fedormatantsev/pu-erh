# tree-view Specification

## Purpose

Default primary Block View renderer: three-column Tree View layout, activation and arrow-key selection policy, and Tree View action descriptors.
## Requirements
### Requirement: Tree View is the default primary renderer

The Tree View MUST register as the default primary renderer in the **`desktop-shell-ui`** renderer registry. When the current selected block's `display` is unset or unrecognized, the Tree View MUST render as the primary surface.

#### Scenario: Tree View selected as default primary

- **WHEN** the current selected block has no `display` property or an unrecognized `display` value
- **THEN** the Tree View renders as the primary surface per **`desktop-shell-ui`**, Requirement: **Default renderer**

### Requirement: Three-column hierarchy layout

The Tree View MUST display the current selected block within a three-column layout. Columns MUST be, left to right: (1) the single parent of the current selected block; (2) the current selected block together with its sibling blocks (ordered per **`child-ordering`**), with the current selected block visibly distinguished; (3) the children of the current selected block.

Every block shown in any column MUST be rendered inline via the **`desktop-shell-ui`** renderer registry per **`desktop-shell-ui`**, Requirement: **Renderer registry**.

The left column MUST be empty when the current selected block is the root block. The right column MUST be empty when the current selected block has no children.

#### Scenario: Three columns reflect the hierarchy

- **WHEN** the Tree View renders the current selected block
- **THEN** the left column shows that block's parent, the center column shows that block and its siblings with the current block distinguished, and the right column shows that block's children
- **AND** each shown block is rendered inline via the renderer registry

#### Scenario: Root hides the parent column content

- **WHEN** the current selected block is the root block
- **THEN** the left (parent) column shows no block

#### Scenario: No children hides the children column content

- **WHEN** the current selected block has no children
- **THEN** the right (children) column shows no block

#### Scenario: Columns derive from current selected block

- **WHEN** the Tree View renders
- **THEN** the parent, sibling, and children columns are derived from the current selected block
- **AND** the shell holds no separate per-node expand/collapse state

### Requirement: Tree View selection policy

The Tree View MUST support two ways to change the current selected block:

- **Activation**: activating (for example clicking) a block's inline preview in any column MUST make that block the current selected block.
- **Arrow-key navigation**: `←` MUST select the parent; `→` MUST select the first child; `↑` MUST select the previous sibling; `↓` MUST select the next sibling within the center column. Each key action MUST be a no-op when the target does not exist.

Whenever the current selected block changes, the Tree View MUST re-render so columns re-center on the newly selected block. The Tree View MUST NOT introduce block-property mutation; selection is ephemeral UI state.

#### Scenario: Activating a block selects it

- **WHEN** the user activates a block's inline preview in any column
- **THEN** that block becomes the current selected block
- **AND** the Tree View re-centers the columns on the newly selected block

#### Scenario: Arrow keys move the selection and re-center

- **WHEN** the user presses `←`, `→`, `↑`, or `↓`
- **THEN** the current selected block becomes the parent, the first child, the previous sibling, or the next sibling respectively
- **AND** the Tree View re-centers the columns on the newly selected block

#### Scenario: Arrow keys are no-ops at hierarchy edges

- **WHEN** the user presses `←` at the root, `→` with no children, or `↑`/`↓` past the first/last sibling
- **THEN** the current selected block does not change

### Requirement: Tree View action descriptors

When the Tree View is the active view (Block View mode), its action descriptor factory MUST return toggle-to-properties and create-child actions per **`action-bar`**.

#### Scenario: Tree View supplies toggle and create-child actions

- **WHEN** the Tree View is the active view
- **THEN** its action factory returns toggle-to-properties and create-child descriptors
- **AND** no other actions

