## MODIFIED Requirements

### Requirement: Properties View surface

The Properties View MUST show settings for the current selected block, including well-known property slots, a generic user properties list, and controls to add and remove user properties. The Properties View MUST be the surface through which `display` is changed per **`property-registry`**, Requirement: **display property**. The Properties View MUST coordinate `core::Session` only per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**.

#### Scenario: Properties View shows current block settings

- **WHEN** the Properties View is shown
- **THEN** it presents controls for the current selected block's well-known properties, generic user properties list, and add-property form

## ADDED Requirements

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
