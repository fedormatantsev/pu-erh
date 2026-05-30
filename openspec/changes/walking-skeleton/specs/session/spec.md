## ADDED Requirements

### Requirement: Session owns graph

A user session MUST hold the in-memory graph for one knowledge base and be the sole entry point for reads and writes during that session.

#### Scenario: Session contains loaded blocks

- **WHEN** a session is opened with a storage file containing blocks
- **THEN** the session's in-memory graph contains all blocks from that file

#### Scenario: Session initializes root for new knowledge base

- **WHEN** a session is opened and the storage file does not exist
- **THEN** the session's in-memory graph contains exactly one root block

### Requirement: Session coordinates persistence

The session MUST track whether the graph has been modified and support saving the current graph to storage.

#### Scenario: Save after mutation

- **WHEN** a mutation is applied in the session
- **THEN** the session marks the graph as modified
- **AND** saving writes the current graph to the storage file

#### Scenario: Save after new graph initialization

- **WHEN** a session is opened for a new knowledge base (missing storage file)
- **THEN** saving persists the root block to the storage file

#### Scenario: No save without mutations

- **WHEN** a session is opened from an existing storage file, only read queries are executed, and no mutations occur
- **THEN** the storage file on disk is unchanged
