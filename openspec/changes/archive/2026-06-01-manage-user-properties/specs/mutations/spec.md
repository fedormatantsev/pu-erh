## ADDED Requirements

### Requirement: Remove block property

The system MUST support removing a single property from an existing block via a `remove_property(block_id, key)` mutation. The mutation MUST append a new block version record whose properties are the block's current properties with `key` absent. Removing a property from a block not present in the active view MUST be rejected. Removing a key that is not present in the block's current properties MUST be rejected. The mutation MUST validate against the active view before appending, consistent with `set_property`.

#### Scenario: Remove an existing property

- **WHEN** `remove_property` is applied to a block present in the snapshot with a key that exists in the block's properties
- **THEN** a new block version record is appended whose properties equal the block's prior properties with that key absent
- **AND** after append the active block no longer has that property via per-call CRDT winner selection

#### Scenario: Remove property on nonexistent block

- **WHEN** `remove_property` is applied to an id not present in the snapshot
- **THEN** the system returns an error and no version records are appended

#### Scenario: Remove nonexistent property key

- **WHEN** `remove_property` is applied to a block present in the snapshot with a key that does not exist in the block's properties
- **THEN** the system returns an error and no version records are appended
