## 1. Backend: create-child IPC command

- [x] 1.1 Add `AppState::create_block(&self, parent: &str) -> Result<String, String>` in `crates/desktop/src/state.rs`, parsing the parent UUID and calling `core::Session::create_block(Some(parent))`, returning the new id as a string and surfacing `CoreError` as-is (no save)
- [x] 1.2 Add a `create_block` Tauri command in `apps/desktop/src-tauri/src/commands.rs` wrapping `AppState::create_block`, and register it in the invoke handler in `lib.rs`
- [x] 1.3 Add the `allow-create-block` permission (`apps/desktop/src-tauri/permissions/`) and include it in `capabilities/default.json`

## 2. Frontend IPC wrapper

- [x] 2.1 Add `createBlock(parent: string): Promise<string>` to `apps/desktop/src/ipc.ts` invoking the `create_block` command

## 3. Design system: ActionBar component

- [x] 3.1 Add a presentational `ActionBar` component to `packages/ui/src/components.tsx` taking props: active `mode`, `onToggleMode`, `onCreateChild`, and `canCreateChild` (enabled flag); render the view-mode toggle and a create-child control, disabling create when `canCreateChild` is false; no IPC, no shell reads
- [x] 3.2 Export `ActionBar` from `packages/ui/src/index.ts`
- [x] 3.3 Style the action bar as a fixed upper-left floating overlay: compact, low-contrast, no decorative border, not displacing content, per `ui-direction`
- [x] 3.4 Add an `ActionBar` example to the Components section of `apps/showcase/src/App.tsx` using local example state (a selected mode and an enabled create action), with session-agnostic no-op/local handlers and no IPC calls

## 4. Shell wiring

- [x] 4.1 In `apps/desktop/src/shell.tsx`, add a `createChild` handler that calls `createBlock(currentBlockId)` and a refresh signal (e.g. a bump counter) exposed on the shell context; surface mutation errors as returned
- [x] 4.2 Ensure `createChild` does not change `currentBlockId` and does not call `save`
- [x] 4.3 In `apps/desktop/src/App.tsx`, render `ActionBar` wired to shell state (`viewMode`/`setViewMode`, `createChild`, `canCreateChild = currentBlockId != null`); remove the inline `ViewModeToggle` placement
- [x] 4.4 Pass the refresh signal into `BlockViewHost`/the children-reading view so the children column re-reads after a create

## 5. Verification

- [ ] 5.1 `cargo build -p desktop` and run the desktop dev command; confirm the action bar appears in the upper-left overlaying content — `cargo build -p desktop` and `bun run build` (apps/desktop) both pass; live GUI walkthrough pending a manual `bun run tauri dev` (cannot launch the desktop GUI in this environment)
- [ ] 5.2 Toggle action switches Block View ↔ Properties View with exactly one shown; no second inline toggle exists — code wired (single toggle in `ActionBar`, inline `ViewModeToggle` removed); live GUI confirmation pending manual run
- [ ] 5.3 Create-child adds a child to the current block that appears in the children column without changing selection and without saving; disabled when no current block; mutation errors surface verbatim — backend covered by `crates/desktop` unit tests (append-child, nonexistent-parent error); live GUI confirmation pending manual run
- [x] 5.4 `bun run build` in `apps/showcase` succeeds and the Components section shows a live `ActionBar` example
- [x] 5.5 `openspec validate add-action-bar-overlay` passes
