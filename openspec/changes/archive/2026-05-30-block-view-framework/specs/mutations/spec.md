## ADDED Requirements

### Requirement: Set block property

The system MUST support setting a single property on an existing block via a generic `set_property(block, key, value)` mutation, where `value` is a `PropertyValue`. The mutation MUST append a new block version record whose properties are the block's current properties with `key` set to `value`. Setting a property on a block not present in the active view MUST be rejected. The mutation MUST validate against the active view before appending, consistent with create/move/delete.

#### Scenario: Set a property on an existing block

- **WHEN** `set_property` is applied to a block present in the snapshot with a key and a `PropertyValue`
- **THEN** a new block version record is appended whose properties equal the block's prior properties with that key set to the value
- **AND** after append the active block reflects the new property via per-call CRDT winner selection

#### Scenario: Overwrite an existing property key

- **WHEN** `set_property` is applied with a key the block already has
- **THEN** the appended block version replaces that key's value with the new value
- **AND** other properties are unchanged

#### Scenario: Set property on nonexistent block

- **WHEN** `set_property` is applied to an id not present in the snapshot
- **THEN** the system returns an error and no version records are appended
