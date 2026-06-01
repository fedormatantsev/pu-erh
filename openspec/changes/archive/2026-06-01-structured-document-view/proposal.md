## Why

The only primary Block View today is the three-column TreeView, which is good for navigating hierarchy but not for reading or writing prose. A block whose children are paragraphs of text has no surface that renders it as a document. This change adds a Structured Document View — a primary Block View renderer that shows a block's `title` as a heading and its child blocks as editable rich-text paragraphs — so a block can be authored and read as a document.

## What Changes

- Add a **Structured Document View**: a new primary Block View renderer, registered with the Block View router under the `display` value `document`, built on the `lexical` rich-text editor package.
- Render the current selected block's `title` (reserved string property) as an editable plain-text **heading**.
- Render each **child block** of the current selected block as an editable **paragraph**, ordered by the `child-ordering` `order` property.
- Support **rich text** in paragraphs: bold, italic, underline, strikethrough, quote, link, and inline code. The heading stays plain text.
- Persist paragraph rich text in a new reserved **`body`** property (one per paragraph child) holding Lexical's serialized editor state (a `PropertyValue::String`). `title` remains the plain-text label used by the Tree/Inline views.
- Support **structural editing**: pressing Enter splits/creates a paragraph child, Backspace at a paragraph start merges into the previous paragraph (deleting the emptied child), and paragraph order is maintained through the `child-ordering` fractional-index `order` property. Editing maps only to `core::Session` mutations (`create_block`, `delete_block`, `move_block`, `set_property`).
- Follow the existing desktop **no-save policy**: edits apply in memory immediately through mutations; persistence to disk happens only via the explicit Save control.
- Extend the desktop shell IPC with the mutations the editor needs: **delete a block**, **move a block** (reorder), and **create a child at a sibling-relative position** (the current create command only appends last).

## Capabilities

### New Capabilities
- `document-view`: the Structured Document View renderer — heading from `title`, child blocks as rich-text paragraphs backed by the `body` property, the supported rich-text marks/nodes, the mapping from editor structure changes to `core::Session` mutations, registration with the Block View router, and its save policy.

### Modified Capabilities
- `desktop-shell`: extend the Block-view IPC commands with delete-block, move-block, and sibling-relative create-child (positioned) wrappers over `core::Session`, each thin and without an unspecified save.
- `well-known-properties`: register `body` as a reserved property key (serialized rich-text content) that is excluded from the generic, user-editable properties list.

## Impact

- New dependency: `lexical` (and its React bindings) added to `apps/desktop`.
- `apps/desktop/src/views/`: new `DocumentView` renderer; registration in `blockView.tsx` `blockRenderers` so `document` is a selectable `display` value.
- `apps/desktop/src/ipc.ts` and `apps/desktop/src-tauri/src/commands.rs`: new `delete_block`, `move_block`, positioned `create_block` commands; matching `crates/desktop/src/state.rs` `AppState` methods and Tauri permissions.
- Presentational document-editor building blocks (toolbar, editor chrome) added to `@pu-erh/ui`, kept session-agnostic per the Block View component-placement rule.
- No changes to `graph`, `storage`, or `core` mutation logic — those mutations already exist; this change exposes and orchestrates them.
