## Context

The desktop app dispatches the current selected block to a primary renderer through the Block View router (`apps/desktop/src/views/blockView.tsx`), keyed by the block's `display` property. Today only `tree` (the three-column TreeView) is registered. Renderers are session-agnostic presentational pieces in `@pu-erh/ui`, wired to `core::Session` through thin Tauri IPC commands (`apps/desktop/src/ipc.ts` → `commands.rs` → `crates/desktop/src/state.rs` → `core::Session`). State (current block, view mode, refresh token) lives in the shell.

`core::Session` already exposes every mutation this change needs: `create_block(parent, PositionHint)`, `move_block(id, new_parent, PositionHint)`, `delete_block(id)`, `set_property(id, key, value)`. `PositionHint` supports `First | Last | Before(id) | After(id)` and the `child-ordering` capability assigns each parent edge an `order` fractional-index string. The desktop IPC currently exposes only a `Last`-positioned `create_block` plus `set_property` — no delete, no move, no positioned create.

This change adds a `document` renderer built on `lexical`, the first rich-text surface in the app, and the first to perform create/delete/move mutations from the UI.

## Goals / Non-Goals

**Goals:**
- A `document` primary Block View renderer: `title` as an editable plain-text heading, direct child blocks as editable rich-text paragraphs ordered by `child-ordering`.
- Rich text limited to bold, italic, underline, strikethrough, inline code, link, quote.
- Structural editing (Enter to split, Backspace-merge to delete, reorder) mapped strictly to `core::Session` mutations.
- Reuse the existing no-save policy: mutate in memory immediately, persist only on explicit Save.

**Non-Goals:**
- No nested/hierarchical rendering: only direct children are paragraphs; descendants are not shown.
- No new block types beyond paragraph and quote (no headings within the body, lists, images, tables, code blocks).
- No collaborative/real-time editing, no undo/redo beyond what a single Lexical editor provides locally.
- No changes to `graph`/`storage`/`core` mutation logic — only desktop IPC surface and frontend.
- The heading is plain text; no rich text in the title.

## Decisions

### Decision: One Lexical editor for the whole document, reconciled to child blocks on change

The body is a **single Lexical editor** holding one paragraph/quote node per direct child block, rather than one editor per paragraph. Cross-paragraph Enter/Backspace-merge, selection, and link insertion all work natively this way; per-paragraph editors would make caret movement and merge across paragraphs awkward.

To keep the mapping authoritative, each top-level Lexical node carries the originating **child block UUID** (stored on a custom node / node key map). On every editor change we diff the editor's top-level nodes against the last-known child list and translate the diff to mutations:

- **Node added** (Enter) → `create_block(parent, After(prevSiblingId))` (or `Last`/`First` when at an edge, `Last` when the document had no children), then `set_property(newId, "body", serialized)`. The returned UUID is attached to the new node.
- **Node removed** (Backspace-merge) → `set_property(survivingId, "body", mergedSerialized)` then `delete_block(removedId)`.
- **Node reordered** → `move_block(id, parent, Before/After(siblingId))`.
- **Node content/format changed** → `set_property(id, "body", serialized)`.

Each top-level node serializes independently to its child's `body` (Lexical's `$generateJSONFromSelectedNodes`-style export per node, or export of a single-node sub-tree). The heading is a separate, minimal plain-text editing surface bound to `title`.

**Alternative considered:** one editor per child block. Rejected — merge/split across blocks and a document-wide selection become hard, and Enter at a paragraph end would need cross-editor focus juggling.

**Alternative considered:** a single editor whose entire state serializes to one `body` on the document block. Rejected — it abandons the block-per-paragraph model the request requires ("children of the document block as paragraphs") and loses per-paragraph identity, ordering, and reuse by the Tree/Inline views.

### Decision: `body` is a new reserved property holding per-node Lexical JSON

