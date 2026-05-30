# block-model Specification

## Purpose

Defines the logical block and edge model for pu-erh: UUID identity, property maps, parent-edge hierarchy, single root invariant, and how active entities are derived from version history at read time.
## Requirements
### Requirement: Block identity

Each block in the knowledge base MUST have a unique identifier represented as a UUID. Newly created blocks MUST receive a newly generated UUID v4.

#### Scenario: New block receives UUID

- **WHEN** a block is created
- **THEN** the system assigns a newly generated UUID v4 as its id
- **AND** no two blocks in the same knowledge base share the same id

#### Scenario: Persisted ids accepted on load

- **WHEN** a block version record is loaded from storage
- **THEN** its `id` is accepted regardless of UUID variant
- **AND** only newly generated ids are required to be UUID v4

### Requirement: Block structure

Each block MUST contain an id and a properties map (generic key/value dictionary with string keys and JSON-compatible values, stored in lexicographic key order). Blocks MUST NOT have a type field or embed parent or other relation fields — relations are represented as edges.

#### Scenario: Block fields are accessible

- **WHEN** a block exists in the graph
- **THEN** its id and properties are readable

#### Scenario: New block has empty properties by default

- **WHEN** a block is created without explicit properties
- **THEN** its properties map is empty

### Requirement: Edge structure

Each edge MUST have a source id, a target id, an edge type represented as a `#[repr(u8)]` enum, and a properties map (generic key/value dictionary with string keys and JSON-compatible values, stored in lexicographic key order).

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

Active edges in the materialized snapshot MUST be indexed by composite key `{target_id}{type}{source_id}` (33 raw bytes: 16-byte target UUID, 1-byte edge type, 16-byte source UUID). Edge identity for CRDT indexing matches this key order; see version-history for record field layout. The active edge for each identity MUST come from the winning edge version record.

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

- **WHEN** a block has a parent in the active view after a single-writer mutation sequence
- **THEN** exactly one `parent` edge exists with that block as source

#### Scenario: Multiple parent edges after replication merge

- **WHEN** replication merge produces multiple active `parent` edges with the same source and different targets
- **THEN** the active view MAY contain more than one parent edge for that block
- **AND** read-time repair of parent uniqueness is not performed

### Requirement: Single root block

Each knowledge base MUST contain exactly one root block. The root block is the only block with no `parent` edge where it is the source.

#### Scenario: New graph has root block after first save

- **WHEN** a new knowledge base is first persisted (via `ensure_root` or first save on an empty knowledge base)
- **THEN** exactly one root block exists in the active view with an empty properties map
- **AND** no `parent` edge exists with the root block as source

#### Scenario: New knowledge base is empty before first save

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the in-memory knowledge base has no version records and no root block until first save

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

