## Why

The current components in `packages/ui` are hand-rolled with no accessibility semantics — no ARIA roles, focus management, keyboard navigation, or screen-reader support. Adobe React Aria provides a battle-tested, spec-compliant accessibility layer for every interactive primitive, letting us keep our own design tokens and CSS while getting correct behavior for free. Adopting it now, before the component surface grows, avoids retrofitting accessibility onto every future component.

## What Changes

- Add `react-aria-components` as a dependency of `packages/ui`; rebuild every interactive component (`Button`, `Badge`, `Stack`, `InlineBlock`, `TreeNode`, `ViewModeToggle`, `PropertiesPanel`) on React Aria primitives. **BREAKING**: some prop names change to align with React Aria's API (e.g. `onClick` → `onPress`).
- Add `Card` and `Divider` as non-interactive wrappers (React Aria not required for pure layout/display components).
- Add `modern-normalize` to `packages/ui` as the CSS reset layer, applied before component styles.
- Update `apps/showcase` to reflect any API changes and demonstrate accessibility (keyboard navigation, focus rings).

## Capabilities

### New Capabilities

### Modified Capabilities

- `base-components`: Requirements change — components must be built on React Aria primitives, must expose React Aria-compatible prop APIs, and must pass through React Aria's accessibility attributes. Existing hand-rolled implementations are replaced.
- `design-tokens`: New requirement — `modern-normalize` reset SHALL be applied as the first layer of the stylesheet, before token declarations and component styles.

## Impact

- `packages/ui/package.json`: add `react-aria-components`, `modern-normalize` as dependencies.
- `packages/ui/src/components.tsx`: all interactive components rebuilt; prop APIs align with React Aria (e.g. `Button` gains `onPress`, `isDisabled`).
- `packages/ui/src/styles.css`: `@import "modern-normalize"` added as first rule; `pu-erh-*` class styles updated to override React Aria's own class slots where needed.
- `packages/ui/src/index.ts`: exports unchanged in name; types change.
- `apps/showcase/src/App.tsx`: prop usage updated to match new APIs.
- `apps/desktop`: any direct uses of `Button` or other interactive components must migrate `onClick` → `onPress`, `disabled` → `isDisabled`.
