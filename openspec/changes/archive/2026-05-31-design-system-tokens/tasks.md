## 1. Design Tokens (packages/ui)

- [x] 1.1 Create `packages/ui/src/tokens.css` with `:root` block and full spacing scale (`--space-1` through `--space-16`, 4 px base unit)
- [x] 1.2 Add border radius tokens to `tokens.css`: `--radius-sm`, `--radius-md`, `--radius-lg`, `--radius-full`
- [x] 1.3 Add typography size tokens: `--text-xs` through `--text-2xl`; line-height tokens: `--leading-tight`, `--leading-base`, `--leading-loose`; weight tokens: `--weight-normal` (400), `--weight-medium` (500), `--weight-semibold` (600)
- [x] 1.4 Add three font-role tokens: `--font-display` (grotesque sans, e.g. `"Geist", "Inter", ui-sans-serif, sans-serif`), `--font-sans` (humanist/neutral sans for body, e.g. `ui-sans-serif, system-ui, sans-serif`), `--font-mono` (e.g. `"Geist Mono", ui-monospace, monospace`)
- [x] 1.5 Add neutral color scale to `tokens.css`: `--color-neutral-0` through `--color-neutral-900` (steps: 0, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900)
- [x] 1.6 Add primary accent tokens: `--color-primary-400`, `--color-primary-500`, `--color-primary-600`
- [x] 1.7 Add semantic color aliases: `--color-bg`, `--color-surface`, `--color-border`, `--color-text`, `--color-text-muted`
- [x] 1.8 Add `@import "./tokens.css";` at the top of `packages/ui/src/styles.css`

## 2. Migrate Existing Component Styles to Tokens

- [x] 2.1 Replace hard-coded padding/border values in `.pu-erh-button` with spacing tokens; set `font-family: var(--font-sans)` and `font-size: var(--text-sm)`
- [x] 2.2 Replace hard-coded values in `.pu-erh-text` with `--font-sans`, `--text-base`, `--leading-base`
- [x] 2.3 Replace hard-coded values in `.pu-erh-stack`, `.pu-erh-inline-block`, `.pu-erh-tree-*`, `.pu-erh-view-mode-toggle`, `.pu-erh-properties-panel` with token vars
- [x] 2.4 Verify no hex colors or raw pixel sizes remain in `styles.css` outside `tokens.css`

## 3. New Base Components (packages/ui)

- [x] 3.1 Add `Badge` React component to `components.tsx` with `variant?: "neutral" | "primary"` prop; use `--radius-full`, `--text-xs`, `--font-mono` (Badge text is a label/technical value), `--space-1`/`--space-2` padding
- [x] 3.2 Add `Card` React component to `components.tsx`; use `--color-surface` background, no border (use padding `--space-4` and surrounding space for separation per ui-direction), `--radius-md`
- [x] 3.3 Add `Divider` React component to `components.tsx`; renders vertical whitespace only (no visible line) — a `<div>` with `margin: var(--space-4) 0` and no border or background
- [x] 3.4 Export `Badge`, `Card`, `Divider` from `packages/ui/src/index.ts`

## 4. Showcase App Scaffold (apps/showcase)

- [x] 4.1 Create `apps/showcase/package.json` as `@pu-erh/showcase` (private) with `@pu-erh/ui: workspace:*`, `react`, `react-dom` as deps; `@vitejs/plugin-react`, `vite`, TypeScript, `@types/react`, `@types/react-dom` as devDeps; add `dev` and `build` scripts
- [x] 4.2 Create `apps/showcase/vite.config.ts` using `@vitejs/plugin-react`
- [x] 4.3 Create `apps/showcase/tsconfig.json` and `apps/showcase/tsconfig.app.json` (mirror `apps/desktop` config)
- [x] 4.4 Create `apps/showcase/index.html` with `<title>Design System — pu-erh</title>` and `<script type="module" src="/src/main.tsx">`
- [x] 4.5 Create `apps/showcase/src/main.tsx` that mounts `<App />` into `#root`
- [x] 4.6 Run `bun install` from the repo root to link the new workspace app

## 5. Showcase Page Content (apps/showcase)

- [x] 5.1 Create `apps/showcase/src/App.tsx` with a content-centric layout: max-width prose container, `--font-display` heading, sections separated by `--space-*` gaps — no decorative borders or sidebars
- [x] 5.2 Add "Spacing" section: one row per `--space-*` token — token name in `--font-mono`, a bar sized to that value, resolved px shown in `--font-mono`; rows separated by space only
- [x] 5.3 Add "Colors" section: swatch grid for neutral scale, primary accent, and semantic aliases — token name in `--font-mono`, no card borders, swatches flush in a tight grid
- [x] 5.4 Add "Typography" section: for each `--text-*` token render a live text sample — token name label in `--font-mono --text-xs`, sample text at that size using appropriate font role (display for heading sizes, sans for body sizes)
- [x] 5.5 Add "Radii" section: one box per radius token with that `border-radius` applied, token name in `--font-mono`
- [x] 5.6 Add "Components" section using progressive disclosure: each component group (Button, Badge, Card, Divider, Text, Stack) is collapsed by default, expanded on click — show component name and brief description in summary; live rendered examples in the expanded detail
- [x] 5.7 Run `bun run dev` in `apps/showcase/` and verify all sections render correctly, progressive disclosure works, and no decorative borders appear between sections
