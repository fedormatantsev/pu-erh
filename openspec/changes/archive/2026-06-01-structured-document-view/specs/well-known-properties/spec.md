## ADDED Requirements

### Requirement: body is a reserved rich-text content property

`body` MUST be a reserved block property key whose value, when present, MUST be a `PropertyValue::String` holding the block's serialized rich-text editor state as produced and consumed by the Structured Document View. Because its value is an opaque serialized payload rather than user-facing scalar text, `body` MUST be excluded from the generic, user-editable properties list in the Properties View; it MUST NOT be presented as a free-text property item. `body` MUST be written only through the `set_property` mutation exposed by `core::Session`.

#### Scenario: body excluded from the generic properties list

- **WHEN** the Properties View renders the generic properties list for a block that has a `body` property
- **THEN** `body` does not appear as an item in the generic properties list

#### Scenario: body holds serialized rich text

- **WHEN** a block carries paragraph rich-text content
- **THEN** the block's properties map contains the key `body` with a `PropertyValue::String` value

#### Scenario: body written via set_property

- **WHEN** the Structured Document View changes a paragraph's content
- **THEN** the change is applied through the `core::Session` `set_property` mutation with key `body`
