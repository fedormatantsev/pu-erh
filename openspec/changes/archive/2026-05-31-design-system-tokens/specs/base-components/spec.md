## ADDED Requirements

### Requirement: Badge React component exported from @pu-erh/ui
The system SHALL export a `Badge` React component from `packages/ui/src/index.ts`. It SHALL accept a `variant` prop (`"neutral" | "primary"`, defaulting to `"neutral"`) and a `children` prop. Its styles SHALL use `--radius-full`, `--text-xs`, and `--space-*` tokens exclusively.

#### Scenario: Badge renders with neutral variant
- **WHEN** `<Badge>Label</Badge>` is rendered
- **THEN** the element's `border-radius` resolves to `var(--radius-full)` and the font size resolves to `var(--text-xs)`

#### Scenario: Badge renders with primary variant
- **WHEN** `<Badge variant="primary">Label</Badge>` is rendered
- **THEN** the element's background color resolves to `var(--color-primary-500)` or a token alias of it

### Requirement: Card React component exported from @pu-erh/ui
The system SHALL export a `Card` React component from `packages/ui/src/index.ts`. It SHALL accept a `children` prop and render a contained surface using `--color-surface` background, `--color-border` border, `--radius-md` corner radius, and `--space-4` padding.

#### Scenario: Card background uses surface token
- **WHEN** a `<Card>` is rendered
- **THEN** `background-color` resolves to `var(--color-surface)`

#### Scenario: Card border uses border token
- **WHEN** a `<Card>` is rendered
- **THEN** the element has a border whose color resolves to `var(--color-border)`

### Requirement: Divider React component exported from @pu-erh/ui
The system SHALL export a `Divider` React component from `packages/ui/src/index.ts`. It SHALL render a horizontal separator (`<hr>` or equivalent) using `--color-border` for color and `--space-4` tokens for vertical margin. It accepts no content children.

#### Scenario: Divider uses border color token
- **WHEN** a `<Divider />` is rendered
- **THEN** its border or background color resolves to `var(--color-border)`

### Requirement: Existing Button component migrated to token vars
The existing `Button` component's CSS class `.pu-erh-button` in `styles.css` SHALL be updated so that all padding, border color, and font values reference design tokens. No hard-coded values SHALL remain.

#### Scenario: Button padding uses spacing tokens
- **WHEN** a `<Button>` is rendered
- **THEN** its `padding` resolves via `var(--space-*)` tokens

### Requirement: All component styles reference only token values
No CSS in `styles.css` SHALL hard-code pixel values, hex colors, or raw font sizes where a design token exists. Every stylistic value SHALL reference a `var(--token-name)`.

#### Scenario: No hard-coded hex colors in component styles
- **WHEN** `styles.css` is audited
- **THEN** no color hex values appear outside `tokens.css`
