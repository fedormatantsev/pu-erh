# document-view Specification

## Purpose
TBD - created by syncing change structured-document-view. Update Purpose after archive.
## Requirements
### Requirement: Structured Document View is a registered Block View renderer

The Structured Document View MUST be a primary Block View renderer registered with the Block View router under the `display` value `document`. When the current selected block's `display` property is `document`, the Block View router MUST select the Structured Document View as the primary renderer. The renderer name `document` MUST appear among the registered block view names so the Properties View `display` dropdown offers it. The Structured Document View MUST read block state and apply mutations only through `core::Session` (via the desktop IPC commands) and MUST NOT duplicate graph, CRDT, trie, or mutation-validation logic.

#### Scenario: Display selects the Document View

- **WHEN** the current selected block's `display` property is `document`
- **THEN** the Block View router renders the block with the Structured Document View

#### Scenario: Document offered in the display dropdown

- **WHEN** the Properties View renders the `display` dropdown
- **THEN** `document` appears as one of the registered view name options

### Requirement: Title rendered as an editable heading

The Structured Document View MUST render the current selected block's `title` (a reserved string property) as an editable plain-text heading. When the block has no `title` property or its value is not a string, the heading MUST be empty; the view MUST NOT invent descriptive or placeholder copy. The heading MUST be plain text only — rich-text marks and nodes MUST NOT be available in the heading. Editing the heading MUST be applied in memory immediately through the `set_property` mutation with key `title`, and MUST NOT be persisted to disk until the explicit Save control is invoked.

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
- **AND** no save to disk occurs until the user invokes the explicit Save control

### Requirement: Child blocks rendered as rich-text paragraphs

The Structured Document View MUST render each direct child of the current selected block as one paragraph, in the order defined by the `child-ordering` `order` property. Each paragraph's content MUST be sourced from that child block's `body` property — a reserved `PropertyValue::String` holding the paragraph's serialized rich-text editor state. When a child has no `body` property or its value is not a string, that paragraph MUST render as empty (no placeholder copy). Only direct children MUST be rendered as paragraphs; descendants nested below a child MUST NOT be shown by this view.

#### Scenario: One paragraph per child in order

- **WHEN** the Document View renders a block with child blocks
- **THEN** each direct child is shown as one paragraph
- **AND** the paragraphs appear in the children's `child-ordering` order

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

Paragraph editing MUST support exactly the following rich-text formats and no others: the inline marks bold, italic, underline, strikethrough, and inline code; inline links; and the quote block. The set of formats MUST be fixed; the view MUST NOT expose additional block types or marks. Applying or removing a format MUST update the affected child block's `body` property in memory immediately through the `set_property` mutation, and MUST NOT persist to disk until the explicit Save control is invoked.

#### Scenario: Supported marks available

- **WHEN** the user formats selected paragraph text
- **THEN** bold, italic, underline, strikethrough, inline code, link, and quote are available
- **AND** no other formats are offered

#### Scenario: Formatting change written immediately via set_property

- **WHEN** the user applies a supported format to paragraph text
- **THEN** the affected child block's `body` is updated in memory through `core::Session::set_property` with key `body`
- **AND** no save to disk occurs until the user invokes the explicit Save control

### Requirement: Structural editing maps to child mutations

The Structured Document View MUST let the user change paragraph structure, and each structural change MUST map to `core::Session` mutations only:

- Pressing Enter within or at the end of a paragraph MUST create a new child block as a paragraph immediately after the source paragraph, using `create_block` with a sibling-relative position (`After` the source child); any text after the caret MUST move into the new paragraph by writing both paragraphs' `body` via `set_property`. When the block has no children yet, the first Enter (or first text entry) MUST create the first child paragraph.
- Pressing Backspace at the start of a paragraph MUST merge that paragraph into the preceding paragraph: the preceding paragraph's `body` is updated via `set_property` to include the merged content, and the now-removed child block is deleted via `delete_block`. Backspace at the start of the first paragraph MUST be a no-op (there is no preceding paragraph).
- Reordering a paragraph MUST move its child block via `move_block` with the corresponding `Before`/`After` sibling-relative position, preserving the `child-ordering` `order` property.

All structural changes MUST be applied in memory immediately and MUST NOT be persisted to disk until the explicit Save control is invoked. The view MUST NOT introduce any block-property or ordering validation logic of its own.

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
- **AND** the `child-ordering` `order` property reflects the new position

#### Scenario: Structural edits are in-memory until Save

- **WHEN** the user performs any structural edit
- **THEN** the mutation is applied in memory
- **AND** no save to disk occurs until the user invokes the explicit Save control

### Requirement: Document View save policy

The Structured Document View MUST follow the desktop shell's no-save policy: all heading, paragraph, and structural edits MUST be applied in memory immediately through mutations, and persistence to disk MUST happen only when the user invokes an explicit Save control. The view MUST NOT auto-save, save on blur, or save on close.

#### Scenario: Explicit Save persists pending edits

- **WHEN** the user invokes the Save control after editing
- **THEN** the in-memory edits are persisted via `core::Session::save`

#### Scenario: No implicit save

- **WHEN** the user edits the heading or a paragraph and does not invoke Save
- **THEN** no save to disk occurs

### Requirement: Document View presentational components stay session-agnostic

Presentational building blocks of the Structured Document View (the editor chrome, formatting toolbar, heading and paragraph editor styling) MUST live in the design-system package and MUST remain session-agnostic: they MUST NOT call IPC/Tauri APIs and MUST NOT assume a current selected block. The Document View renderer that wires session reads and mutations to those presentational components MUST live in the application adapter layer.

#### Scenario: Presentational editor components receive data via props

- **WHEN** a Document View presentational component is added to the design-system package
- **THEN** it receives content and callbacks as props
- **AND** it does not call IPC/Tauri APIs or assume a current selected block

#### Scenario: Renderer wiring lives in the adapter layer

- **WHEN** the Document View resolves the current block's title and child paragraphs and applies edits
- **THEN** it reads and mutates state through `core::Session` in the application adapter layer
- **AND** it does not reimplement domain logic in the design-system package
