## MODIFIED Requirements

### Requirement: Serialize knowledge base to file

The system MUST persist a knowledge base by exporting all block and edge version records from the in-memory version tries into a **storage directory** with:

- a root manifest `format_version.toml` declaring `format_version` `3`;
- one TOML file per block version record under `blocks/`;
- one TOML file per edge version record under `edges/`.

Each record file MUST contain the serde fields of `BlockVersion` or `EdgeVersion` respectively. Saved version records MUST use digests computed with the binary `PropertyValue` digest encoding (same algorithm as storage format version 2). Record filenames MUST be the lowercase hexadecimal encoding of that record's trie key (`block_version_key_from` / `edge_version_key_from`).

On save, the system MUST remove stale `*.toml` files in `blocks/` and `edges/` that are not part of the current export set.

#### Scenario: Round-trip preservation

- **WHEN** a knowledge base with version records is saved and loaded
- **THEN** the set of version records is identical to before save
- **AND** active graph semantics after load match those before save

#### Scenario: Per-record files exist after save

- **WHEN** a knowledge base with N block version records and M edge version records is saved
- **THEN** `blocks/` contains N `*.toml` files and `edges/` contains M `*.toml` files
- **AND** `format_version.toml` declares version 3

### Requirement: Load builds trie from file

When loading from storage, the system MUST construct the in-memory knowledge base by reading each version record TOML file under `blocks/` and `edges/`, deserializing it, verifying its digest, and inserting it into the appropriate version trie. The system MUST NOT retain a separate in-memory vector of the same records alongside the trie.

When loading from a **storage directory path** that does not exist, the system MUST initialize an empty knowledge base (no version records in either trie) rather than returning an error. The root block is not created at load time; it is inserted on first save via the session `ensure_root` path.

#### Scenario: Missing directory yields empty knowledge base

- **WHEN** storage is loaded from a path that does not exist
- **THEN** the result is an empty knowledge base with no version records in either trie
- **AND** no root block exists in the active view until first save

#### Scenario: Save exports from trie

- **WHEN** a knowledge base is saved
- **THEN** block and edge version records are collected from trie iteration for TOML serialization
- **AND** no separate in-memory vector is the save source of truth

#### Scenario: Save creates parent directories

- **WHEN** a knowledge base is saved to a path whose parent directory does not exist
- **THEN** the system creates parent directories before writing files

### Requirement: Reject invalid storage

When loading from a storage directory that exists but contains an invalid manifest, malformed TOML, an unsupported format version, records that cannot be parsed, or version records whose digests fail verification, the system MUST return an error.

#### Scenario: Corrupt record file

- **WHEN** storage is loaded from a directory containing a malformed TOML file under `blocks/` or `edges/`
- **THEN** the system returns an error describing the load failure

#### Scenario: Digest mismatch

- **WHEN** storage is loaded from a directory containing a version record whose stored digest does not match recomputation
- **THEN** the system returns an error describing the digest mismatch

#### Scenario: Unsupported format version

- **WHEN** storage is loaded from a directory whose `format_version.toml` declares an unsupported `format_version`
- **THEN** the system returns an error describing the version mismatch

## ADDED Requirements

### Requirement: Storage format version 3

The system MUST use `format_version` `3` for knowledge bases written after this change. Format version 3 stores per-record TOML files under `blocks/` and `edges/` with binary `PropertyValue` digest encoding.

#### Scenario: Version 3 directory saves and loads

- **WHEN** a knowledge base is saved under format version 3 and loaded again
- **THEN** all version records pass digest verification
- **AND** property values round-trip as `PropertyValue` variants
