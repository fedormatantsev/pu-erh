## MODIFIED Requirements

### Requirement: Properties View

The Properties View MUST contain the settings of the current Block View, including the current selected block's `display` property. The Properties View MUST be the surface through which `display` is changed. The `display` property MUST be rendered in a dedicated, fixed-position layout slot (before the generic properties list) as a constrained one-of selector (dropdown) populated with the registered block view names available in the app; it MUST NOT appear as an item in the generic properties list and MUST NOT be presented as a free-text input. No property title label MUST be shown for `display` — only the selector control is rendered in its slot. When the block has no `display` property or its stored value is not a recognized view name, the selector MUST implicitly resolve to the default view and MUST overwrite the stored value with the resolved default on the next save.

#### Scenario: Properties View exposes display as a dropdown

- **WHEN** the Properties View is shown for the current selected block
- **THEN** it presents the block's `display` property as a dropdown populated with registered view names
- **AND** no free-text input is shown for `display`
- **AND** no property title label is shown for `display`

#### Scenario: Absent display resolves to default in selector

- **WHEN** the Properties View loads a block that has no `display` property
- **THEN** the selector shows the default view as the selected value
- **AND** no write to storage occurs at load time

#### Scenario: Unrecognized display resolves to default in selector

- **WHEN** the Properties View loads a block whose `display` value is not a registered view name
- **THEN** the selector shows the default view as the selected value
- **AND** no error or warning is surfaced to the user

#### Scenario: Absent or unrecognized display overwritten on save

- **WHEN** a block's `display` was absent or unrecognized at load time
- **AND** the user saves
- **THEN** the resolved default value is written to the block's `display` property before persisting
- **AND** the stored value after save is the default view name
