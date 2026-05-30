## MODIFIED Requirements

### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base by exporting all block and edge version records from the in-memory version tries into a single versioned JSON file (`format_version`, `block_versions`, `edge_versions`). Saved version records MUST use digests computed with the binary `PropertyValue` digest encoding (format version 2).

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with version records is saved and loaded
- **THEN** the set of version records is identical to before save
- **AND** active graph semantics after load match those before save

## ADDED Requirements

### Requirement: Storage format version 2

The system MUST use `format_version` `2` for knowledge-base files written after this change. Version 2 digests use binary-encoded property values; version 1 files used JSON-encoded property value bytes in digests and are not compatible.

#### Scenario: Version 1 file rejected

- **WHEN** storage is loaded from a file with `format_version` `1`
- **THEN** the system returns an error describing unsupported format version
- **AND** does not partially construct a knowledge base

#### Scenario: Version 2 file saves and loads

- **WHEN** a knowledge base is saved under format version 2 and loaded again
- **THEN** all version records pass digest verification
- **AND** property values round-trip as `PropertyValue` variants
