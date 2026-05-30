## Why

The `block-view` capability defines the UI contract (current selected block, Block View, Properties View, `display` dispatch) but ships no code — the desktop app is still the bare ping/root_id scaffold. This change builds the runnable framework: the view routers, a working Default Block View, a default Inline view, and the Properties View with `display` editing. It is the first change to render and mutate real block content in the desktop app.

## What Changes

- Add a generic **`set_property` mutation** (`set_property(block, key, value)`) to `core`/`graph`, mirroring create/move/delete as an append-only block version record. Backs `display` (and future property) editing.
- Add **read-only and write IPC commands** to the desktop shell: read a block, read a block's children, set a block property, and an explicit save — mirroring the `children:` query and `Session` APIs.
- Add presentational **Block View components to `packages/ui`** (tree node, inline block, view-mode toggle, properties panel layout) — no Tauri, no current-block/session assumptions, per the design-system boundary.
- Add **shell state management in `apps/desktop`**: current selected block (initialized to root), active view mode (Block View vs Properties View), and expand/collapse state — wiring IPC to the presentational components.
- Introduce the **Block View router** and **Inline View router**: dispatch a block to a renderer by its `display` property; both have a default; unrecognized `display` values are surfaced and fall back to the default.
- Implement the **Default Block View**: a recursive children tree of the current selected block, with expand/collapse controls, where clicking a node makes it the current selected block (this is the Default view's selection policy).
- Implement the **default Inline view**: shows the block's `title` (a reserved string property); when unset, shows the block id as a neutral placeholder.
- Implement the **Properties View** with full `display` editing: change the current block's `display` via the `set_property` mutation, persisted only through an **explicit Save control** (no auto-save).
- **BREAKING (contract-level)**: the children-tree Default view replaces the `block-view` "Raw fallback renderer"; and `desktop-shell`'s "No unspecified save policy" is amended to permit one explicit Save control.

## Capabilities

### New Capabilities

- None. This change extends the existing `block-view` capability with concrete framework requirements.

### Modified Capabilities

- `block-view`: ADD the Block View router, Inline View router, Default Block View (recursive tree + expand/collapse + click-selects), default Inline view + reserved `title` property, and component/state placement requirements. MODIFY "Raw fallback renderer" (children tree replaces raw) and "Display property" (written via the new `set_property` mutation through the Properties View).
- `mutations`: ADD the generic `set_property` block-property mutation (append-only, validated against the active view).
- `desktop-shell`: ADD block-view IPC commands (read block, read children, set property, save). MODIFY "No unspecified save policy" to permit one explicit Save control. MODIFY "Anti-default UI shell" so the Default Block View's in-content children tree with expand/collapse is permitted as the Block View renderer, while separate sidebar/tree chrome and file dialogs remain forbidden.

## Impact

- **Specs**: delta edits to `block-view`, `mutations`, `desktop-shell`. `block-model` and `frontend-scaffold` are respected, not changed (`title` is a UI-reserved property key like `display`; `packages/ui` stays presentational).
- **Code**: `crates/core` (`mutation.rs`, `session.rs`), `crates/graph` if needed for the property merge helper, `crates/desktop` (`state.rs`), `apps/desktop/src-tauri` (`commands.rs`, `lib.rs` handler registration), `packages/ui` (new components), `apps/desktop/src` (shell state, routers, views).
- **Dependencies**: none added — shell state uses React built-ins (context/`useState`), consistent with the existing scaffold.
- **Deferred**: non-default renderers (chart, calendar, document editor), clearing/removing a property, selection/view-mode persistence, save-on-close, and keyboard shortcuts remain out of scope.
