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

Each edge MUST have a source id, a target id, an edge type string, and a properties map (generic key/value dictionary).

#### Scenario: Edge fields are accessible

- **WHEN** an edge exists in the graph
- **THEN** its source, target, edge type, and properties are readable

#### Scenario: New edge has empty properties by default

- **WHEN** an edge is created without explicit properties
- **THEN** its properties map is empty

### Requirement: Edge key

Edges MUST be indexed by a composite key of `{target_id}{type}{source_id}` to support prefix lookup of all edges pointing to a given target.

#### Scenario: Key determines uniqueness

- **WHEN** two edges share the same target, type, and source
- **THEN** the system treats them as the same edge and MUST NOT store duplicates

### Requirement: Parent edges

Hierarchy MUST be represented by edges with type `"parent"`, where the source is the child block and the target is the parent block.

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

