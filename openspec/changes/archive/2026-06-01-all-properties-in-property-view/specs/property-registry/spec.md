## MODIFIED Requirements

### Requirement: body property

`body` MUST be a reserved block property key whose value, when present, MUST be a `PropertyValue::String` holding a block's serialized rich-text editor state as produced and consumed by the Document View per **`document-view`**. Writes MUST go through `set_property` per **`mutations`**. The `body` key MUST NOT carry a dedicated layout slot in any user-facing surface; visibility in introspection surfaces such as the Properties View is governed by **`properties-view`**, Requirement: **Well-known property layout slots**, not by this registry.

#### Scenario: body holds serialized rich text

- **WHEN** a block carries paragraph rich-text content
- **THEN** the block's properties map contains the key `body` with a `PropertyValue::String` value

#### Scenario: body written via set_property

- **WHEN** the Document View changes paragraph content
- **THEN** the change is applied through `core::Session::set_property` with key `body`

#### Scenario: body has no dedicated layout slot in the Properties View

- **WHEN** the Properties View is shown for a block carrying a `body` property
- **THEN** `body` MUST NOT occupy a dedicated layout slot
- **AND** `body` appears in the generic properties list per **`properties-view`**, Scenario: **Generic list excludes only dedicated-slot keys**
