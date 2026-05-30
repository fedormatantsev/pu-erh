# version-history Specification

## Purpose
TBD - created by archiving change immutable-graph-crdt. Update Purpose after archive.
## Requirements
### Requirement: Block version record

Each block mutation MUST append a block version record containing `id`, `version`, `digest`, `previous_digest`, and `properties`. The `id` is stable across versions of the same block.

#### Scenario: First version has no previous digest

- **WHEN** a block is first created
- **THEN** its version record has `version` equal to 1
- **AND** `previous_digest` is absent

#### Scenario: Subsequent version links to predecessor

- **WHEN** a new version of an existing block is appended
- **THEN** `version` is greater than the prior version for that `id`
- **AND** `previous_digest` equals the `digest` of the version record being extended

### Requirement: Edge version record

Each edge mutation MUST append an edge version record containing `source`, `target`, `edge_type` (a `#[repr(u8)]` enum), `version`, `digest`, `previous_digest`, and `properties`. Edge identity is the triple `(source, target, edge_type)`.

#### Scenario: Edge version chain

- **WHEN** a new version of an edge is appended
- **THEN** `version` increments for that edge identity
- **AND** `previous_digest` links to the prior edge version's `digest`

### Requirement: Digest computation

Each version record MUST include a `digest` computed as a BLAKE3 hash over the record's in-memory fields using in-place incremental hashing. Field values MUST be fed to the hasher in a fixed, documented order. Serialization MUST NOT be used as an intermediate step when computing digests. Property keys MUST be hashed in lexicographic order, followed by each key's JSON-encoded value.

#### Scenario: Same in-memory content produces same digest

- **WHEN** two records have identical in-memory field values hashed in the defined order
- **THEN** their `digest` values are equal

#### Scenario: Digest is stable across hash invocations

- **WHEN** the same record is hashed multiple times
- **THEN** the resulting `digest` is identical each time

#### Scenario: Property insertion order does not affect digest

- **WHEN** two property maps contain the same keys and values but entries were inserted in different orders
- **THEN** hashing either map produces the same digest contribution

### Requirement: Append-only history

Version records MUST NOT be updated or deleted in place. Mutations MUST only insert new records into the version tries.

#### Scenario: Mutation appends records

- **WHEN** a successful mutation occurs
- **THEN** new version record(s) are inserted into the version tries
- **AND** existing records in the tries are unchanged

### Requirement: History branching

The version history MUST retain all concurrent branches. Concurrent mutations MAY produce multiple version records for the same entity id that share the same `previous_digest` or the same `version` number with different `digest` values.

#### Scenario: Concurrent branches coexist

- **WHEN** two version records exist for the same block `id` at version 2 with different digests
- **THEN** both records remain in the history

### Requirement: Replication union

Merging knowledge bases from replication MUST union all version records into the trie without selecting a winner.

#### Scenario: Merge preserves all branches

- **WHEN** two knowledge bases are merged
- **THEN** the result trie contains all block and edge version records from both inputs

### Requirement: Append metadata from trie CRDT winner

When appending a version record, the system MUST derive `version` and `previous_digest` from the CRDT winner for that entity in the version trie (maximum `version`, tie-breaking by maximum `digest`, including tombstoned winners).

#### Scenario: First version has no previous digest

- **WHEN** a version record is appended for an entity with no prior records in the trie
- **THEN** its version is 1
- **AND** `previous_digest` is absent

#### Scenario: Subsequent version links to CRDT winner

- **WHEN** a new version of an existing entity is appended
- **THEN** `version` is one greater than the CRDT winner's version for that entity
- **AND** `previous_digest` equals the CRDT winner's `digest`

