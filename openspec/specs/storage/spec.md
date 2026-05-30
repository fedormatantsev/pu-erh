# storage Specification

## Purpose

Defines JSON persistence of version records to disk: format version, trie round-trip, missing-file behavior, digest verification on load, and invalid-file rejection.
## Requirements
### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base by exporting all block and edge version records from the in-memory version tries into a single versioned JSON file (`format_version`, `block_versions`, `edge_versions`).

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with version records is saved and loaded
- **THEN** the set of version records is identical to before save
- **AND** active graph semantics after load match those before save

### Requirement: Load builds trie from file

When loading from storage, the system MUST construct the in-memory knowledge base by inserting each deserialized version record into the appropriate version trie. The system MUST NOT retain a separate in-memory vector of the same records alongside the trie.

When loading from a path that does not exist, the system MUST initialize an empty knowledge base (no version records in either trie) rather than returning an error. The root block is not created at load time; it is inserted on first save via the session `ensure_root` path.

#### Scenario: Missing file yields empty knowledge base

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the result is an empty knowledge base with no version records in either trie
- **AND** no root block exists in the active view until first save

#### Scenario: Save exports from trie

- **WHEN** a knowledge base is saved
- **THEN** block and edge version records are collected from trie iteration for JSON serialization
- **AND** no separate in-memory vector is the save source of truth

#### Scenario: Save creates parent directories

- **WHEN** a knowledge base is saved to a path whose parent directory does not exist
- **THEN** the system creates parent directories before writing the file

### Requirement: Reject invalid storage

When loading from a path that exists but contains invalid JSON, an unsupported format version, records that cannot be parsed, or version records whose digests fail verification, the system MUST return an error.

#### Scenario: Corrupt file

- **WHEN** storage is loaded from a file containing malformed JSON
- **THEN** the system returns an error describing the load failure

#### Scenario: Digest mismatch

- **WHEN** storage is loaded from a file containing a version record whose stored digest does not match recomputation
- **THEN** the system returns an error describing the digest mismatch

#### Scenario: Unsupported format version

- **WHEN** storage is loaded from a file with an unsupported `format_version`
- **THEN** the system returns an error describing the version mismatch

