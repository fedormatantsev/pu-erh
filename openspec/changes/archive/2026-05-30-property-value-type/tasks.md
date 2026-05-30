## 1. PropertyValue type

- [x] 1.1 Add `PropertyValue` enum (`String`, `Number(f64)`, `Boolean`, `Null`) with `Serialize`/`Deserialize` accepting JSON string, number, boolean, and `null` only
- [x] 1.2 Change `Properties` to `BTreeMap<String, PropertyValue>` and re-export `PropertyValue` from `graph`
- [x] 1.3 Implement `digest_bytes()` per spec (tags 0–3, LE `u64` length + UTF-8 for strings, LE `f64` for numbers) with unit tests for each variant

## 2. Digest and version records

- [x] 2.1 Update `hash_properties` in `digest.rs` to use `digest_bytes()`; remove `PropertyNotSerializable` (encoding is infallible)
- [x] 2.2 Update `version.rs` / `snapshot.rs` expect messages and tests to use `PropertyValue` instead of `serde_json::json!`
- [x] 2.3 Update digest tests for binary encoding (stability, insertion-order independence); expect digests to differ from pre-change JSON encoding

## 3. Downstream crates

- [x] 3.1 Remove `serde_json` from `graph/Cargo.toml` entirely
- [x] 3.2 Remove `serde_json` from `core` and update any property usage
- [x] 3.3 Update `cli` query output to format `Properties` without `serde_json::Value` (graph helper or `Serialize` on map)
- [x] 3.4 Bump `FORMAT_VERSION` to `2` in `storage`; reject v1 on load; verify `storage` is the only crate calling `serde_json::from_str` / `to_string_pretty` on the file

## 4. Validation

- [x] 4.1 Add test that storage load rejects property JSON arrays and objects; `null` round-trips as `PropertyValue::Null`
- [x] 4.2 Add test that `format_version` 1 files are rejected
- [x] 4.3 Run `cargo test --workspace` and fix any failures; re-init local knowledge-base JSON files as needed
