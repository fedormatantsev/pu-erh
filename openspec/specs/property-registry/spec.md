# property-registry Specification

## Purpose

Canonical domain semantics for reserved block and edge property keys (`title`, `display`, `body`, and the parent-edge `order` registry entry).
## Requirements
### Requirement: title property

`title` MUST be a reserved block property key. When present, its value MUST be a `PropertyValue::String`. Writes MUST go through the `set_property` mutation per the **`mutations`** capability.

#### Scenario: title is a reserved string property

- **WHEN** a block has a title configured
- **THEN** the block's properties map contains the key `title` with a `PropertyValue::String` value

#### Scenario: Absent title in user-facing editors

- **WHEN** a user-facing editor slot for `title` is rendered (Properties View per **`properties-view`**, Document View heading per **`document-view`**)
- **THEN** the control is empty
- **AND** no placeholder or descriptive copy is shown

#### Scenario: Absent title in inline labels

- **WHEN** a block has no `title` property or a non-string `title`
- **THEN** inline label text MUST NOT be invented descriptive or marketing copy
- **AND** the inline label value is determined by **`desktop-shell-ui`**, Requirement: **Default renderer**

### Requirement: display property

`display` MUST be a reserved block property key. Its value MUST be a `PropertyValue::String` naming the renderer registered in the **`desktop-shell-ui`** renderer registry. `display` MUST be stored in the block properties map. Writes MUST go through the `set_property` mutation per the **`mutations`** capability; the UI layer MUST NOT introduce display-specific mutation or validation logic.

#### Scenario: display is a reserved string property

- **WHEN** a block has a renderer configured
- **THEN** the block's properties map contains the key `display` with a `PropertyValue::String` value naming a registered renderer

#### Scenario: display changed via set_property

- **WHEN** the Properties View changes a block's `display`
- **THEN** the change is applied through `core::Session::set_property`
- **AND** no display-specific mutation or validation logic is introduced in the UI layer

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

### Requirement: Well-known edge property registry

The system MUST maintain a registry of well-known edge property keys reserved for system use. The registry MUST include `"order"` for `EdgeType::Parent` edges. Normative behavior for each key lives in the capability named alongside it.

| Key | Edge type | Capability spec |
|-----|-----------|-----------------|
| `"order"` | `Parent` | `child-ordering` |

Well-known edge properties MUST NOT be surfaced in generic user-editable property lists.

#### Scenario: order is a reserved property key for parent edges

- **WHEN** the system writes a parent edge version record
- **THEN** the `"order"` key carries the fractional-index position value per **`child-ordering`**
- **AND** the key is not exposed as a generic user-editable property

