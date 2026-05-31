## Context

The action bar in `App.tsx` (`Workspace`) always renders two fixed actions — toggle-mode and create-child — regardless of which view is active. When the Properties View is shown, `create-child` appears but has no meaning there. The view selection is an inline `viewMode === "block" ? <BlockViewHost> : <PropertiesView>` conditional with no first-class home.

Current wiring:
```
Shell (currentBlockId, viewMode, createChild, setViewMode)
  → Workspace renders ActionBar with hard-coded onToggleMode + onCreateChild
  → Workspace inline-branches on viewMode to pick the view
```

Target wiring:
```
Shell (currentBlockId, viewMode, …)
  → ViewRouter(blockId, mode) → { View, actions }
  → Workspace renders ActionBar with actions from ViewRouter
  → Workspace renders View from ViewRouter
```

## Goals / Non-Goals

**Goals:**
- Each view declares its own actions; ActionBar renders exactly what the active view provides.
- View dispatch is a named function with a clear signature instead of an inline conditional.
- TreeView supplies: toggle-to-properties + create-child. PropertiesView supplies: toggle-to-block-view only.
- ActionBar `@pu-erh/ui` API changes from fixed props to a generic `actions` list.

**Non-Goals:**
- Adding new views or actions beyond the two currently existing.
- Persisting view mode or action state.
- Changing shell state shape (`currentBlockId`, `viewMode`, `refreshToken`).

## Decisions

### Decision: ViewRouter returns a `ViewDescriptor` — both the component and an action factory

`ViewRouter(blockId: string, mode: ViewMode): ViewDescriptor` where:
```ts
type ViewDescriptor = {
  View: (props: { blockId: string }) => ReactElement;
  actions: (ctx: ViewActionContext) => ActionBarAction[];
};
```
`ViewActionContext` carries only the shell callbacks and derived flags needed to build the action list (`setViewMode`, `createChild`, `canCreateChild`). `Workspace` calls `viewRouter`, then renders `<ActionBar actions={descriptor.actions(ctx)} />` and `<descriptor.View blockId={…} />`.

**Alternative considered**: a React context-based "action registry" where each view pushes actions into a context at render time. Rejected — too indirect for the two views here, and it makes the action list invisible until the view mounts.

**Alternative considered**: keeping actions in the shell and passing `mode` to ActionBar so ActionBar itself decides what to show. Rejected — this encodes view-specific knowledge in the presentational layer.

### Decision: `ActionBarAction` type in `apps/desktop/src/types.ts`; ActionBar API in `@pu-erh/ui` accepts `actions: ActionBarAction[]`

The new `ActionBarAction` type:
```ts
type ActionBarAction = {
  id: string;
  label: string;
  onPress: () => void;
  isDisabled?: boolean;
  pressed?: boolean;  // drives aria-pressed on toggle actions
};
```
Kept in the app layer (`types.ts`) rather than exported from `@pu-erh/ui` because it's the application's vocabulary for wiring shell callbacks to a presentational list. The `ActionBar` component signature becomes `{ actions: ActionBarAction[] }` and renders one button per entry.

**Alternative considered**: exporting `ActionBarAction` from `@pu-erh/ui`. Rejected — the design-system package is session-agnostic and has no dependency on shell concepts.

### Decision: `ViewRouter` lives in `apps/desktop/src/viewRouter.ts` (new file)

A dedicated module makes the dispatch explicit and independently testable. It imports both view modules and exports `viewRouter` as a named function. `App.tsx` imports `viewRouter`.

### Decision: `aria-pressed` is driven by `pressed` on the action descriptor

The toggle actions in each view set `pressed: true` when the view they toggle away from is active (i.e., the toggle reflects current state). This keeps the ARIA semantics correct without ActionBar needing to know which action is the toggle.

## Risks / Trade-offs

- **BREAKING ActionBar API**: all callers of `ActionBar` must update from `{mode, onToggleMode, onCreateChild, canCreateChild}` to `{actions}`. Currently there is one call site (`App.tsx`), so the blast radius is minimal.
- **`ViewDescriptor.actions` is called on every render**: the factory is a pure function with no side effects, so this is acceptable at this scale.

## Migration Plan

1. Add `ActionBarAction` to `types.ts`.
2. Refactor `ActionBar` in `@pu-erh/ui` to accept `actions: ActionBarAction[]`.
3. Add `viewRouter.ts` with `ViewDescriptor`, `ViewActionContext`, and the dispatch function.
4. Update `App.tsx` (`Workspace`) to use `viewRouter` instead of the inline conditional and to derive `ActionBar` actions from the descriptor.
5. Remove the old fixed props (`mode`, `onToggleMode`, `onCreateChild`, `canCreateChild`) from ActionBar.
