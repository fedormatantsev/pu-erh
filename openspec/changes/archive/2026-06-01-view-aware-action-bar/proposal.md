## Why

The ActionBar currently exposes a fixed set of shell-level actions (toggle-mode and create-child) regardless of which view is active, so `create-child` is always offered even when the Properties View — where it has no meaning — is shown. Separately, the routing logic that chooses between the Block View and Properties View is an ad-hoc conditional in `App.tsx`, not a first-class concept, which makes it harder to extend or test in isolation.

## What Changes

- **View-provided actions**: each view (TreeView, PropertiesView) declares the set of actions it contributes; the shell gathers those and passes them to the ActionBar rather than hard-wiring a fixed list.
- **TreeView actions**: `create-child` + `toggle-to-properties-view`.
- **PropertiesView actions**: `toggle-to-block-view` only (no `create-child`).
- **Explicit View Router**: the shell's inline `viewMode === "block" ? … : …` is replaced by a formal `ViewRouter` — a pure function `(blockId, mode) → ReactElement` that encapsulates the dispatch logic.
- **Shell state contract**: the shell continues to own `currentBlockId` and `viewMode`; it passes them to the View Router and wires handlers down.

## Capabilities

### New Capabilities

*(none)*

### Modified Capabilities

- `action-bar`: action set is no longer fixed by the shell; it is assembled from the actions declared by the currently active view. The two-action requirement (`toggle-mode`, `create-child`) is replaced by a view-driven list where each view opts in or out of each action.
- `block-view`: the View Router is formalized as an explicit, testable dispatch function `(blockId: string, mode: ViewMode) → ReactElement` rather than an ad-hoc conditional; spec adds a requirement for this contract.

## Impact

- `apps/desktop/src/App.tsx` — remove inline view conditional; add `ViewRouter` component/function wired through shell state.
- `apps/desktop/src/shell.tsx` — no new state; ViewRouter reads existing `currentBlockId` and `viewMode`.
- `apps/desktop/src/views/blockView.tsx` — TreeView exports its action descriptors (create-child, toggle-to-properties).
- `apps/desktop/src/views/PropertiesView.tsx` — PropertiesView exports its action descriptors (toggle-to-block-view only).
- `packages/ui/src/components.tsx` — ActionBar receives a generic `actions` list instead of the current fixed-prop surface (`onToggleMode`, `onCreateChild`, `canCreateChild`). **BREAKING** for ActionBar component API.
