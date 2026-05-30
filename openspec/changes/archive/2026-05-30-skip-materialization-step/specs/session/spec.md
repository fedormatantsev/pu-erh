## MODIFIED Requirements

### Requirement: Session owns graph

A user session MUST hold the materialized snapshot for one knowledge base and be the sole entry point for reads and writes during that session.

#### Scenario: Session contains loaded blocks

- **WHEN** a session is opened with version history that includes block records
- **THEN** the session's snapshot serves active blocks via pure per-call reads from block version tries

#### Scenario: Session initializes root for new knowledge base

- **WHEN** a session is opened for a new knowledge base and the first save occurs
- **THEN** a root block version record is appended
- **AND** the materialized snapshot contains exactly one root block

### Requirement: Session owns version history

A user session MUST hold the append-only version history and a materialized snapshot for one knowledge base.

#### Scenario: Session materializes on open

- **WHEN** a session is opened with a storage file containing version records
- **THEN** the session merges version records into snapshot tries without an eager active-view rebuild

#### Scenario: Session starts empty for new knowledge base

- **WHEN** a session is opened and the storage file does not exist
- **THEN** the session holds an empty version history

## ADDED Requirements

### Requirement: Incremental snapshot update on mutation

After a successful mutation, the session MUST update snapshot tries by merging newly appended version records, sharing unchanged subtrees with the prior snapshot where possible.

#### Scenario: Rematerialize merges without active map rebuild

- **WHEN** a mutation succeeds and the session rematerializes
- **THEN** new version records are inserted into snapshot tries
- **AND** no hash-map active view is built before or after rematerialize

#### Scenario: Mutation validates before append

- **WHEN** a mutation is requested through the session
- **THEN** invariant validation runs against the current snapshot before version records are appended
