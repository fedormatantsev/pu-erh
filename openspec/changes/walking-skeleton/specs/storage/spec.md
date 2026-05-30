## ADDED Requirements

### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base as a single JSON file containing all blocks and edges.

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with blocks and edges is saved to a file and then loaded
- **THEN** all block ids and properties, and all edge sources, targets, types, and properties are identical to before save

### Requirement: Load missing file as new graph

When loading from a path that does not exist, the system MUST initialize a new knowledge base with exactly one root block rather than returning an error.

#### Scenario: First use of new path

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the result is a graph containing exactly one root block and no edges

### Requirement: Reject invalid storage

When loading from a path that exists but contains invalid JSON or an invalid knowledge base structure, the system MUST return an error.

#### Scenario: Corrupt file

- **WHEN** storage is loaded from a file containing malformed JSON
- **THEN** the system returns an error describing the load failure
