## ADDED Requirements

### Requirement: Trie-backed version record storage

The materialized snapshot MUST store block and edge version records in radix trie maps. Each key MUST consist of an entity identity prefix followed by CRDT fields (`version` as big-endian u64, `digest` as 32 bytes, `previous_digest` as 32 bytes with zeros when absent): 88 bytes for blocks, 105 bytes for edges.

#### Scenario: Block active read resolves winner

- **WHEN** an active block exists for a given id
- **THEN** lookup by that block's 16-byte id prefix via prefix descent and last-child descent returns the winning non-tombstoned version record's block fields

#### Scenario: Edge active read resolves winner

- **WHEN** an active edge exists for a given target, type, and source
- **THEN** lookup by the 33-byte edge entity prefix returns the winning non-tombstoned version record's edge fields

### Requirement: Prefix-based edge queries

The snapshot MUST support retrieving active edges whose entity identity prefixes extend a given navigation prefix without scanning unrelated keys.

#### Scenario: Children via parent prefix

- **WHEN** parent edge version records exist with target set to a given parent block id
- **AND** query uses the 17-byte prefix of target UUID bytes followed by the `Parent` edge type byte
- **THEN** the winning non-tombstoned parent edge per distinct source child is returned
- **AND** each returned edge has edge type `parent` and target equal to the parent block id

### Requirement: Snapshot structural diff

The snapshot MUST support comparing two materialized snapshots by iterating block and edge differences lazily, using pointer-identical subtree sharing where applicable.

#### Scenario: Equal snapshots yield no diff entries

- **WHEN** diff iteration is performed on two snapshots materialized from identical version history
- **THEN** no block or edge diff entries are yielded
- **AND** block and edge trie roots are pointer-identical where maps are identical

#### Scenario: Single block change localized diff

- **WHEN** diff iteration is performed on two snapshots that differ by one block's properties
- **THEN** exactly one changed block entry is yielded among the divergent items
- **AND** unchanged subtrees remain shared between the two snapshot tries

### Requirement: Materialization equivalence

Replacing hash-map storage with radix trie storage MUST NOT change active graph semantics: CRDT winner selection (max version, max digest), tombstone exclusion, and invariant filtering produce the same set of active blocks and edges as before this change.

#### Scenario: Existing snapshot tests remain valid

- **WHEN** version history is materialized into a snapshot
- **THEN** active blocks, edges, root identity, and parent/child relationships match the prior hash-map-based materialization rules

### Requirement: All version records inserted

Materialization MUST insert every block and edge version record from history into the trie using full CRDT keys, not only pre-selected winners.

#### Scenario: Branched history retains all versions in trie

- **WHEN** version history contains multiple version records for the same block id at the same version number with different digests
- **THEN** all such records are present in the trie under the same entity id prefix with distinct full keys
