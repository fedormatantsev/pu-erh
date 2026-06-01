## Why

The knowledge base is persisted as one large JSON snapshot. That makes diffs noisy, complicates partial inspection, and couples every version record to a single parse/write cycle. Splitting records into TOML files under `blocks/` and `edges/` improves human readability, aligns with one-file-per-version-record storage, and keeps the on-disk layout closer to the trie’s logical objects.

## What Changes

- **BREAKING**: Replace the monolithic JSON file (`format_version`, `block_versions`, `edge_versions`) with a **storage directory** containing `blocks/` and `edges/` subdirectories.
- Each block version record is written as one TOML file under `blocks/`; each edge version record as one TOML file under `edges/`.
- Serialization uses TOML instead of JSON; digest encoding and `PropertyValue` rules remain those of storage format version 2 (binary property digests).
- Bump on-disk `format_version` to **3** for the directory layout.
- `load` / `save` paths refer to the storage **directory** (missing directory → empty knowledge base; parent directories created on save).
- Desktop default path changes from `{app_data_dir}/pu-erh/kb.json` to `{app_data_dir}/pu-erh/kb/` (directory).
- CLI `--file` examples and tests updated to use a directory path; flag name unchanged.

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

- `storage`: Directory layout, per-record TOML files, format version 3, load/save semantics.
- `cli`: `--file` points at a storage directory; scenario paths use a directory instead of `kb.json`.
- `desktop-shell`: Deterministic storage path is a directory under app data.
- `session`: Wording and scenarios refer to a storage directory rather than a single file where persistence is described.

## Impact

- `crates/storage`: Replace `serde_json` with `toml`; implement directory traversal, per-file naming, and format-version manifest at storage root.
- `crates/graph`: No domain logic change; `BlockVersion` / `EdgeVersion` remain serde types.
- `crates/core`, `crates/cli`, `crates/desktop`: Default paths and tests.
- `README.md`, examples, and storage integration tests.
