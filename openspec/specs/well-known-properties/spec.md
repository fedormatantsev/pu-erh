# well-known-properties Specification

## Purpose
TBD - created by syncing change display-property-selector. Update Purpose after archive.
## Requirements
### Requirement: Well-known properties have dedicated layout slots

The Properties View layout MUST reserve dedicated, fixed-position UI slots for well-known predefined properties. A well-known property MUST NOT appear as an item in the generic properties list — its dedicated slot is its only representation. `display` MUST be the first well-known property with a dedicated slot.

#### Scenario: display rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `display` property is rendered in its dedicated layout slot (before the generic properties list)
- **AND** `display` does not appear as an item in the generic properties list

#### Scenario: Generic list excludes well-known properties

- **WHEN** the Properties View renders the generic properties list
- **THEN** any property key that has a dedicated layout slot is excluded from that list
- **AND** only properties without dedicated slots appear in the generic list

### Requirement: display slot renders as a dropdown with no label

The dedicated layout slot for `display` MUST render a dropdown (select element) populated with the registered block view names available in the app. No property title label MUST be shown — the slot contains only the dropdown control.

#### Scenario: Display slot shows dropdown only

- **WHEN** the Properties View renders the display slot
- **THEN** a dropdown is shown with all registered view names as options
- **AND** no title label is rendered alongside the dropdown
