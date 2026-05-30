# immutable-snapshot Specification

## Purpose
TBD - created by archiving change immutable-graph-crdt. Update Purpose after archive.
## Requirements
### Requirement: Read-time materialization

The system MUST derive a read-only graph snapshot from version history at query time, not persist the snapshot as authoritative state.

#### Scenario: Snapshot built on read

- **WHEN** a query or validation requires graph state
- **THEN** the system materializes a snapshot from the current version history

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

After selecting winners, the system MUST drop any block or edge that violates graph invariants rather than returning an error.

#### Scenario: Invalid edge ignored

- **WHEN** a winning edge references a block id not present in the active snapshot
- **THEN** that edge is omitted from the snapshot

#### Scenario: Cycle participant ignored

- **WHEN** active parent edges contain a cycle
- **THEN** edges participating in the cycle are omitted from the snapshot

### Requirement: Snapshot immutability

Materialized snapshots MUST NOT expose mutation methods.

#### Scenario: Snapshot is read-only

- **WHEN** code holds a snapshot reference
- **THEN** it can read blocks and edges but cannot mutate snapshot state directly

