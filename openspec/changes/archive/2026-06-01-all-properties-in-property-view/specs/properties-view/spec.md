## MODIFIED Requirements

### Requirement: Well-known property layout slots

The Properties View layout MUST reserve dedicated, fixed-position UI slots for well-known predefined properties. A property with a dedicated layout slot MUST NOT appear as an item in the generic properties list. `title` MUST be the first slot, followed by `display` as the second. Keys with dedicated slots are defined in **`property-registry`**. A property without a dedicated slot MUST appear in the generic properties list regardless of whether the **`property-registry`** assigns it reserved domain semantics.

#### Scenario: title rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `title` property is rendered in its dedicated layout slot before the `display` slot and before the generic properties list
- **AND** `title` does not appear in the generic properties list

#### Scenario: display rendered in its dedicated slot

- **WHEN** the Properties View is shown
- **THEN** the `display` property is rendered in its dedicated layout slot after the `title` slot and before the generic properties list
- **AND** `display` does not appear in the generic properties list

#### Scenario: Generic list excludes only dedicated-slot keys

- **WHEN** the Properties View renders the generic properties list
- **THEN** only properties whose keys have a dedicated layout slot (`title`, `display`) are excluded from that list
- **AND** every other property present on the block — including reserved-semantics keys such as `body` per **`property-registry`** — appears in the generic list

#### Scenario: body present on the current block appears in the generic list

- **WHEN** the Properties View is shown for a block that carries a `body` property
- **THEN** a row for `body` is rendered in the generic properties list
- **AND** the row exposes the same remove action as any other generic row per Requirement: **Generic list user property rows have remove action**

### Requirement: Generic list shows add-property form

The Properties View generic properties list MUST include an inline add-property form rendered at the bottom of the list. The form MUST contain a key input, a value input, and a confirm action. Submitting the form MUST delegate to the add-user-property operation per **`user-property-management`**, Requirement: **Add user property**. The form MUST reject only keys with a dedicated layout slot (`title`, `display`) as invalid; any other key — including reserved-semantics keys without a dedicated slot such as `body` per **`property-registry`** — MUST be accepted. Persistence timing follows the **`desktop-shell`** save policy.

#### Scenario: Add form present in generic properties list

- **WHEN** the Properties View renders the generic properties list
- **THEN** an add-property form is shown at the bottom of the list with a key input and a value input

#### Scenario: Valid add submits through set_property

- **WHEN** the user fills in a non-empty key that is not a dedicated-slot key and a value, and confirms
- **THEN** the property is added via `core::Session::set_property`
- **AND** the new property appears in the generic list

#### Scenario: Empty key shows error in form

- **WHEN** the user submits the add form with an empty key
- **THEN** the form shows an error indication
- **AND** `set_property` is NOT called

#### Scenario: Dedicated-slot key shows error in form

- **WHEN** the user submits the add form with a key that has a dedicated layout slot (`title` or `display`)
- **THEN** the form shows an error indication
- **AND** `set_property` is NOT called

#### Scenario: Reserved-semantics non-slot key accepted by add form

- **WHEN** the user submits the add form with the key `body` and a value
- **THEN** the property is added via `core::Session::set_property`
- **AND** a row for `body` appears in the generic list
