## MODIFIED Requirements

### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base by exporting all block and edge version records from the in-memory version tries into a single versioned JSON file (`format_version`, `block_versions`, `edge_versions`).

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with version records is saved and loaded
- **THEN** the set of version records is identical to before save
- **AND** active graph semantics after load match those before save

## ADDED Requirements

### Requirement: Load builds trie from file

When loading from storage, the system MUST construct the in-memory knowledge base by inserting each deserialized version record into the appropriate version trie. The system MUST NOT retain a separate in-memory vector of the same records alongside the trie.

#### Scenario: Missing file yields empty knowledge base

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the result is an empty knowledge base with no version records in either trie

#### Scenario: Save exports from trie

- **WHEN** a knowledge base is saved
- **THEN** block and edge version records are collected from trie iteration for JSON serialization
- **AND** no separate in-memory vector is the save source of truth
