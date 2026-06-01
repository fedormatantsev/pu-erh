## 1. Desktop IPC: delete, move, positioned create

- [x] 1.1 Add `delete_block`, `move_block`, and positioned `create_block` (optional `after` sibling) methods to `AppState` in `crates/desktop/src/state.rs`, each a thin wrapper over `core::Session` mapping `after` to `PositionHint::After`/default `Last`; surface `CoreError`-derived errors verbatim, no save.
- [x] 1.2 Add matching `#[tauri::command]` wrappers in `apps/desktop/src-tauri/src/commands.rs` (`delete_block`, `move_block`, extended `create_block`) and register them in the invoke handler in `lib.rs`.
- [x] 1.3 Add Tauri permission files (`allow-delete-block.toml`, `allow-move-block.toml`) and update `capabilities/default.json`; keep `allow-create-block` covering the extended signature.
- [x] 1.4 Add `deleteBlock`, `moveBlock`, and the `after`-aware `createBlock` wrappers to `apps/desktop/src/ipc.ts`.
- [x] 1.5 Add an ordered-children read command: `AppState::children_ordered` (over `KnowledgeBase::children_ordered`), the `children_ordered` Tauri command + permission + handler registration, and a `getChildrenOrdered` ipc wrapper — the unordered `children` command does not expose edge `order`.
- [x] 1.6 Add Rust tests in `state.rs` covering delete, move (reorder verified via ordered children), and create-after-sibling against an in-memory session.

## 2. Lexical dependency and design-system editor components

- [x] 2.1 Add `lexical` and the needed React/node packages (`@lexical/react`, `@lexical/rich-text`, `@lexical/link`, `@lexical/utils`) to `apps/desktop/package.json`; install via bun.
- [x] 2.2 Add a session-agnostic presentational document editor + formatting toolbar to `@pu-erh/ui` (heading slot, body editor slot, toolbar with bold/italic/underline/strikethrough/code/link/quote controls) — props/callbacks only, no IPC, export from `packages/ui/src/index.ts`.
- [x] 2.3 Configure the Lexical editor with exactly the supported nodes/marks (text-format marks, `LinkNode` + `LinkPlugin`, `QuoteNode`); omit list/heading/image/table/code-block nodes so unsupported formats cannot be produced or pasted.

## 3. Document View renderer (adapter layer)

- [x] 3.1 Create `apps/desktop/src/views/documentView.tsx` exporting a `DocumentView` primary renderer; read the current block and its children via IPC, ordered by `child-ordering`.
- [x] 3.2 Render `title` as an editable plain-text heading; empty when absent/non-string (no placeholder); write changes immediately via `setProperty(id, "title", ...)`.
- [x] 3.3 Build the single-editor body: one top-level Lexical node per child, each decoded from the child's `body` (empty when absent/non-string), each anchored to its child UUID per the design's node↔block mapping.
- [x] 3.4 Implement per-node `body` serialization and a debounced `setProperty(childId, "body", ...)` write on content/format changes.
- [x] 3.5 Implement the editor-change diff → mutations: node added (Enter) → positioned `createBlock` then bind returned UUID and write `body`; node removed (Backspace-merge) → write surviving `body` then `deleteBlock`; reorder → `moveBlock`. Handle the empty-document first-paragraph and first-paragraph Backspace no-op cases.
- [x] 3.6 Add an explicit Save control that calls the `save` IPC; no auto-save, save-on-blur, or save-on-close.
- [x] 3.7 Register `document` in the `blockRenderers` registry in `apps/desktop/src/views/blockView.tsx` so `resolveBlockView` selects it and `BLOCK_VIEW_NAMES` includes it (Properties View dropdown offers `document`).

## 4. Properties View: exclude reserved `body`

- [x] 4.1 Exclude `body` from the generic, user-editable properties list in `PropertiesView` (alongside `title`/`display`), so it is never shown as a free-text item.

## 5. Verification

- [x] 5.1 Type-check the desktop app (`tsc -p tsconfig.app.json --noEmit`) and build the UI package; run `cargo test -p desktop`.
- [ ] 5.2 Manually verify against the spec scenarios: set a block's `display` to `document`; heading edits, paragraph rich-text marks (bold/italic/underline/strikethrough/code/link/quote), Enter-split, Backspace-merge, reorder, first-paragraph Backspace no-op, and explicit Save persisting to disk.
- [x] 5.3 Run `openspec validate structured-document-view --strict` and resolve any issues.
