## MODIFIED Requirements

### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base as a single versioned JSON file containing all block and edge version records (`format_version`, `block_versions`, `edge_versions`).

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with version history is saved and loaded
- **THEN** all version records are identical to before save
- **AND** materializing after load produces the same snapshot as before save

### Requirement: Load missing file as new graph

When loading from a path that does not exist, the system MUST initialize an empty version history rather than returning an error.

#### Scenario: First use of new path

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the result is an empty version history with no records

### Requirement: Reject invalid storage

When loading from a path that exists but contains invalid JSON, an unsupported format version, or records that cannot be parsed, the system MUST return an error.

#### Scenario: Corrupt file

- **WHEN** storage is loaded from a file containing malformed JSON
- **THEN** the system returns an error describing the load failure

#### Scenario: Unsupported format version

- **WHEN** storage is loaded from a file with an unsupported `format_version`
- **THEN** the system returns an error describing the version mismatch
