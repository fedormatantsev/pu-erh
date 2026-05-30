# version-history Specification

## Purpose

Defines append-only block and edge version records, BLAKE3 digest computation and verification, CRDT append metadata, history branching, and replication merge semantics for the trie-backed knowledge base.
## Requirements
### Requirement: Block version record

Each block mutation MUST append a block version record containing `id`, `version`, `digest`, `previous_digest`, `tombstoned`, and `properties`. The `id` is stable across versions of the same block.

#### Scenario: First version has no previous digest

- **WHEN** a block is first created
- **THEN** its version record has `version` equal to 1
- **AND** `previous_digest` is absent

#### Scenario: Subsequent version links to predecessor

- **WHEN** a new version of an existing block is appended
- **THEN** `version` is greater than the prior version for that `id`
- **AND** `previous_digest` equals the `digest` of the version record being extended

### Requirement: Edge version record

Each edge mutation MUST append an edge version record containing `source`, `target`, `edge_type` (a `#[repr(u8)]` enum), `version`, `digest`, `previous_digest`, `tombstoned`, and `properties`. Edge identity for CRDT indexing and trie keys is the triple `(target, edge_type, source)` — encoded as 33 raw bytes in that order. Record fields remain `source`, `target`, and `edge_type` separately.

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

### Requirement: Digest field order

Digest computation MUST hash field values in the following fixed order using BLAKE3 incremental hashing. Property values MUST be JSON-serializable; non-serializable values MUST cause digest computation to fail.

**Block version record hash input order:**

1. `id` — 16 raw UUID bytes
2. `version` — u64 little-endian (8 bytes)
3. `tombstoned` — single byte (`0` or `1`)
4. `properties` — for each key in lexicographic order: key UTF-8 bytes, then JSON-encoded value bytes

**Edge version record hash input order:**

1. `source` — 16 raw UUID bytes
2. `target` — 16 raw UUID bytes
3. `edge_type` — single u8 byte
4. `version` — u64 little-endian (8 bytes)
5. `tombstoned` — single byte (`0` or `1`)
6. `properties` — for each key in lexicographic order: key UTF-8 bytes, then JSON-encoded value bytes

Trie CRDT key suffixes encode `version` as **big-endian** u64 for lexicographic ordering; digest hashing uses **little-endian** u64 as specified above. These encodings serve different purposes and MUST NOT be conflated.

#### Scenario: Block digest matches field order

- **WHEN** a block version record is hashed
- **THEN** the digest equals BLAKE3 over the block field order specified above

#### Scenario: Edge digest matches field order

- **WHEN** an edge version record is hashed
- **THEN** the digest equals BLAKE3 over the edge field order specified above

### Requirement: Verify digest on load

When loading version records from storage, the system MUST recompute each record's digest from its field values and reject the load if the stored `digest` does not match.

#### Scenario: Tampered digest rejected

- **WHEN** storage contains a version record whose stored `digest` does not match recomputation
- **THEN** the system returns an error describing the digest mismatch

#### Scenario: Valid records load successfully

- **WHEN** all version records pass digest verification
- **THEN** the knowledge base is constructed from the deserialized records

### Requirement: Previous digest encoding

In JSON persistence, first-version records MUST have `previous_digest` absent (`null` or omitted). In trie CRDT key suffixes, absent `previous_digest` is encoded as 32 zero bytes; this sentinel applies to key encoding only.

#### Scenario: First version has null previous digest in JSON

- **WHEN** a version record with no predecessor is saved to storage
- **THEN** `previous_digest` is absent in JSON
- **AND** the trie key suffix uses 32 zero bytes for the previous-digest field

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

#### Scenario: Merge deduplicates identical full CRDT keys

- **WHEN** two knowledge bases contain version records with the same full CRDT trie key
- **THEN** the merged trie retains one record for that key (last inserted wins)

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

