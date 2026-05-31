## Context

`packages/ui` currently exports hand-rolled React components with no ARIA semantics — focus management, keyboard navigation, screen-reader announcements, and ARIA roles are all absent. Two consumers exist: `apps/desktop` (Tauri shell) and `apps/showcase`. Desktop uses `Button`, `Stack`, `Text`, `ViewModeToggle`, `InlineBlock`, `TreeNode`, `PropertiesPanel`.

## Goals / Non-Goals

**Goals:**
- Rebuild interactive components on `react-aria-components` primitives so ARIA correctness is inherited, not hand-coded.
- Add `modern-normalize` as the CSS reset baseline so browser defaults are neutralised before our token layer.
- Preserve the `pu-erh-*` class-name API for CSS; align TypeScript prop names with React Aria conventions.

**Non-Goals:**
- Not migrating to React Aria's full design-system component set (we keep our own visual layer).
- Not introducing CSS Modules, CSS-in-JS, or Tailwind — plain CSS with token vars remains the styling approach.
- Not adopting React Aria's `useTheme`/`Provider` — our token layer via `:root` custom properties is sufficient.
- Not migrating pure layout/display components (`Stack`, `Text`, `Badge`, `Card`, `PropertiesPanel`) — they have no interactive surface and React Aria adds nothing.

## Decisions

### Package choice: `react-aria-components` (not individual `@react-aria/*` hooks)

**Decision:** Use the unified `react-aria-components` package.

**Rationale:** The component package composes the low-level hooks internally and exposes a familiar JSX API. The hooks API is more flexible but requires wiring up refs, interactions, and ARIA props manually — negating the benefit of adopting the library. Alternative: `@radix-ui/react-*` — rejected because React Aria has stronger keyboard/pointer model and better mobile touch support.

### CSS reset: `modern-normalize`

**Decision:** Import `modern-normalize` as the first rule in `styles.css`.

**Rationale:** `modern-normalize` uses modern CSS (`box-sizing`, `text-size-adjust`, font smoothing) and targets current browsers only — no legacy IE hacks. It normalises rather than hard-resets, which pairs well with a token-driven design system where we want consistent baselines, not stripped-out defaults. Alternative: `the-new-css-reset` — more aggressive (strips all margins/paddings), which would require re-adding spacing via tokens on every element. Alternative: no reset — rejected because browser default styles cause visible inconsistencies across platforms.

### Styling React Aria components: data-attribute selectors

**Decision:** Style state variants using React Aria's `data-*` attribute hooks (`[data-pressed]`, `[data-focused]`, `[data-disabled]`, `[data-hovered]`) in `styles.css` rather than separate CSS classes.

**Rationale:** React Aria sets these attributes automatically based on interaction state; using them in CSS keeps state styling co-located with component styles and removes the need to manage class toggling manually.

### Component migration scope: interactive components only

**Decision:** Migrate only `Button` and `InlineBlock` to `react-aria-components` in this change. `Divider` gets `Separator` from React Aria. `TreeNode`, `ViewModeToggle` use `Button` internally and benefit transitively.

**Rationale:** `Badge`, `Card`, `Stack`, `Text`, `PropertiesPanel` are non-interactive display wrappers — React Aria provides no benefit there and adding it would increase bundle size for zero UX gain. `TreeView` from React Aria is a heavier primitive — out of scope until tree interaction requirements are specified.

### API breaking changes: align with React Aria conventions

**Decision:** `Button` and `InlineBlock` drop the `onClick`/`disabled` props in favour of React Aria's `onPress`/`isDisabled`. Both desktop and showcase consumers must be updated.

**Rationale:** React Aria's `onPress` is semantically richer than `onClick` (handles keyboard activation, touch, and pointer correctly). Keeping both would require bridging code. Since there are only two consumers and limited usages, a clean cut is cheaper.

## Risks / Trade-offs

- **Bundle size increase** → `react-aria-components` is tree-shakeable; only imported components are bundled. Mitigation: verify bundle size at build time.
- **Breaking prop changes propagate to both apps** → desktop's `Button` usages (`disabled`, `onClick`) must be updated. Low risk: only `PropertiesView.tsx` and indirectly `ViewModeToggle`. Mitigation: TypeScript will catch all missed sites at compile time.
- **React Aria focus-visible styles may conflict with our CSS** → React Aria applies `:focus-visible` rings automatically. Mitigation: use `[data-focused]` selectors in our CSS to control focus appearance and rely on React Aria to set them correctly.

## Migration Plan

1. Add `react-aria-components` and `modern-normalize` to `packages/ui/package.json`.
2. Add `@import "modern-normalize"` as first import in `styles.css`.
3. Rebuild `Button` on `react-aria-components` `Button`; update CSS to use `[data-disabled]`, `[data-pressed]`, `[data-focused]`.
4. Rebuild `InlineBlock` on `react-aria-components` `Button`.
5. Rebuild `Divider` on `react-aria-components` `Separator`.
6. Update `TreeNode` and `ViewModeToggle` to pass `isDisabled`/`onPress` to the internal `Button`.
7. Update `apps/desktop` consumer sites (`PropertiesView.tsx` uses `Button` with `disabled`/`onClick`).
8. Update `apps/showcase` component examples.
9. Build both apps to confirm TypeScript correctness.
