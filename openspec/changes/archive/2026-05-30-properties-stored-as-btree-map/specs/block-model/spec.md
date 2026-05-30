## MODIFIED Requirements

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
