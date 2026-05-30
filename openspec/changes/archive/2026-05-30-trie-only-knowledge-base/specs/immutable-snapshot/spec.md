## MODIFIED Requirements

### Requirement: Read-time materialization

The system MUST keep version records in radix-trie-backed stores as the authoritative in-memory representation. Active blocks and edges MUST be derived at read time via CRDT winner selection on the tries, not from a separate in-memory vector log or eager active maps.

#### Scenario: Active reads use trie winner

- **WHEN** a query or validation requires graph state
- **THEN** the system resolves active entities via per-call winner selection on block and edge version tries

#### Scenario: No separate in-memory version vector

- **WHEN** a knowledge base is loaded or mutated in a session
- **THEN** version records exist in trie storage only
- **AND** no parallel append-only vector holds the same records in memory

### Requirement: All version records inserted

Mutations and merge MUST insert every new version record into the trie using full CRDT keys. Records MUST NOT be held only in a separate in-memory structure pending materialization.

#### Scenario: Append inserts directly into trie

- **WHEN** a mutation appends a version record
- **THEN** the record is inserted into the appropriate version trie immediately
- **AND** no full-history rematerialization pass is required

#### Scenario: Branched history retains all versions in trie

- **WHEN** version records exist for the same block id at the same version number with different digests
- **THEN** all such records are present in the trie under the same entity id prefix with distinct full keys
