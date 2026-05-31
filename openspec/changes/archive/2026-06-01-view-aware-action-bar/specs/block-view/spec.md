## ADDED Requirements

### Requirement: View Router

The application MUST dispatch the active view through a named View Router function with the signature `(blockId: string, mode: ViewMode) → ViewDescriptor`. The View Router MUST be the single authoritative place where `ViewMode` values are mapped to concrete view components and their associated action descriptor factories. The inline conditional branch (`viewMode === "block" ? … : …`) in the application host MUST be replaced by a call to the View Router.

A `ViewDescriptor` MUST carry:
- `View`: the React component that renders the active view for the given block.
- `actions`: a factory function `(ctx: ViewActionContext) → ActionBarAction[]` that returns the action descriptors for the active view, given the shell callbacks and derived flags in `ctx`.

`ViewActionContext` MUST carry only the callbacks and derived flags needed by view action factories: at minimum `setViewMode`, `createChild`, and `canCreateChild`.

#### Scenario: ViewMode maps to Tree View component and actions

- **WHEN** the View Router is called with `mode === "block"`
- **THEN** it returns a `ViewDescriptor` whose `View` is the Tree View component
- **AND** whose `actions` factory returns toggle-to-properties and create-child descriptors

#### Scenario: ViewMode maps to Properties View component and actions

- **WHEN** the View Router is called with `mode === "properties"`
- **THEN** it returns a `ViewDescriptor` whose `View` is the Properties View component
- **AND** whose `actions` factory returns only the toggle-to-block-view descriptor

#### Scenario: View Router is the single dispatch point

- **WHEN** the application host renders the active view
- **THEN** it calls the View Router to obtain both the view component and the action list
- **AND** does not contain a separate inline conditional that branches on view mode