Paragraph rich text is stored in a reserved `body` string property on each paragraph child, serialized as Lexical's editor-state JSON for that single node. `title` is left as the plain-text label the Tree/Inline views already render. Overloading `title` would break those views (they render it as a label). `body` is registered as well-known and excluded from the Properties View generic list because it is an opaque serialized payload, not user-facing scalar text.

**Alternative considered:** serialize to HTML. Rejected — Lexical's HTML import/export is lossy for some mark combinations and round-trips less reliably than native editor-state JSON.

### Decision: Restrict the editor to the seven supported formats

Register only the Lexical nodes/marks needed: text-format marks (bold, italic, underline, strikethrough, code), `LinkNode` (+ `LinkPlugin`), and `QuoteNode`. Omit list, heading, image, table, and code-block nodes from the editor config so unsupported formats cannot be produced or pasted in. A small formatting toolbar (presentational, in `@pu-erh/ui`) exposes exactly these controls.

### Decision: Extend desktop IPC with delete, move, and positioned create

Add to `crates/desktop/src/state.rs` + `commands.rs` + permissions + `ipc.ts`:
- `delete_block(id)` → `Session::delete_block`.
- `move_block(id, after: Option<String>)` (and/or `before`) → `Session::move_block(id, parent, PositionHint::After/Before)`. Parent is the current document block; the command resolves the existing parent.
- Extend create to accept an optional `after` sibling: `create_block(parent, after: Option<String>)` mapping to `PositionHint::After(after)` or `Last`. Keep the existing no-arg-position behavior (Last) as the default so the action-bar "New child" path is unaffected.

Each command is a thin wrapper, no save, `CoreError`-derived errors surfaced verbatim — consistent with existing commands and the `desktop-shell` spec delta.

### Decision: Save via an explicit control, consistent with PropertiesView

Edits call `set_property`/`create_block`/`delete_block`/`move_block` immediately (in-memory), matching the documented no-save policy and the title-slot/PropertiesView precedent. Persistence is an explicit Save. The Document View provides a Save control (mirroring `PropertiesView`'s in-panel Save button) rather than inventing an auto-save or save-on-blur. Structural mutations bump the shell `refreshToken` only where the view needs to re-read; the editor itself is the source of truth for in-flight text so we avoid clobbering the caret on every keystroke.

## Risks / Trade-offs

- **Editor↔block diff drift** → A subtle diff (e.g. mis-identifying a reorder as delete+create) could create/delete the wrong blocks. Mitigation: anchor every top-level node to its child UUID and diff by identity, not position; treat unknown nodes as creates and missing UUIDs as deletes; cover split/merge/reorder with explicit tests.
- **Mutation latency vs. keystrokes** → Calling `set_property` per keystroke over IPC could lag. Mitigation: debounce `body` writes per node; structural mutations (create/delete/move) fire on the discrete Enter/Backspace/reorder events, not per keystroke. All writes remain in-memory (no disk I/O) until Save.
- **`create_block` returns the new UUID asynchronously** → the new Lexical node exists before its block id is known. Mitigation: keep a pending-node map; attach the returned UUID when `create_block` resolves; defer the node's first `body` write until the id is bound.
- **Lexical bundle size / new dependency** → first heavy frontend dependency. Mitigation: import only the needed Lexical packages and nodes; acceptable for a desktop app.
- **Anti-default scope creep** → structural editing is broad surface. Mitigation: the spec fixes exactly which gestures map to which mutations; no copy, no extra block types, no unspecified shortcuts.

## Open Questions

- Exact Lexical packages/versions to pin (core `lexical`, `@lexical/react`, `@lexical/link`, `@lexical/rich-text`, `@lexical/utils`) — resolved during implementation against the installed registry.
- Per-node serialization helper: use Lexical's node export API vs. a thin custom serializer for a single top-level node — decided in implementation; behavior (round-trip of the seven formats) is what the spec constrains.
- Debounce interval for `body` writes — a tuning detail, not spec-visible.
