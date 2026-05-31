# design-tokens Specification

## Purpose
CSS custom property design tokens and stylesheet layering for `@pu-erh/ui`.

## Requirements

### Requirement: modern-normalize CSS reset applied as first stylesheet layer
`packages/ui/src/styles.css` SHALL import `modern-normalize` as its very first rule, before the token import and before all component styles. This ensures browser default styles are normalised for all consumers of `@pu-erh/ui`.

#### Scenario: Reset applied before tokens
- **WHEN** a browser loads the stylesheet bundle from `@pu-erh/ui`
- **THEN** `modern-normalize` styles appear before any `--space-*` or component class declarations in the resolved CSS

#### Scenario: Box-sizing is border-box globally
- **WHEN** any element is rendered after importing `@pu-erh/ui`
- **THEN** `getComputedStyle(el).boxSizing` returns `"border-box"` (applied by `modern-normalize`)

### Requirement: Spacing scale defined as CSS custom properties in packages/ui
The system SHALL define a spacing scale on `:root` in `packages/ui/src/tokens.css` using CSS custom properties with a 4 px base unit: `--space-1` (4 px) through `--space-16` (64 px), covering at minimum steps 1, 2, 3, 4, 6, 8, 10, 12, 16.

#### Scenario: Spacing tokens present after importing @pu-erh/ui
- **WHEN** a React app imports `@pu-erh/ui`
- **THEN** `getComputedStyle(document.documentElement).getPropertyValue('--space-4')` returns `16px`

### Requirement: Border radius scale defined as CSS custom properties
The system SHALL define border radius tokens on `:root` in `tokens.css`: `--radius-sm`, `--radius-md`, `--radius-lg`, `--radius-full`.

#### Scenario: Radius tokens present after import
- **WHEN** a React app imports `@pu-erh/ui`
- **THEN** `--radius-sm`, `--radius-md`, `--radius-lg`, and `--radius-full` are all defined and non-empty

### Requirement: Typography scale defined as CSS custom properties
The system SHALL define in `tokens.css` on `:root`:
- Font-size tokens: `--text-xs`, `--text-sm`, `--text-base`, `--text-lg`, `--text-xl`, `--text-2xl`.
- Font-stack tokens with three distinct semantic roles: `--font-display` (grotesque sans, for headings and display text), `--font-sans` (neutral/humanist sans-serif, for body copy), `--font-mono` (monospace, for technical values, code, and UI labels).
- Font-weight tokens: `--weight-normal` (400), `--weight-medium` (500), `--weight-semibold` (600).
- Line-height tokens: `--leading-tight`, `--leading-base`, `--leading-loose`.

#### Scenario: All three font-role tokens present after import
- **WHEN** a React app imports `@pu-erh/ui`
- **THEN** `--font-display`, `--font-sans`, and `--font-mono` are all defined and non-empty

#### Scenario: Typography size tokens present after import
- **WHEN** a React app imports `@pu-erh/ui`
- **THEN** `--text-base` and `--leading-base` are defined and non-empty

### Requirement: Color palette defined as CSS custom properties
The system SHALL define in `tokens.css` on `:root`:
- Neutral scale: `--color-neutral-0` through `--color-neutral-900` (steps: 0, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900).
- Primary accent: `--color-primary-400`, `--color-primary-500`, `--color-primary-600`.
- Semantic aliases: `--color-bg`, `--color-surface`, `--color-border`, `--color-text`, `--color-text-muted`.

#### Scenario: Semantic aliases resolve after import
- **WHEN** a React app imports `@pu-erh/ui`
- **THEN** `--color-bg`, `--color-text`, and `--color-border` are all defined and non-empty

### Requirement: tokens.css is imported by styles.css
`packages/ui/src/styles.css` SHALL import `./tokens.css` at the top so that any consumer of `@pu-erh/ui` receives both tokens and component styles through the existing single import.

#### Scenario: Single import delivers tokens
- **WHEN** `components.tsx` imports `./styles.css` (as it already does)
- **THEN** all `--space-*`, `--radius-*`, `--text-*`, and `--color-*` custom properties are available in the page

### Requirement: Existing component styles migrated to token vars
All hard-coded pixel, color, and font values in `packages/ui/src/styles.css` SHALL be replaced with the corresponding token `var(--token-name)`. No stylistic value in `styles.css` SHALL hard-code a hex color, raw pixel size, or font family string where a token exists.

#### Scenario: No hard-coded colors remain in styles.css
- **WHEN** `styles.css` is audited
- **THEN** no hex color literals appear outside `tokens.css`
