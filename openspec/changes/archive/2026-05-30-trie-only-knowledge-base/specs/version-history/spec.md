## MODIFIED Requirements

### Requirement: Append-only history

Version records MUST NOT be updated or deleted in place. Mutations MUST only insert new records into the version tries.

#### Scenario: Mutation appends records

- **WHEN** a successful mutation occurs
- **THEN** new version record(s) are inserted into the version tries
- **AND** existing records in the tries are unchanged

### Requirement: Replication union

Merging knowledge bases from replication MUST union all version records into the trie without selecting a winner.

#### Scenario: Merge preserves all branches

- **WHEN** two knowledge bases are merged
- **THEN** the result trie contains all block and edge version records from both inputs

## ADDED Requirements

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
