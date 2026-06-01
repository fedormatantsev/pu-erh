# properties-view Specification

## Purpose

Properties View surface: well-known property layout slots (`title`, `display`), generic property list exclusions, and action descriptors for Properties mode.
## Requirements
### Requirement: Properties View surface

The Properties View MUST show settings for the current selected block, including well-known property slots, a generic user properties list, and controls to add and remove user properties. The Properties View MUST be the surface through which `display` is changed per **`property-registry`**, Requirement: **display property**. The Properties View MUST coordinate `core::Session` only per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**.

#### Scenario: Properties View shows current block settings

- **WHEN** the Properties View is shown
- **THEN** it presents controls for the current selected block's well-known properties, generic user properties list, and add-property form

### Requirement: Well-known property layout slots

The Properties View layout MUST reserve dedicated, fixed-position UI slots for well-known predefined properties. A well-known property MUST NOT appear as an item in the generic properties list. `title` MUST be the first slot, followed by `display` as the second. Keys with dedicated slots are defined in **`property-registry`**.

#### Scenario: title rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `title` property is rendered in its dedicated layout slot before the `display` slot and before the generic properties list
- **AND** `title` does not appear in the generic properties list

#### Scenario: display rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `display` property is rendered in its dedicated layout slot after the `title` slot and before the generic properties list
- **AND** `display` does not appear in the generic properties list

#### Scenario: Generic list excludes well-known and reserved properties

- **WHEN** the Properties View renders the generic properties list
- **THEN** any property key with a dedicated layout slot or reserved semantics in **`property-registry`** (including `body`) is excluded from that list
- **AND** only unreserved properties without dedicated slots appear in the generic list

### Requirement: Generic list shows add-property form

The Properties View generic properties list MUST include an inline add-property form rendered at the bottom of the list. The form MUST contain a key input, a value input, and a confirm action. Submitting the form MUST delegate to the add-user-property operation per **`user-property-management`**, Requirement: **Add user property**. Persistence timing follows the **`desktop-shell`** save policy.

#### Scenario: Add form present in generic properties list

- **WHEN** the Properties View renders the generic properties list
- **THEN** an add-property form is shown at the bottom of the list with a key input and a value input

#### Scenario: Valid add submits through set_property

- **WHEN** the user fills in a non-empty, non-reserved key and a value and confirms
- **THEN** the property is added via `core::Session::set_property`
- **AND** the new property appears in the generic list

#### Scenario: Invalid add shows error in form

- **WHEN** the user submits the add form with an empty or reserved key
- **THEN** the form shows an error indication
- **AND** `set_property` is NOT called

### Requirement: Generic list user property rows have remove action

Each row in the generic properties list that represents a user property MUST render a remove action. Activating the remove action MUST delegate to the remove-user-property operation per **`user-property-management`**, Requirement: **Remove user property**. Persistence timing follows the **`desktop-shell`** save policy.

#### Scenario: User property row has remove action

- **WHEN** the Properties View renders a user property row in the generic list
- **THEN** a remove action is visible on that row

#### Scenario: Remove action deletes the property

- **WHEN** the user activates the remove action on a user property row
- **THEN** `core::Session::remove_property` is called with that key
- **AND** the row disappears from the list

### Requirement: title slot renders as a labeled text input

The dedicated slot for `title` MUST render a text input with a visible "Title" label. The input MUST reflect the block's current `title` string value per **`property-registry`**, Requirement: **title property**, Scenario: **Absent title in user-facing editors**. Changes MUST apply in memory immediately through `set_property` per **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**.

#### Scenario: Title slot shows labeled text input

- **WHEN** the Properties View renders the title slot
- **THEN** a text input with the label "Title" is shown
- **AND** the input value reflects the block's current `title` string property when present

#### Scenario: Absent or non-string title shows empty input

- **WHEN** the Properties View loads a block with no `title` property or a non-string `title`
- **THEN** the title input is empty
- **AND** no error or placeholder copy is shown

#### Scenario: Title change written immediately via set_property

- **WHEN** the user edits the title input
- **THEN** the change is applied in memory through `core::Session::set_property` immediately
- **AND** persistence timing follows **`desktop-shell`** save policy

### Requirement: display slot renders as a dropdown with no label

The dedicated slot for `display` MUST render a dropdown populated with registered renderer keys from the **`desktop-shell-ui`** renderer registry. No property title label MUST be shown. When the block has no `display` property or its stored value is not a registered key, the selector MUST implicitly resolve to the default primary renderer and MUST overwrite the stored value with the resolved default on the next save per **`desktop-shell`**.

#### Scenario: Display slot shows dropdown only

- **WHEN** the Properties View renders the display slot
- **THEN** a dropdown is shown with all registered renderer keys as options
- **AND** no title label is rendered alongside the dropdown

#### Scenario: Absent display resolves to default in selector

- **WHEN** the Properties View loads a block that has no `display` property
- **THEN** the selector shows the default primary renderer as the selected value
- **AND** no write to storage occurs at load time

#### Scenario: Unrecognized display resolves to default in selector

- **WHEN** the Properties View loads a block whose `display` value is not a registered renderer key
- **THEN** the selector shows the default primary renderer as the selected value
- **AND** no error or warning is surfaced to the user

#### Scenario: Absent or unrecognized display overwritten on save

- **WHEN** a block's `display` was absent or unrecognized at load time
- **AND** the user saves per **`desktop-shell`**
- **THEN** the resolved default value is written to the block's `display` property before persisting

### Requirement: Properties View action descriptors

When the Properties View is the active view, its action descriptor factory MUST return only the toggle-to-block-view action per **`action-bar`**.

#### Scenario: Properties View supplies toggle only

- **WHEN** the Properties View is the active view
- **THEN** its action factory returns only the toggle-to-block-view descriptor
- **AND** no create-child action

