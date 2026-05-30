## MODIFIED Requirements

### Requirement: Block structure

Each block MUST contain an id and a properties map (string keys in lexicographic order, values as `PropertyValue`: string, number (`f64`), boolean, or null). Blocks MUST NOT have a type field or embed parent or other relation fields — relations are represented as edges.

#### Scenario: Block fields are accessible

- **WHEN** a block exists in the graph
- **THEN** its id and properties are readable

#### Scenario: New block has empty properties by default

- **WHEN** a block is created without explicit properties
- **THEN** its properties map is empty

#### Scenario: Property value is a supported type

- **WHEN** a property is set on a block in memory
- **THEN** its value is one of `PropertyValue::String`, `PropertyValue::Number`, `PropertyValue::Boolean`, or `PropertyValue::Null`

### Requirement: Edge structure

Each edge MUST have a source id, a target id, an edge type represented as a `#[repr(u8)]` enum, and a properties map (string keys in lexicographic order, values as `PropertyValue`: string, number (`f64`), boolean, or null).

#### Scenario: Edge fields are accessible

- **WHEN** an edge exists in the graph
- **THEN** its source, target, edge type, and properties are readable

#### Scenario: Edge type is u8-backed

- **WHEN** an edge exists in the graph
- **THEN** its edge type is one of the defined `EdgeType` variants serialized as a u8

#### Scenario: New edge has empty properties by default

- **WHEN** an edge is created without explicit properties
- **THEN** its properties map is empty

#### Scenario: Property value is a supported type

- **WHEN** a property is set on an edge in memory
- **THEN** its value is one of `PropertyValue::String`, `PropertyValue::Number`, `PropertyValue::Boolean`, or `PropertyValue::Null`
