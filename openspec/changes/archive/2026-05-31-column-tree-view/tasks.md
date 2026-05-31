## 1. Backend: parent IPC command

- [x] 1.1 Add `AppState::parent(&self, id: &str) -> Result<Option<BlockDto>, String>` in `crates/desktop/src/state.rs`, wrapping `session.query("parent:<id>")` and mapping the zero-or-one result through `BlockDto::new` with `kb.has_children` (mirror `children`).
- [x] 1.2 Add unit tests in `state.rs`: a child returns its parent (with correct `has_children`); the root returns `None`; a missing id surfaces the `CoreError`-derived string.
- [x] 1.3 Add the `parent` Tauri command in `apps/desktop/src-tauri/src/commands.rs` and register it in the `invoke_handler` in `apps/desktop/src-tauri/src/lib.rs`.
- [x] 1.4 Add the Tauri permission file (mirror `permissions/allow-children.toml`) and wire it into `capabilities/default.json`.

## 2. Frontend IPC + types

- [x] 2.1 Add `getParent(id: string): Promise<BlockDto | null>` to `apps/desktop/src/ipc.ts` invoking the `parent` command.

## 3. Design-system column components

- [x] 3.1 Add presentational column components to `packages/ui/src/components.tsx`: a three-column container and a column list of inline blocks supporting a "current" highlight. Keep them session-agnostic (props + callbacks, no IPC).
- [x] 3.2 Add minimalist styling in `packages/ui/src/styles.css` using spacing and typography to communicate the parent → current → children structure; export the new components from `packages/ui/src/index.ts`.
- [x] 3.3 Remove the now-unused `TreeNode` component (and its expand toggle) from `@pu-erh/ui`; update `apps/showcase/src/App.tsx` if it references `TreeNode`.
- [x] 3.4 Add a `Disclosure` entry to the Components section of `apps/showcase/src/App.tsx` that renders a live TreeView column example with static sample data — a parent, a current block with siblings, and children, with the current block distinguished — and a `mono` caption noting the tokens used.

## 4. TreeView renderer + shell wiring

- [x] 4.1 In `apps/desktop/src/shell.tsx`, remove `expandedIds`/`toggleExpanded` from `ShellState` and the provider.
- [x] 4.2 Replace `DefaultBlockView`/`TreeBranch` in `apps/desktop/src/views/blockView.tsx` with the three-column TreeView: load parent (`getParent`), center siblings (children of parent, or `[root]` when no parent), and children (`getChildren`) of the current selected block; render each cell inline via `resolveInlineView`; highlight the current block; keep it registered under the `tree` key.
- [x] 4.3 Wire activation: clicking an inline block in any column calls `selectBlock` and re-centers.
- [x] 4.4 Wire arrow-key navigation on a focusable Block View container: `←` → parent id, `→` → first child id, `↑`/`↓` → previous/next sibling id within the rendered center list; each a no-op when the target does not exist; arrows call `selectBlock`.
- [x] 4.5 Render nothing for loading/empty columns (root → empty parent column; no children → empty children column) and surface IPC errors as the returned `CoreError` string via `Text`.

## 5. Verification

- [x] 5.1 Run `cargo test -p pu-erh-desktop` (or workspace tests) for the new `parent` command tests.
- [x] 5.2 Typecheck/build the frontend (desktop + ui + showcase) and confirm no references to removed `TreeNode`/`expandedIds` remain.
- [ ] 5.3 Manually verify in the running desktop app: columns reflect parent/current+siblings/children; click and all four arrow keys move and re-center; root has an empty parent column; a childless block has an empty children column; edge-arrow presses are no-ops.
