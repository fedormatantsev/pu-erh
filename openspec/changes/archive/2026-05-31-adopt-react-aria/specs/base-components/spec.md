## MODIFIED Requirements

### Requirement: Button React component built on React Aria
The system SHALL export a `Button` React component from `packages/ui/src/index.ts`. It SHALL be implemented using `Button` from `react-aria-components`. It SHALL accept `children`, `onPress` (replaces `onClick`), `isDisabled` (replaces `disabled`), and `type` props. Its styles SHALL use `[data-disabled]` and `[data-focused]` attribute selectors for state styling instead of `:disabled` and `:focus` pseudo-classes.

#### Scenario: Button onPress fires on click and keyboard activation
- **WHEN** a user clicks a `<Button>` or activates it via keyboard (Enter/Space)
- **THEN** the `onPress` handler is called

#### Scenario: Disabled button is non-interactive
- **WHEN** a `<Button isDisabled>` is rendered
- **THEN** `[data-disabled]` is set on the element, `pointer-events: none` is applied, and no `onPress` fires on interaction

#### Scenario: Button shows focus ring when focused via keyboard
- **WHEN** a user focuses a `<Button>` via keyboard navigation
- **THEN** `[data-focused]` is set on the element and a visible focus indicator is applied via CSS

### Requirement: InlineBlock component built on React Aria Button
The system SHALL implement `InlineBlock` using `Button` from `react-aria-components`. It SHALL accept `label` and `onActivate` (maps to `onPress`) props. It SHALL inherit React Aria's keyboard activation and ARIA role.

#### Scenario: InlineBlock activates on Enter/Space
- **WHEN** an `<InlineBlock>` has keyboard focus and the user presses Enter or Space
- **THEN** the `onActivate` handler is called

### Requirement: Divider component built on React Aria Separator
The system SHALL implement `Divider` using `Separator` from `react-aria-components`. It SHALL render with `role="separator"` and render vertical whitespace only (no visible line). It accepts no content children.

#### Scenario: Divider has correct ARIA role
- **WHEN** a `<Divider />` is rendered
- **THEN** the rendered element has `role="separator"`

### Requirement: Badge React component exported from @pu-erh/ui
The system SHALL export a `Badge` React component from `packages/ui/src/index.ts`. It SHALL accept a `variant` prop (`"neutral" | "primary"`, defaulting to `"neutral"`) and a `children` prop. Its styles SHALL use `--radius-full`, `--text-xs`, and `--space-*` tokens exclusively.

#### Scenario: Badge renders with neutral variant
- **WHEN** `<Badge>Label</Badge>` is rendered
- **THEN** the element's `border-radius` resolves to `var(--radius-full)` and the font size resolves to `var(--text-xs)`

#### Scenario: Badge renders with primary variant
- **WHEN** `<Badge variant="primary">Label</Badge>` is rendered
- **THEN** the element's background color resolves to `var(--color-primary-500)` or a token alias of it

### Requirement: Card React component exported from @pu-erh/ui
The system SHALL export a `Card` React component from `packages/ui/src/index.ts`. It SHALL accept a `children` prop and render a contained surface using `--color-surface` background, `--radius-md` corner radius, and `--space-4` padding.

#### Scenario: Card background uses surface token
- **WHEN** a `<Card>` is rendered
- **THEN** `background-color` resolves to `var(--color-surface)`

### Requirement: All component styles reference only token values
No CSS in `styles.css` SHALL hard-code pixel values, hex colors, or raw font sizes where a design token exists. Every stylistic value SHALL reference a `var(--token-name)`.

#### Scenario: No hard-coded hex colors in component styles
- **WHEN** `styles.css` is audited
- **THEN** no color hex values appear outside `tokens.css`

## REMOVED Requirements

### Requirement: Existing Button component migrated to token vars
**Reason:** Superseded by the React Aria–based Button requirement above, which covers token usage as part of its broader specification.
**Migration:** The new Button requirement includes the token-only styling constraint; no separate migration step needed.
