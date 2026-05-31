## ADDED Requirements

### Requirement: Content occupies the primary visual field
The UI SHALL be content-centric: the content area fills the available width and visual weight. Structural chrome (navigation, toolbars, sidebars) SHALL be visually subordinate — low contrast, compact, and absent when not needed. No element of chrome SHALL compete with content for the user's primary attention.

#### Scenario: Content area is dominant
- **WHEN** a page is rendered with content present
- **THEN** the content region occupies the majority of the visible area and carries the highest visual weight on the page

#### Scenario: Chrome is subordinate
- **WHEN** a page is rendered
- **THEN** navigation and toolbar elements use lower contrast and smaller type than content, and do not have decorative backgrounds or prominent borders

### Requirement: Visual hierarchy is built from spacing and typography, not borders
The UI SHALL use spacing scale (margin, padding, gap) and typography scale (font size, weight, line height) as the primary tools for creating visual hierarchy and grouping. Decorative borders and dividing lines between content regions SHALL NOT be used. A border is permissible only where it provides a direct interactive affordance (e.g., a focused input field).

#### Scenario: Content sections are separated by space, not lines
- **WHEN** two adjacent content sections are rendered
- **THEN** they are visually separated by a spacing-scale gap with no `border` or `<hr>` between them

#### Scenario: Interactive inputs may use borders
- **WHEN** an input field is rendered
- **THEN** it MAY use a border to communicate its interactive affordance; this is the sole permitted exception to the no-border rule

### Requirement: Grotesque font used for all display text and headings
All heading elements (h1–h4) and display-scale text SHALL use `var(--font-display)`, which SHALL be a neo-grotesque or grotesque sans-serif typeface. Headings SHALL additionally use `var(--weight-semibold)` or heavier.

#### Scenario: h1 uses display font
- **WHEN** an h1 element is rendered using the design system
- **THEN** its `font-family` resolves to `var(--font-display)` and its `font-weight` is at least 600

#### Scenario: h2–h4 use display font
- **WHEN** h2, h3, or h4 elements are rendered using the design system
- **THEN** their `font-family` resolves to `var(--font-display)`

### Requirement: Sans-serif font used for body copy
All body prose and paragraph text SHALL use `var(--font-sans)`. Line height for body text SHALL use `var(--leading-base)` or `var(--leading-loose)` to maintain readability.

#### Scenario: Body text uses sans font
- **WHEN** a body paragraph is rendered using the design system
- **THEN** its `font-family` resolves to `var(--font-sans)` and its `line-height` resolves to `var(--leading-base)` or `var(--leading-loose)`

### Requirement: Mono font used for technical content and UI labels
Monospace text (`var(--font-mono)`) SHALL be used for: code snippets, UUIDs and other IDs, keyboard shortcut indicators, token names, version strings, timestamps, and UI labels that represent technical or machine-readable values.

#### Scenario: Token name label uses mono font
- **WHEN** a UI label displays a token name (e.g. `--space-4`) or a UUID
- **THEN** its `font-family` resolves to `var(--font-mono)`

#### Scenario: Regular UI labels use body font
- **WHEN** a UI label displays a human-readable description (e.g. "Created at", "Title")
- **THEN** its `font-family` resolves to `var(--font-sans)`, not `var(--font-mono)`

### Requirement: Progressive disclosure is the default UX pattern for complex information
The UI SHALL default to showing a minimal, scannable summary of any complex or multi-part piece of information. Full detail SHALL be revealed progressively — only on explicit user action (expand, click, hover, focus). Screens and panels SHALL NOT surface all available information simultaneously.

#### Scenario: Expandable detail is collapsed by default
- **WHEN** a component contains secondary or detailed information
- **THEN** that information is hidden by default and a clear affordance (e.g. expand button, chevron, "show more") is present to reveal it

#### Scenario: Revealed content is dismissible
- **WHEN** a user has expanded a detail section
- **THEN** a mechanism exists to collapse it back to the summary state

### Requirement: Screens avoid information overload
No screen or panel SHALL display more than the information required for the user's current task. Supplementary information (metadata, related items, secondary actions) SHALL be accessible but not visible by default. This applies especially to list views, detail panels, and forms.

#### Scenario: List view shows minimal data per item
- **WHEN** a list of items is rendered
- **THEN** each item shows only the primary identifier and status; secondary attributes are hidden until the item is focused or expanded
