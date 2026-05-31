# design-showcase Specification

## Purpose
Design system reference app (`apps/showcase`) for visualizing tokens and live `@pu-erh/ui` components.
## Requirements
### Requirement: apps/showcase is a Bun/Vite React workspace app
The system SHALL provide `apps/showcase/` as a Bun workspace app with:
- `package.json` named `@pu-erh/showcase`, marked private, with `"@pu-erh/ui": "workspace:*"` as a dependency and `react`, `react-dom` as dependencies.
- A `vite.config.ts` using `@vitejs/plugin-react`.
- A `tsconfig.json` and `tsconfig.app.json` consistent with the desktop app setup.
- An entry `index.html` pointing to `src/main.tsx`.

#### Scenario: Showcase builds with Bun
- **WHEN** `bun run build` is executed in `apps/showcase/`
- **THEN** the build completes without errors and produces output in `dist/`

#### Scenario: Showcase dev server starts with Bun
- **WHEN** `bun run dev` is executed in `apps/showcase/`
- **THEN** Vite starts and serves the showcase at `localhost:5173` (or next available port)

### Requirement: Showcase imports and renders @pu-erh/ui components directly
The showcase SHALL import React components from `@pu-erh/ui` and render them as live React elements — not static HTML markup. All displayed component examples SHALL be the actual exported components.

#### Scenario: Badge imported from @pu-erh/ui renders in showcase
- **WHEN** the showcase is loaded
- **THEN** the Badge section renders a `<Badge>` element that is the live `Badge` component from `@pu-erh/ui`

### Requirement: Showcase displays the full spacing scale
The system SHALL render a visual row for each spacing token (`--space-1` through `--space-16`) showing the token name, its pixel value, and a colored bar sized to that spacing value.

#### Scenario: Spacing scale section is present
- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Spacing" or equivalent contains one row per spacing token with a visible bar

### Requirement: Showcase displays the full color palette
The system SHALL render a swatch grid showing all color tokens grouped by category (neutral, primary, semantic aliases). Each swatch SHALL display the token name.

#### Scenario: Color palette section is present
- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Colors" or equivalent contains one swatch per color token with its name visible

### Requirement: Showcase displays the typography scale
The system SHALL render each `--text-*` token as a live text sample at that font size, labeled with the token name.

#### Scenario: Typography section is present
- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Typography" or equivalent contains one sample per `--text-*` token

### Requirement: Showcase displays the border radius scale
The system SHALL render a swatch for each radius token showing a box with that border radius applied, labeled with the token name.

#### Scenario: Radius section is present
- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Radii" or equivalent contains one swatch per radius token

### Requirement: Showcase displays all base components

The system SHALL render live `@pu-erh/ui` React component examples covering: Button (primary, secondary, disabled), Badge (neutral, primary), Card with sample content, Divider, Text (heading levels and body), Stack, and the TreeView column component(s) shown with static sample data (a parent, a current block with siblings, and children) and a distinguished current block.

#### Scenario: Components section is present

- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Components" or equivalent contains a live rendered example of each exported component

#### Scenario: TreeView column example is present

- **WHEN** the showcase page is loaded
- **THEN** the Components section contains a live rendered example of the TreeView column component(s) using static sample data
- **AND** the example renders the three columns (parent, current block with siblings, children) with the current block distinguished

### Requirement: Showcase has a title and section headings
The page SHALL have a `<title>` and visible `<h1>` identifying it as the design system reference. Each token category and the components section SHALL have a visible section heading.

#### Scenario: Page has a title and section headings
- **WHEN** the showcase page is loaded
- **THEN** the `<title>` and visible heading identify the page, and each section has a heading

