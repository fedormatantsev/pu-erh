## ADDED Requirements

### Requirement: modern-normalize CSS reset applied as first stylesheet layer
`packages/ui/src/styles.css` SHALL import `modern-normalize` as its very first rule, before the token import and before all component styles. This ensures browser default styles are normalised for all consumers of `@pu-erh/ui`.

#### Scenario: Reset applied before tokens
- **WHEN** a browser loads the stylesheet bundle from `@pu-erh/ui`
- **THEN** `modern-normalize` styles appear before any `--space-*` or component class declarations in the resolved CSS

#### Scenario: Box-sizing is border-box globally
- **WHEN** any element is rendered after importing `@pu-erh/ui`
- **THEN** `getComputedStyle(el).boxSizing` returns `"border-box"` (applied by `modern-normalize`)
