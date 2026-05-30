## Why

Block and edge properties are typed as `serde_json::Value`, which admits arrays and objects (nested composites) the product does not intend to support, while obscuring the intended scalar + null model. That widens the in-memory surface area, forces digest hashing through generic JSON serialization, and spreads `serde_json` through graph, core, and CLI code. A narrow `PropertyValue` enum makes the supported scalar types explicit, keeps domain logic free of `serde_json::Value`, and confines JSON parsing to the storage file boundary.

## What Changes

- Introduce a `PropertyValue` enum in the graph crate with variants `String`, `Number` (`f64`), `Boolean`, and `Null`.
- Change `Properties` from `BTreeMap<String, serde_json::Value>` to `BTreeMap<String, PropertyValue>`.
- Remove `serde_json::Value` (and `serde_json` dependency) from `graph`, `core`, and `cli`; use `PropertyValue` APIs and serde derives on the enum instead.
- Keep `serde_json` only in `storage` for reading and writing the knowledge-base JSON file (`from_str` / `to_string_pretty`).
- Replace JSON-encoded property bytes in digest hashing with a fixed binary encoding for `PropertyValue` (type tag + payload; see version-history spec).
- Bump persisted `format_version` to `2`; reject v1 knowledge-base files at load.
- **BREAKING**: All version-record digests change; v1 storage files are unsupported.
- **BREAKING**: Loading a storage file whose property values are JSON arrays or objects MUST fail with a clear error (string, number, boolean, and null are accepted).

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `block-model`: Property values are `PropertyValue` (string, number, boolean, null), not arbitrary JSON.
- `version-history`: Digest property encoding uses binary `PropertyValue` bytes; array/object JSON in persisted files is rejected at load.
- `storage`: `format_version` 2; v1 files rejected.
- `cli`: Block property display serializes `PropertyValue` maps without `serde_json::Value`.

## Impact

- **`crates/graph`**: New `property_value.rs` (or module in `model.rs`), `Properties` type alias, `digest.rs` hashing, `version.rs` / `snapshot.rs` tests, remove `serde_json` from `Cargo.toml`.
- **`crates/core`**, **`crates/cli`**: Replace `serde_json::json!` and `serde_json::to_string` with `PropertyValue` constructors / display helpers; drop `serde_json` from `core` and `cli` if no longer needed.
- **`crates/storage`**: Remains the only crate that calls `serde_json::from_str` / `to_string_pretty`; deserialization errors surface unsupported property JSON types.
- **On-disk format**: `format_version` 2; JSON property values still string/number/boolean/null in the file body; digests use binary encoding. v1 files and files with array/object property values no longer load.
