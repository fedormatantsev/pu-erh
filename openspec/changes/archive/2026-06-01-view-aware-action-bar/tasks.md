## 1. Types and ActionBar API

- [x] 1.1 Add `ActionBarAction` type to `apps/desktop/src/types.ts` (id, label, onPress, isDisabled?, pressed?)
- [x] 1.2 Refactor `ActionBar` in `packages/ui/src/components.tsx` to accept `actions: ActionBarAction[]` and render one button per entry (use `pressed` for `aria-pressed`, `isDisabled` for disabled state)
- [x] 1.3 Remove the old fixed props (`mode`, `onToggleMode`, `onCreateChild`, `canCreateChild`) from `ActionBar`

## 2. View Router

- [x] 2.1 Create `apps/desktop/src/viewRouter.ts` — define `ViewActionContext`, `ViewDescriptor`, and `viewRouter(blockId, mode)` function
- [x] 2.2 Implement the `block` branch: `View = DefaultBlockView`, actions = [toggle-to-properties (pressed when mode is `block`), create-child (isDisabled when no blockId)]
- [x] 2.3 Implement the `properties` branch: `View = PropertiesView`, actions = [toggle-to-block-view (pressed when mode is `properties`)]

## 3. Workspace wiring

- [x] 3.1 Update `Workspace` in `apps/desktop/src/App.tsx` to call `viewRouter(currentBlockId, viewMode)` and obtain `{ View, actions }`
- [x] 3.2 Replace the inline `viewMode === "block" ? … : …` conditional with `<View blockId={currentBlockId} />`
- [x] 3.3 Pass `descriptor.actions(ctx)` to `<ActionBar actions={…} />` — remove the old `onToggleMode`, `onCreateChild`, `canCreateChild` props from the call site
