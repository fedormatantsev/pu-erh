# frontend-scaffold Specification

## Purpose

Defines the Bun/TypeScript/React frontend monorepo scaffold: workspace layout, `@pu-erh/ui` design-system package, `apps/desktop` app package, and bare shell UI constraints.
## Requirements
### Requirement: Bun workspace root

The repository MUST define a root `package.json` with Bun workspaces that include `packages/*` and `apps/*`.

#### Scenario: Install workspace dependencies

- **WHEN** a developer runs `bun install` at the repository root
- **THEN** dependencies for `packages/ui` and `apps/desktop` are installed and linked

### Requirement: Design system package location

Reusable design-system components MUST live in `packages/ui` and MUST NOT import application-specific code from `apps/desktop`.

#### Scenario: Package boundary

- **WHEN** a component is added for shared visual primitives (e.g. text, stack, button)
- **THEN** its source file resides under `packages/ui`
- **AND** the package exports it via a public entry point (e.g. `packages/ui/src/index.ts`)

### Requirement: Desktop app package location

Application-specific frontend code MUST live in `apps/desktop`, including the Vite configuration, React entry, and Tauri frontend integration.

#### Scenario: App consumes design system

- **WHEN** the desktop app renders UI
- **THEN** it imports presentational components from `@pu-erh/ui` (or the configured package name for `packages/ui`)
- **AND** session/IPC calls are defined in `apps/desktop`, not in `packages/ui`

### Requirement: TypeScript and React toolchain

The frontend scaffold MUST use TypeScript and React with Vite as the bundler for `apps/desktop`.

#### Scenario: Typecheck and dev build

- **WHEN** a developer runs the documented frontend dev or build script for `apps/desktop`
- **THEN** TypeScript sources compile and Vite serves or emits the React application without manual bundler configuration outside the scaffold

### Requirement: Tauri frontend invoke wiring

The `apps/desktop` frontend MUST invoke at least the `ping` Tauri command on load (or via an explicit scaffold control) to verify IPC connectivity.

#### Scenario: Ping displayed in UI

- **WHEN** the React app mounts successfully
- **THEN** the UI displays the string returned from the `ping` invoke command
- **OR** displays the invoke error message without rewriting it

### Requirement: No product UX in design system

The design system package MUST contain only presentational building blocks. It MUST NOT encode navigation metaphors, session state, save behavior, or knowledge-base domain types.

#### Scenario: UI package stays presentational

- **WHEN** a new export is added to `packages/ui`
- **THEN** it MUST NOT call Tauri APIs or assume a current block, open file, or selection model

### Requirement: Neutral empty shell in app

The desktop app's main surface MUST follow the `block-view` capability: it renders the current selected block through the Block View. The app MUST NOT include tutorial text, sample data, or marketing welcome flows. Before user data is available through a specified workflow, the app MUST show only bare scaffold content or the Block View's specified empty/raw rendering, without calls to action beyond IPC wiring proof.

#### Scenario: Main surface follows block-view

- **WHEN** the app renders its main surface
- **THEN** it renders the current selected block through the Block View as defined by `block-view`
- **AND** it does not include tutorial text, sample data, or marketing welcome flows

#### Scenario: Initial render without user data

- **WHEN** no user data has been loaded through a specified workflow
- **THEN** the app shows bare scaffold content or the Block View's specified raw rendering without calls to action beyond IPC wiring proof

