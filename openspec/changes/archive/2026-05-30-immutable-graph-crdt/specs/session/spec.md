## ADDED Requirements

### Requirement: Session owns version history

A user session MUST hold the append-only version history and a materialized snapshot for one knowledge base.

#### Scenario: Session materializes on open

- **WHEN** a session is opened with a storage file containing version records
- **THEN** the session materializes a snapshot from that history

#### Scenario: Session starts empty for new knowledge base

- **WHEN** a session is opened and the storage file does not exist
- **THEN** the session holds an empty version history

## MODIFIED Requirements

### Requirement: Session owns graph

A user session MUST hold the materialized snapshot for one knowledge base and be the sole entry point for reads and writes during that session.

#### Scenario: Session contains loaded blocks

- **WHEN** a session is opened with version history that includes block records
- **THEN** the session's materialized snapshot contains the active blocks from read-time resolution

#### Scenario: Session initializes root for new knowledge base

- **WHEN** a session is opened for a new knowledge base and the first save occurs
- **THEN** a root block version record is appended
- **AND** the materialized snapshot contains exactly one root block

### Requirement: Session coordinates persistence

The session MUST track whether version history has been modified and support saving to storage.

#### Scenario: Save after mutation

- **WHEN** a mutation appends version records
- **THEN** the session marks history as modified
- **AND** saving writes the version history to the storage file

#### Scenario: Save after new graph initialization

- **WHEN** a session is opened for a new knowledge base and save is called
- **THEN** saving persists the root block version record

#### Scenario: No save without mutations

- **WHEN** a session is opened from an existing storage file, only read queries are executed, and no mutations occur
- **THEN** the storage file on disk is unchanged
