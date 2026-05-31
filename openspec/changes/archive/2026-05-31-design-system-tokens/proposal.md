## Why

The project has no shared design language — colors, spacing, radii, and typography are scattered as hard-coded values across `packages/ui/src/styles.css` and individual component styles. A formal design token layer establishes a single source of truth and makes the UI coherent and maintainable from the start.

## What Changes

- Introduce a design token stylesheet (`tokens.css`) in `packages/ui/src/` defining CSS custom properties for spacing, radii, typography, and color.
- Migrate existing `packages/ui` component styles to consume those tokens; add new base components (Badge, Card, Divider) as React components in the same package.
- Add `apps/showcase` — a Bun/Vite React app that imports `@pu-erh/ui` and renders a one-pager design system reference.

## Capabilities

### New Capabilities

- `design-tokens`: CSS custom-property token definitions for spacing, radii, typography, and color, living in `packages/ui/src/tokens.css`.
- `base-components`: Minimal React components (Badge, Card, Divider) added to `packages/ui`; existing components (Button, Text, Stack, etc.) migrated to consume tokens.
- `design-showcase`: `apps/showcase` — a Vite/React app built with Bun that uses `@pu-erh/ui` as a workspace dependency and renders all tokens and components as a living style guide.
- `ui-direction`: Normative design philosophy for the UI — content-centric layout, border-free separation via space and type, three distinct font roles (grotesque display, sans body, mono technical), and progressive disclosure as the primary UX pattern.

### Modified Capabilities

## Impact

- `packages/ui/src/styles.css`: hard-coded values replaced with token vars; `tokens.css` imported at the top.
- `packages/ui/src/components.tsx`: new Badge, Card, Divider components added; existing components updated.
- `packages/ui/src/index.ts`: new components exported.
- `apps/showcase/`: new Vite/React workspace app, no existing code touched.
- `package.json` (root): `apps/showcase` covered by existing `apps/*` workspace glob — no root change needed.
