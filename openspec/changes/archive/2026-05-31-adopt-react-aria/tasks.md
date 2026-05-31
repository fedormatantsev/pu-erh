## 1. Dependencies

- [x] 1.1 Add `react-aria-components` and `modern-normalize` to `packages/ui/package.json` dependencies
- [x] 1.2 Run `bun install` from repo root to resolve the new packages

## 2. CSS Reset

- [x] 2.1 Add `@import "modern-normalize"` as the very first line of `packages/ui/src/styles.css`, before `@import "./tokens.css"`
- [x] 2.2 Verify `box-sizing: border-box` is globally applied after the import (inspect in browser or build output)

## 3. Button Component

- [x] 3.1 Replace the `Button` implementation in `components.tsx` with `Button` from `react-aria-components`; accept `children`, `onPress`, `isDisabled`, `type` props
- [x] 3.2 Update `.pu-erh-button` CSS in `styles.css`: replace `:disabled` pseudo-class with `[data-disabled]`, add `[data-focused]` for focus ring, add `[data-pressed]` for active state

## 4. InlineBlock Component

- [x] 4.1 Replace `InlineBlock` implementation with `Button` from `react-aria-components`; map `onActivate` → `onPress`
- [x] 4.2 Update `.pu-erh-inline-block` CSS to use `[data-disabled]` and `[data-focused]` selectors

## 5. Divider Component

- [x] 5.1 Replace `Divider` implementation with `Separator` from `react-aria-components`; keep whitespace-only styling (no visible line)
- [x] 5.2 Verify rendered element has `role="separator"` in the DOM

## 6. TreeNode and ViewModeToggle

- [x] 6.1 Update `TreeNode` to pass `isDisabled` (instead of `disabled`) and `onPress` (instead of `onClick`) to its internal `Button` component
- [x] 6.2 Update `ViewModeToggle` to pass `isDisabled` and `onPress` to its internal `Button` calls

## 7. Consumer Migration — apps/desktop

- [x] 7.1 Update `apps/desktop/src/views/PropertiesView.tsx`: replace `disabled` → `isDisabled` and `onClick` → `onPress` on any `Button` usages
- [x] 7.2 Scan all other desktop source files for any remaining `onClick`/`disabled` props passed to `@pu-erh/ui` components and update them

## 8. Consumer Migration — apps/showcase

- [x] 8.1 Update `apps/showcase/src/App.tsx`: replace `onClick`/`disabled` → `onPress`/`isDisabled` in the Button examples
- [x] 8.2 Update showcase Button section description to reflect `onPress`/`isDisabled` API

## 9. Verification

- [x] 9.1 Run `bun run --filter @pu-erh/showcase build` — must exit 0 with no TypeScript errors
- [x] 9.2 Start `apps/showcase` dev server and verify: Button focus ring appears on keyboard navigation, `[data-disabled]` button is visually distinct and non-interactive, Divider has `role="separator"` in DevTools
- [x] 9.3 Run `bun run --filter @pu-erh/desktop build` — must exit 0 (or confirm only pre-existing errors remain)
