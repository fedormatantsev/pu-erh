## MODIFIED Requirements

### Requirement: Session owns graph

A user session MUST hold one trie-backed knowledge base for one storage file and be the sole entry point for reads and writes during that session.

#### Scenario: Session contains loaded blocks

- **WHEN** a session is opened with a storage file that includes block version records
- **THEN** the session's knowledge base serves active blocks via pure per-call reads from block version tries

#### Scenario: Session initializes root for new knowledge base

- **WHEN** a session is opened for a new knowledge base and the first save occurs
- **THEN** a root block version record is inserted into the knowledge base
- **AND** the knowledge base contains exactly one root block

### Requirement: Session coordinates persistence

The session MUST track whether the knowledge base has been modified and support saving to storage.

#### Scenario: Save after mutation

- **WHEN** a mutation inserts version records
- **THEN** the session marks the knowledge base as modified
- **AND** saving exports version records from the knowledge base to the storage file

#### Scenario: Save after new graph initialization

- **WHEN** a session is opened for a new knowledge base and save is called
- **THEN** saving persists the root block version record

#### Scenario: No save without mutations

- **WHEN** a session is opened from an existing storage file, only read queries are executed, and no mutations occur
- **THEN** the storage file on disk is unchanged

## REMOVED Requirements

### Requirement: Session owns version history

**Reason:** Replaced by a single trie-backed knowledge base; the in-memory vector log is removed.

**Migration:** Sessions expose the knowledge base (or snapshot alias) instead of separate version history and materialized snapshot.

### Requirement: Incremental snapshot update on mutation

**Reason:** Mutations insert directly into version tries; rematerialization from a vector log is no longer used.

**Migration:** After mutation, new records are already in the trie; no rematerialize step.

## ADDED Requirements

### Requirement: Direct trie update on mutation

After a successful mutation, the session MUST insert new version records into the knowledge base tries without rebuilding from a separate in-memory log.

#### Scenario: Mutation validates before append

- **WHEN** a mutation is requested through the session
- **THEN** invariant validation runs against the current knowledge base before version records are inserted

#### Scenario: No rematerialize after mutation

- **WHEN** a mutation succeeds
- **THEN** new version records are present in the knowledge base tries
- **AND** the session does not rebuild tries from a vector history
