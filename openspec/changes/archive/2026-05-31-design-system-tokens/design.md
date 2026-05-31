## Context

`packages/ui` is an existing Bun workspace package (`@pu-erh/ui`) that exports React components consumed by `apps/desktop` (Tauri/Vite). Its `styles.css` currently uses hard-coded values. The root workspace already covers `apps/*` so a new `apps/showcase` Vite app slots in without touching root config.

## Goals / Non-Goals

**Goals:**
- Define CSS custom-property tokens in `packages/ui/src/tokens.css` (spacing, radii, typography, color).
- Migrate existing component styles in `packages/ui` to consume those tokens; add Badge, Card, Divider.
- Ship `apps/showcase` — a Bun/Vite React app that imports `@pu-erh/ui` and renders a one-page design-system reference.

**Non-Goals:**
- No dark/light mode toggle in v0.
- No design tooling export (Figma, Storybook, Style Dictionary).
- No changes to `apps/desktop` behavior — it already consumes `@pu-erh/ui` and will pick up token-migrated styles automatically.
- No JS-in-CSS or build-time token generation — tokens are plain CSS custom properties.

## Decisions

### Token location: `packages/ui/src/tokens.css`

**Decision:** Tokens live in `packages/ui/src/tokens.css`, imported at the top of `styles.css`.

**Rationale:** Keeps tokens co-located with the components that consume them and makes the single import (`import "@pu-erh/ui"`) bring in tokens automatically for any consumer. Alternative (separate npm package) adds indirection with no benefit at this scale.

### Token naming: `--{category}-{scale}`

**Decision:** `--space-{n}` (4 px base grid), `--radius-{label}`, `--text-{size}`, `--leading-{label}`, `--font-{role}`, `--color-{role}-{shade}`.

**Rationale:** Category-first grouping aids DevTools autocomplete. Numeric scale follows 4 px base (aligns with Tailwind/Material). Semantic aliases (`--color-bg`, `--color-text`) decouple components from raw palette values.

### Three distinct font roles

**Decision:** Three `--font-*` tokens with distinct semantic roles:
- `--font-display`: grotesque sans (neo-grotesque stack — e.g. Geist, Inter) — used for all headings and display text.
- `--font-sans`: humanist or neutral sans-serif — used for all body copy and prose.
- `--font-mono`: monospace — used for technical values, code, and UI labels (token names, keyboard shortcuts, IDs).

**Rationale:** The grotesque/display distinction makes headings feel intentional and structured without requiring a decorative typeface. Mono for labels creates a visual grammar that distinguishes data from prose without color or borders. Using three clearly named tokens prevents font-role drift as the codebase grows.

### UI philosophy: content-centric, border-free, progressive disclosure

**Decision:** These three principles are normative and captured in the `ui-direction` spec:
1. **Content-centric**: the content area is the page; chrome is invisible.
2. **No decorative borders**: visual separation comes from spacing and type weight, not lines. Borders are reserved for interactive elements that require a clear affordance (e.g. input fields).
3. **Progressive disclosure**: complex or secondary information starts hidden; revealed on explicit user action (expand, hover, click). Screens MUST NOT show everything at once.

**Rationale:** Borders and chrome compete with content for attention. Spacing hierarchy is more expressive and scales better across density levels. Progressive disclosure keeps initial views clean and surfaces depth only for users who need it — especially critical in a knowledge-base tool where information density is high.

### Component format: React TSX with CSS class names

**Decision:** New components (Badge, Card, Divider) follow the existing pattern in `components.tsx` — typed React function components with `pu-erh-*` CSS class names defined in `styles.css` using token vars.

**Rationale:** Consistency with the existing codebase. No CSS Modules or runtime-CSS library needed; plain class names keep the bundle minimal.

### Showcase: `apps/showcase` Vite/React app

**Decision:** New app at `apps/showcase/` with its own `package.json` (`@pu-erh/showcase`, private), `"@pu-erh/ui": "workspace:*"` as a dependency, and a standard `@vitejs/plugin-react` Vite config.

**Rationale:** Mirrors the pattern of `apps/desktop`. Bun workspace resolution means `@pu-erh/ui` is linked automatically. The showcase renders live React components — not static HTML — so it can import and mount the actual exported components directly.

**Alternative considered:** Extend `apps/desktop` with a `/showcase` route. Rejected because it couples the design reference to the product shell and adds Tauri overhead to a pure UI concern.

## Risks / Trade-offs

- **Existing consumers pick up style changes immediately** — `apps/desktop` will get migrated styles on next build. Visual regression is low-risk (values are equivalent; only the source changes from literals to vars), but the showcase provides a visual reference to compare against.
- **Token naming will evolve** → Mitigation: semantic aliases mean internal palette changes are localized to `tokens.css`.
- **No automated visual regression** → Accepted; the showcase page is the visual contract.
