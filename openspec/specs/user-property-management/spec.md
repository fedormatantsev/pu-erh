# user-property-management Specification

## Purpose

Defines the classification, addition, and removal of user properties on blocks, including validation rules and the Session operations through which these mutations are applied.
## Requirements
### Requirement: User property classification

A user property is any block property key that is NOT in the reserved key set defined by **`property-registry`**. The reserved set includes `title`, `display`, and `body` for block properties. No separate registry of user properties is maintained; the classification is derived by exclusion.

#### Scenario: Non-reserved key is a user property

- **WHEN** a block has a property with a key that is not `title`, `display`, or `body`
- **THEN** that property is classified as a user property

#### Scenario: Reserved key is not a user property

- **WHEN** a block has a property with a reserved key (`title`, `display`, or `body`)
- **THEN** that property is NOT classified as a user property
- **AND** it is excluded from user-facing add and remove controls

### Requirement: Add user property

The system MUST allow adding a new user property to a block by supplying a non-empty, non-reserved key and a string value. The add operation MUST be rejected if the key is empty or matches any reserved block property key. The add operation MUST go through `core::Session::set_property` per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**.

#### Scenario: Add property with valid key and string value

- **WHEN** the user submits an add request with a non-empty, non-reserved key and a string value
- **THEN** `core::Session::set_property` is called with that block id, key, and `PropertyValue::String`
- **AND** the property appears in the block's generic properties list

#### Scenario: Add property with empty key is rejected

- **WHEN** the user submits an add request with an empty key
- **THEN** the request is rejected without calling `set_property`
- **AND** an error is indicated in the UI

#### Scenario: Add property with reserved key is rejected

- **WHEN** the user submits an add request with a key that matches a reserved block property key
- **THEN** the request is rejected without calling `set_property`
- **AND** an error is indicated in the UI

#### Scenario: Add property with duplicate key overwrites

- **WHEN** the user submits an add request with a key that already exists as a user property on the block
- **THEN** `core::Session::set_property` is called and overwrites the existing value
- **AND** no warning is required

### Requirement: Remove user property

The system MUST allow removing an existing user property from a block by key. The remove operation MUST go through `core::Session::remove_property` per **`agent-anti-default`**, Requirement: **UI adapters coordinate Session only**. Reserved properties MUST NOT be removable through this control.

#### Scenario: Remove existing user property

- **WHEN** the user activates the remove control for a user property row
- **THEN** `core::Session::remove_property` is called with that block id and key
- **AND** the property disappears from the generic properties list

#### Scenario: Remove control absent for reserved properties

- **WHEN** the Properties View renders any property slot for a reserved key
- **THEN** no remove control is shown for that slot
