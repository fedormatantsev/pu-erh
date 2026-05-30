# block-model Specification

## Purpose
TBD - created by archiving change walking-skeleton. Update Purpose after archive.
## Requirements
### Requirement: Block identity

Each block in the knowledge base MUST have a unique identifier represented as a UUID v4.

#### Scenario: New block receives UUID

- **WHEN** a block is created
- **THEN** the system assigns a newly generated UUID v4 as its id
- **AND** no two blocks in the same knowledge base share the same id

### Requirement: Block structure

Each block MUST contain an id and a properties map (generic key/value dictionary). Blocks MUST NOT have a type field or embed parent or other relation fields — relations are represented as edges.

#### Scenario: Block fields are accessible

- **WHEN** a block exists in the graph
- **THEN** its id and properties are readable

#### Scenario: New block has empty properties by default

- **WHEN** a block is created without explicit properties
- **THEN** its properties map is empty

### Requirement: Edge structure

Each edge MUST have a source id, a target id, an edge type represented as a `#[repr(u8)]` enum, and a properties map (generic key/value dictionary).

#### Scenario: Edge fields are accessible

- **WHEN** an edge exists in the graph
- **THEN** its source, target, edge type, and properties are readable

#### Scenario: Edge type is u8-backed

- **WHEN** an edge exists in the graph
- **THEN** its edge type is one of the defined `EdgeType` variants serialized as a u8

#### Scenario: New edge has empty properties by default

- **WHEN** an edge is created without explicit properties
- **THEN** its properties map is empty

### Requirement: Edge key

Active edges in the materialized snapshot MUST be indexed by composite key `{target_id}{type}{source_id}`. The active edge for each identity MUST come from the winning edge version record.

#### Scenario: Key determines uniqueness

- **WHEN** two active edges share the same target, type, and source
- **THEN** the system treats them as the same edge and MUST NOT store duplicates

#### Scenario: Superseded edge versions inactive

- **WHEN** an edge identity has a winning tombstoned version
- **THEN** that edge is absent from the active view

### Requirement: Edge type enum

Edge types MUST be represented by a `#[repr(u8)]` enum. The v0 enum MUST define `Parent = 0`.

#### Scenario: Parent edge type value

- **WHEN** a parent edge exists
- **THEN** its edge type equals `EdgeType::Parent` (u8 value 0)

### Requirement: Parent edges

Hierarchy MUST be represented by edges with type `EdgeType::Parent`, where the source is the child block and the target is the parent block.

#### Scenario: Child linked to parent

- **WHEN** a block is created with a parent
- **THEN** a `parent` edge exists with source set to the child and target set to the parent

#### Scenario: Root block has no parent edge

- **WHEN** a block exists with no parent
- **THEN** no `parent` edge exists with that block as source

#### Scenario: At most one parent per block

- **WHEN** a block has a parent
- **THEN** exactly one `parent` edge exists with that block as source

### Requirement: Single root block

Each knowledge base MUST contain exactly one root block. The root block is the only block with no `parent` edge where it is the source.

#### Scenario: New graph has root block

- **WHEN** a new knowledge base is initialized
- **THEN** exactly one root block exists with an empty properties map
- **AND** no `parent` edge exists with the root block as source

#### Scenario: Root block is unique

- **WHEN** a knowledge base exists
- **THEN** exactly one block has no `parent` edge with itself as source

### Requirement: Versioned block and edge state

Block and edge structure in the active view MUST be determined by materializing version history at read time. The winning version record for each entity id defines the active block or edge.

#### Scenario: Active block from winning version

- **WHEN** version history contains multiple versions of a block id
- **THEN** the active block reflects the winning version's properties

#### Scenario: Tombstoned block absent

- **WHEN** the winning version of a block is tombstoned
- **THEN** that block does not appear in the active view

