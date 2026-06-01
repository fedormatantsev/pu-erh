## MODIFIED Requirements

### Requirement: Structured Document View is a registered Block View renderer

The Document View MUST register renderer `document` with the **`desktop-shell-ui`** renderer registry as **primary-only** (inline not supported). When the current selected block's `display` is `document`, the primary surface MUST render the Document View. The key `document` MUST appear in the registry so the Properties View `display` dropdown per **`properties-view`** offers it. The Document View MUST coordinate Session only per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**.

#### Scenario: Display selects the Document View on primary

- **WHEN** the current selected block's `display` property is `document`
- **THEN** the primary surface renders the Document View

#### Scenario: Document offered in the display dropdown

- **WHEN** the Properties View renders the `display` dropdown
- **THEN** `document` appears as one of the registered renderer keys

#### Scenario: Document block shown inline in Tree View

- **WHEN** a block with `display` `document` appears in an inline context (for example a Tree View column)
- **THEN** the Document View component MUST NOT render
- **AND** the default inline renderer MUST be used per **`desktop-shell-ui`**, Scenario: **Primary-only registry entry falls back on inline dispatch**

### Requirement: Title rendered as an editable heading

The Document View MUST render the current selected block's `title` as an editable plain-text heading per **`property-registry`**, Requirement: **title property**, Scenario: **Absent title in user-facing editors**. The heading MUST be plain text only. Edits MUST apply in memory immediately through `set_property` per **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**.

#### Scenario: Title shown as heading when present

- **WHEN** the Document View renders a block whose `title` is a string
- **THEN** the heading shows the `title` value

#### Scenario: Empty heading when title absent or non-string

- **WHEN** the Document View renders a block with no `title` property or a non-string `title`
- **THEN** the heading is empty
- **AND** no placeholder or descriptive copy is shown

#### Scenario: Heading edit written immediately via set_property

- **WHEN** the user edits the heading
- **THEN** the change is applied in memory through `core::Session::set_property` with key `title`
- **AND** persistence follows **`desktop-shell`** save policy

### Requirement: Child blocks rendered as rich-text paragraphs

The Document View MUST render each direct child of the current selected block as one paragraph, in order per **`child-ordering`**. Each paragraph's content MUST come from that child's `body` property per **`property-registry`**, Requirement: **body property**. When a child has no `body` or a non-string `body`, the paragraph MUST render empty with no placeholder copy. Only direct children MUST be rendered.

#### Scenario: One paragraph per child in order

- **WHEN** the Document View renders a block with child blocks
- **THEN** each direct child is shown as one paragraph
- **AND** the paragraphs appear in **`child-ordering`** order

#### Scenario: Paragraph content from body property

- **WHEN** a child block has a string `body` property
- **THEN** its paragraph renders the rich text decoded from that `body` value

#### Scenario: Empty paragraph when body absent or non-string

- **WHEN** a child block has no `body` property or a non-string `body`
- **THEN** its paragraph renders empty
- **AND** no placeholder or descriptive copy is shown

#### Scenario: Nested descendants not shown

- **WHEN** a child block itself has children
- **THEN** the Document View renders only the child as a paragraph
- **AND** does not render the child's descendants

### Requirement: Supported rich-text formats in paragraphs

Paragraph editing MUST support exactly: bold, italic, underline, strikethrough, inline code; inline links; and quote block. No other formats. Format changes MUST update `body` in memory via `set_property` per **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**.

#### Scenario: Supported marks available

- **WHEN** the user formats selected paragraph text
- **THEN** bold, italic, underline, strikethrough, inline code, link, and quote are available
- **AND** no other formats are offered

#### Scenario: Formatting change written immediately via set_property

- **WHEN** the user applies a supported format to paragraph text
- **THEN** the affected child block's `body` is updated in memory through `core::Session::set_property` with key `body`
- **AND** persistence follows **`desktop-shell`** save policy

### Requirement: Structural editing maps to child mutations

Structural edits MUST map to `core::Session` mutations only per **`mutations`** and **`child-ordering`**: Enter creates a child with sibling-relative position; Backspace at paragraph start merges into the previous paragraph and deletes the child when not first; reordering uses `move_block`. The view MUST NOT add validation logic. All edits MUST follow **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**.

#### Scenario: Enter creates a following paragraph child

- **WHEN** the user presses Enter within a paragraph
- **THEN** a new child block is created via `create_block` positioned immediately after the source paragraph's child
- **AND** text after the caret is moved into the new paragraph by writing both `body` values via `set_property`

#### Scenario: First paragraph created when none exist

- **WHEN** the block has no children and the user starts a paragraph (Enter or text entry)
- **THEN** a first child block is created via `create_block`
- **AND** its `body` reflects the entered content

#### Scenario: Backspace at paragraph start merges into the previous paragraph

- **WHEN** the user presses Backspace at the start of a paragraph that is not the first
- **THEN** the preceding paragraph's `body` is updated via `set_property` to include this paragraph's content
- **AND** this paragraph's child block is deleted via `delete_block`

#### Scenario: Backspace at the first paragraph is a no-op

- **WHEN** the user presses Backspace at the start of the first paragraph
- **THEN** no mutation occurs and the structure is unchanged

#### Scenario: Reordering a paragraph moves its child

- **WHEN** the user reorders a paragraph
- **THEN** its child block is moved via `move_block` with the matching `Before`/`After` position
- **AND** the **`child-ordering`** `order` property reflects the new position

#### Scenario: Structural edits are in-memory until Save

- **WHEN** the user performs any structural edit
- **THEN** the mutation is applied in memory
- **AND** persistence follows **`desktop-shell`** save policy

### Requirement: Document View presentational components stay session-agnostic

Presentational Document View building blocks MUST live in the design-system package and MUST remain session-agnostic per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**. The Document View renderer wiring MUST live in `apps/desktop`.

#### Scenario: Presentational editor components receive data via props

- **WHEN** a Document View presentational component is added to the design-system package
- **THEN** it receives content and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Renderer wiring lives in the adapter layer

- **WHEN** the Document View resolves the current block's title and child paragraphs and applies edits
- **THEN** it reads and mutates state through `core::Session` in the application adapter layer
- **AND** it does not reimplement domain logic in the design-system package

## REMOVED Requirements

### Requirement: Document View save policy

**Reason**: Consolidated into **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**, and **`desktop-shell`** explicit Save policy.

**Migration**: Reference those capabilities instead of duplicating save rules in **`document-view`**.
