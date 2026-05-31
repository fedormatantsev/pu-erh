## Why

The desktop shell currently renders the view-mode toggle inline above the content and offers no way to create blocks from the UI, so the only path to a new child block is the CLI. A small, always-available action surface lets the user switch view mode and add a child to the current selected block without inline chrome competing with content.

## What Changes

- Add an **action bar** overlay: a compact floating panel pinned to the upper-left corner that overlays the content surface and adapts its actions to the current shell state.
- The action bar hosts two actions:
  - **Toggle view mode** between Block View and Properties View. This action subsumes the inline `ViewModeToggle` placement; the toggle moves into the action bar and the inline placement is removed.
  - **Create child block** — appends a new child of the current selected block through the existing `create_block` mutation. The action is disabled when there is no current selected block.
- Expose a `create_block` Tauri IPC command wiring the frontend to `core::Session::create_block(Some(parent))`.
- Creating a child does **not** change the current selected block and does **not** save (consistent with the no-save policy); the Block View re-reads so the new child appears in the children column.
- Permit the action bar as an allowed overlay surface in the desktop shell (the shell currently forbids surfaces beyond the Block View and Properties View).
- Add a live `ActionBar` example to the design-system showcase app.

## Capabilities

### New Capabilities
- `action-bar`: a context-adaptive floating overlay surface that exposes a fixed set of shell-level actions (view-mode toggle and create-child) wired to shell state and `core::Session`, with presentational parts session-agnostic in the design system.

### Modified Capabilities
- `desktop-shell`: extend the "Anti-default UI shell" requirement to permit the action bar as an allowed overlay surface, and add `create_block` to the minimal IPC commands.
- `design-showcase`: extend the "Showcase displays all base components" requirement to include a live `ActionBar` example.

## Impact

- New design-system component `ActionBar` in `packages/ui` (presentational, props/callbacks only), with a live example in `apps/showcase`.
- `apps/desktop`: action-bar wiring in the shell/`App`, removal of inline `ViewModeToggle` placement, new `createBlock` IPC wrapper in `ipc.ts`, Block View refresh after create.
- `apps/desktop/src-tauri`: new `create_block` command + capability/permission entry; `crates/desktop` `AppState::create_block` adapter calling `core::Session::create_block`.
- No new domain logic: the create mutation and `create_block` already exist in `core`/`graph`.
