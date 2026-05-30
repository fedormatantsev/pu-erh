## ADDED Requirements

### Requirement: Versioned block and edge state

Block and edge structure in the active view MUST be determined by materializing version history at read time. The winning version record for each entity id defines the active block or edge.

#### Scenario: Active block from winning version

- **WHEN** version history contains multiple versions of a block id
- **THEN** the active block reflects the winning version's properties

#### Scenario: Tombstoned block absent

- **WHEN** the winning version of a block is tombstoned
- **THEN** that block does not appear in the active view

## MODIFIED Requirements

### Requirement: Edge key

Active edges in the materialized snapshot MUST be indexed by composite key `{target_id}{type}{source_id}`. The active edge for each identity MUST come from the winning edge version record.

#### Scenario: Key determines uniqueness

- **WHEN** two active edges share the same target, type, and source
- **THEN** the system treats them as the same edge and MUST NOT store duplicates

#### Scenario: Superseded edge versions inactive

- **WHEN** an edge identity has a winning tombstoned version
- **THEN** that edge is absent from the active view
