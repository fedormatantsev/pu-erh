# immutable-snapshot Specification

## Purpose
TBD - created by archiving change immutable-graph-crdt. Update Purpose after archive.
## Requirements
### Requirement: Read-time materialization

The system MUST derive a read-only graph snapshot from version history at query time, not persist the snapshot as authoritative state. Materialization MUST populate radix-trie-backed version sets only; it MUST NOT require a separate eager pass that pre-computes cached active block and edge maps before reads.

#### Scenario: Snapshot built on read

- **WHEN** a query or validation requires graph state
- **THEN** the system provides a snapshot whose authoritative state is the merged block and edge version tries

#### Scenario: No eager active map rebuild

- **WHEN** version history is materialized into a snapshot after a mutation
- **THEN** the system updates version tries without building separate active block or edge hash maps upfront

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

### Requirement: Invariant filtering

After selecting winners, read operations MUST apply only cheap, local invariant checks rather than global graph analysis. Successful mutation operations are assumed not to introduce invariant violations; read paths MUST NOT perform cycle detection or graph-wide exclusion passes.

#### Scenario: Invalid edge ignored on point read

- **WHEN** a winning edge is returned by a point edge lookup
- **AND** either endpoint block is absent or tombstoned
- **THEN** that edge is treated as absent for that lookup

#### Scenario: No global cycle filtering at read time

- **WHEN** active parent edges contain a cycle
- **THEN** read operations do not run a global cycle detection pass to omit cycle participants

### Requirement: Snapshot immutability

Materialized snapshots MUST NOT expose mutation methods.

#### Scenario: Snapshot is read-only

- **WHEN** code holds a snapshot reference
- **THEN** it can read blocks and edges but cannot mutate snapshot state directly

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

### Requirement: Mutation-presumed invariant validity

The system MUST validate graph invariants before appending mutation version records. Read operations MAY assume the active view for session-scoped mutation sequences does not require global invariant repair.

#### Scenario: Successful mutation preserves readable graph

- **WHEN** a mutation succeeds through the session mutation path
- **THEN** subsequent point reads for entities affected by that mutation reflect the intended graph structure without global read-time filtering

