## ADDED Requirements

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

Each edge mutation MUST append an edge version record containing `source`, `target`, `edge_type`, `version`, `digest`, `previous_digest`, and `properties`. Edge identity is the triple `(source, target, edge_type)`.

#### Scenario: Edge version chain

- **WHEN** a new version of an edge is appended
- **THEN** `version` increments for that edge identity
- **AND** `previous_digest` links to the prior edge version's `digest`

### Requirement: Digest computation

Each version record MUST include a `digest` computed as a BLAKE3 hash over the record's in-memory fields using in-place incremental hashing. Field values MUST be fed to the hasher in a fixed, documented order. Serialization MUST NOT be used as an intermediate step when computing digests.

#### Scenario: Same in-memory content produces same digest

- **WHEN** two records have identical in-memory field values hashed in the defined order
- **THEN** their `digest` values are equal

#### Scenario: Digest is stable across hash invocations

- **WHEN** the same record is hashed multiple times
- **THEN** the resulting `digest` is identical each time

### Requirement: Append-only history

Version records MUST NOT be updated or deleted in place. Mutations MUST only append new records.

#### Scenario: Mutation appends records

- **WHEN** a successful mutation occurs
- **THEN** new version record(s) are appended to the history
- **AND** existing records are unchanged

### Requirement: History branching

The version history MUST retain all concurrent branches. Concurrent mutations MAY produce multiple version records for the same entity id that share the same `previous_digest` or the same `version` number with different `digest` values.

#### Scenario: Concurrent branches coexist

- **WHEN** two version records exist for the same block `id` at version 2 with different digests
- **THEN** both records remain in the history

### Requirement: Replication union

Merging histories from replication MUST union all version records without selecting a winner.

#### Scenario: Merge preserves all branches

- **WHEN** two histories are merged
- **THEN** the result contains all block and edge version records from both inputs
