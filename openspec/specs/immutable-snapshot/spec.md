# immutable-snapshot Specification

## Purpose

Defines how active blocks and edges are derived at read time from radix-trie version stores via CRDT winner selection, tombstone exclusion, local invariant filtering, and per-call active reads without cached materialization.
## Requirements
### Requirement: Read-time materialization

The system MUST keep version records in radix-trie-backed stores as the authoritative in-memory representation. Active blocks and edges MUST be derived at read time via CRDT winner selection on the tries, not from a separate in-memory vector log or eager active maps.

#### Scenario: Active reads use trie winner

- **WHEN** a query or validation requires graph state
- **THEN** the system resolves active entities via per-call winner selection on block and edge version tries

#### Scenario: No separate in-memory version vector

- **WHEN** a knowledge base is loaded or mutated in a session
- **THEN** version records exist in trie storage only
- **AND** no parallel append-only vector holds the same records in memory

### Requirement: Conflict resolution rule

For each block `id` and each edge identity, the system MUST select the winning version record by **maximum `version`**, tie-breaking by **maximum `digest`** (lexicographic).

#### Scenario: Higher version wins

- **WHEN** two block version records share the same `id` with versions 2 and 3
- **THEN** the version 3 record is selected

#### Scenario: Digest tie-breaks equal version

- **WHEN** two block version records share the same `id` and `version` with different digests
- **THEN** the record with the lexicographically greater `digest` is selected

### Requirement: Tombstone exclusion

Winning records marked `tombstoned` MUST be excluded from the active snapshot.

#### Scenario: Tombstoned block absent

- **WHEN** the winning block version for an id is tombstoned
- **THEN** that block does not appear in the snapshot

#### Scenario: Tombstoned winner with older live versions

- **WHEN** the CRDT winner for a block or edge is tombstoned
- **AND** older non-tombstone versions exist for the same entity
- **THEN** the entity is still absent from the active view
- **AND** no fallback to an older non-tombstone version is performed

### Requirement: Invariant filtering

After selecting winners, read operations MUST apply only cheap, local invariant checks rather than global graph analysis. Successful mutation operations are assumed not to introduce invariant violations; read paths MUST NOT perform cycle detection or graph-wide exclusion passes. The single-writer mutation path maintains at most one active parent edge per block; after replication merge, multiple active parent edges for the same child MAY exist.

#### Scenario: Invalid edge ignored on point read

- **WHEN** a winning edge is returned by a point edge lookup
- **AND** either endpoint block is absent or tombstoned
- **THEN** that edge is treated as absent for that lookup

#### Scenario: No global cycle filtering at read time

- **WHEN** active parent edges contain a cycle
- **THEN** read operations do not run a global cycle detection pass to omit cycle participants

#### Scenario: Multiple active parent edges after merge

- **WHEN** replication merge produces multiple active `parent` edges with the same source and different targets
- **THEN** `parent_of` returns one matching parent with unspecified choice order
- **AND** no read-time repair is performed

### Requirement: Active read API

Consumer read methods (`block`, `parent`, `children`, `get_edge`, and related active-view lookups) MUST derive state per call from version tries without mutating trie contents. Version-record append methods (`append_block_version`, `append_edge_version`, `merge`, etc.) exist on `KnowledgeBase` for session, mutation, and storage paths and are not part of the active-read contract.

#### Scenario: Active reads do not mutate tries

- **WHEN** code calls active read methods on a knowledge base
- **THEN** version trie contents are unchanged by those calls

#### Scenario: Append methods insert version records

- **WHEN** a mutation appends a version record through the session path
- **THEN** the record is inserted into the appropriate version trie

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
- **AND** each returned edge has edge type `EdgeType::Parent` and target equal to the parent block id

### Requirement: Snapshot structural diff

The snapshot MUST support comparing two knowledge bases by lazily iterating differences in their version-record tries, using pointer-identical subtree sharing where applicable. Diff entries project `old` and `new` values through the active read API (post-CRDT-winner active entity state), not raw version records.

#### Scenario: Equal snapshots yield no diff entries

- **WHEN** diff iteration is performed on two knowledge bases with identical version-record trie structure
- **THEN** no block or edge diff entries are yielded
- **AND** block and edge trie roots are pointer-identical where maps are identical

#### Scenario: Single block property change yields active diff

- **WHEN** diff iteration is performed on two knowledge bases whose active views differ by one block's properties
- **THEN** at least one diff entry includes that changed active block
- **AND** unchanged subtrees remain shared between the two snapshot tries

### Requirement: Materialization equivalence

Replacing hash-map storage with radix trie storage MUST NOT change active graph semantics: CRDT winner selection (max version, max digest), tombstone exclusion, and invariant filtering produce the same set of active blocks and edges as before this change.

#### Scenario: Existing snapshot tests remain valid

- **WHEN** version history is materialized into a snapshot
- **THEN** active blocks, edges, root identity, and parent/child relationships match the prior hash-map-based materialization rules

### Requirement: All version records inserted

Mutations and merge MUST insert every new version record into the trie using full CRDT keys. Records MUST NOT be held only in a separate in-memory structure pending materialization.

#### Scenario: Append inserts directly into trie

- **WHEN** a mutation appends a version record
- **THEN** the record is inserted into the appropriate version trie immediately
- **AND** no full-history rematerialization pass is required

#### Scenario: Branched history retains all versions in trie

- **WHEN** version records exist for the same block id at the same version number with different digests
- **THEN** all such records are present in the trie under the same entity id prefix with distinct full keys

### Requirement: Pure per-call active reads

Each active read MUST recompute its result from version tries at call time without using cached active maps or cross-call invalidation state from prior reads.

#### Scenario: Point block lookup uses trie winner

- **WHEN** `block(id)` is called on a snapshot
- **THEN** the system resolves the winner under the block entity prefix in the block version trie on that call and returns active block fields or absence

#### Scenario: Point edge lookup uses trie winner

- **WHEN** `get_edge(source, target, edge_type)` is called
- **THEN** the system resolves the winner under the edge entity prefix in the edge version trie on that call and returns active edge fields or absence

#### Scenario: Children query uses trie prefix scan

- **WHEN** `children_of(parent)` is called
- **THEN** the system scans the edge version trie by parent navigation prefix on that call and resolves per-child winners without a pre-built edge map

#### Scenario: Parent lookup scans edge entities

- **WHEN** `parent_of(child)` is called
- **THEN** the system iterates distinct edge entity prefixes, filters to `Parent` type with matching source, and returns the target of the first active match
- **AND** no prefix navigation index is required for parent lookup

### Requirement: Mutation-presumed invariant validity

The system MUST validate graph invariants before appending mutation version records. Read operations MAY assume the active view for session-scoped mutation sequences does not require global invariant repair.

#### Scenario: Successful mutation preserves readable graph

- **WHEN** a mutation succeeds through the session mutation path
- **THEN** subsequent point reads for entities affected by that mutation reflect the intended graph structure without global read-time filtering

