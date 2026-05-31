# well-known-properties Specification

## Purpose
TBD - created by syncing change display-property-selector. Update Purpose after archive.
## Requirements
### Requirement: Well-known properties have dedicated layout slots

The Properties View layout MUST reserve dedicated, fixed-position UI slots for well-known predefined properties. A well-known property MUST NOT appear as an item in the generic properties list — its dedicated slot is its only representation. `title` MUST be the first well-known property slot, followed by `display` as the second.

#### Scenario: title rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `title` property is rendered in its dedicated layout slot (before the `display` slot and before the generic properties list)
- **AND** `title` does not appear as an item in the generic properties list

#### Scenario: display rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `display` property is rendered in its dedicated layout slot (after the `title` slot, before the generic properties list)
- **AND** `display` does not appear as an item in the generic properties list

#### Scenario: Generic list excludes well-known properties

- **WHEN** the Properties View renders the generic properties list
- **THEN** any property key that has a dedicated layout slot is excluded from that list
- **AND** only properties without dedicated slots appear in the generic list

### Requirement: title slot renders as a labeled text input

The dedicated layout slot for `title` MUST render a text input with a visible "Title" label. The input MUST be pre-populated with the block's current `title` string value. When the block has no `title` property or its value is not a string, the input MUST show an empty string as the initial value. Changes to the input MUST be written immediately through the `set_property` mutation exposed by `core::Session`; they MUST NOT wait for the explicit Save action to be applied in memory. Persisting the value to disk MUST only happen when the user invokes the explicit Save control, consistent with the desktop shell's no-save policy.

#### Scenario: Title slot shows labeled text input

- **WHEN** the Properties View renders the title slot
- **THEN** a text input with the label "Title" is shown
- **AND** the input value reflects the block's current `title` string property

#### Scenario: Absent or non-string title shows empty input

- **WHEN** the Properties View loads a block that has no `title` property or whose `title` value is not a string
- **THEN** the title input is empty
- **AND** no error or placeholder copy is shown

#### Scenario: Title change written immediately via set_property

- **WHEN** the user edits the title input
- **THEN** the change is applied in memory through `core::Session::set_property` immediately
- **AND** no save to disk occurs until the user invokes the explicit Save control

### Requirement: display slot renders as a dropdown with no label

The dedicated layout slot for `display` MUST render a dropdown (select element) populated with the registered block view names available in the app. No property title label MUST be shown — the slot contains only the dropdown control.

#### Scenario: Display slot shows dropdown only

- **WHEN** the Properties View renders the display slot
- **THEN** a dropdown is shown with all registered view names as options
- **AND** no title label is rendered alongside the dropdown

### Requirement: Well-known edge property registry

The system MUST maintain a registry of well-known edge property keys that carry semantic meaning and are reserved for system use. The registry MUST include `"order"` as a well-known key for `EdgeType::Parent` edges. Well-known edge properties MUST NOT be surfaced in generic, user-editable property lists for edges. The normative specification for each well-known edge property key's behavior lives in the capability spec named alongside it.

| Key | Edge type | Capability spec |
|-----|-----------|-----------------|
| `"order"` | `Parent` | `child-ordering` |

#### Scenario: order is a reserved property key for parent edges

- **WHEN** the system writes a parent edge version record
- **THEN** the `"order"` key in the edge properties carries the fractional-index position value as defined by the child-ordering spec
- **AND** the key is not exposed as a generic user-editable property
