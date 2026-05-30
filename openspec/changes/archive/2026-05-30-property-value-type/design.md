## Context

The graph crate defines `Properties` as `BTreeMap<String, serde_json::Value>`. Version records, digest hashing, snapshots, CLI formatting, and storage round-trip all flow through that type. Digest hashing calls `serde_json::to_vec` per property value. The storage crate is the only place that should touch raw JSON text, but today `serde_json` is also a direct dependency of `graph`, `core`, and `cli`.

## Goals / Non-Goals

**Goals:**

- Introduce `PropertyValue` with `String`, `Number(f64)`, `Boolean`, and `Null` variants.
- Use `BTreeMap<String, PropertyValue>` as `Properties` everywhere in the in-memory model.
- Remove `serde_json` from `graph`, `core`, and `cli`; keep it only in `storage` for file I/O.
- Hash property values with a fixed binary encoding (type tag + payload), not JSON text.
- Reject nested composite JSON (arrays and objects) at deserialization with actionable errors.

**Non-Goals:**

- Property schema validation, required keys, or typed property names per block kind.
- Arrays or objects as property values (nested composite properties are not supported).
- Changing the top-level knowledge-base JSON field layout (`block_versions`, `edge_versions`, record fields).
- Query-language or mutation API for setting typed properties (callers use Rust constructors only for now).

## Decisions

### 1. `PropertyValue` lives in `crates/graph`

**Choice:** Define `PropertyValue` in the graph crate (e.g. `property_value.rs`), re-export from `lib.rs`, and alias `Properties = BTreeMap<String, PropertyValue>`.

**Rationale:** Properties are part of the domain model shared by versions, snapshots, and digests. Storage and CLI depend on graph types already.

**Alternative:** Separate tiny `property` crate — rejected as premature for four variants.

### 2. Serde on `PropertyValue`, not `serde_json::Value`

**Choice:** `#[derive(Serialize, Deserialize)]` on `PropertyValue` with an `untagged` enum or custom deserializer that accepts JSON string, number, boolean, and `null` tokens only.

**Rationale:** `KnowledgeBaseFile` and version records deserialize through `serde` while `storage` uses `serde_json::from_str` / `to_string_pretty`. No `serde_json::Value` in graph; unsupported tokens fail at deserialize.

**Alternative:** Storage DTO with `serde_json::Value` then convert — rejected; user asked to drop `JsonValue` except at fs boundary, and derive-on-enum keeps conversion implicit at that boundary.

### 3. Binary property-value encoding for digests

**Choice:** Replace JSON-encoded property bytes in `hash_properties` with a fixed binary layout via `PropertyValue::digest_bytes()`:

| Tag (u8) | Variant | Payload |
|----------|---------|---------|
| `0` | `Null` | (none) |
| `1` | `Boolean` | 1 byte: `0` = false, `1` = true |
| `2` | `Number` | 8 bytes: IEEE 754 `f64` little-endian |
| `3` | `String` | `u64` LE byte length, then UTF-8 bytes |

For each property entry (keys iterated in lexicographic order): feed key UTF-8 bytes, then the value's `digest_bytes()` payload to the BLAKE3 hasher. No JSON serialization anywhere in digest computation.

**Rationale:** Removes `serde_json` from digest path entirely, avoids float/string formatting ambiguities from JSON text, and ties digests to the typed `PropertyValue` model.

**Alternative:** Preserve JSON-encoded digest bytes for backward compatibility — rejected; user requested new binary encoding.

**Breaking:** All version-record digests computed under the old JSON-value encoding are invalid. Bump `FORMAT_VERSION` to `2` so v1 files are rejected at load with an explicit version error (no silent digest mismatch on mixed semantics).

### 4. `DigestError` wording

**Choice:** Rename `PropertyNotSerializable` to something like `PropertyEncoding` or remove the error variant if encoding is infallible for `PropertyValue`.

**Rationale:** All in-memory values are always encodable once typed.

### 5. CLI property display

**Choice:** Serialize properties map to a compact JSON object string via `serde_json` in CLI **or** a small `Display`/`to_json_string` on `Properties` in graph without exposing `Value`.

**Rationale:** Current CLI uses `serde_json::to_string(&block.properties)`. Simplest path: `serde_json::to_string` in CLI on types that implement `Serialize` — but that re-adds `serde_json` to CLI. Better: graph provides `properties_to_json_string(&Properties) -> String` using the same serde derive, and CLI calls that without depending on `serde_json` directly.

**Choice (refined):** **graph** provides `Properties::to_json_string()` (or equivalent) using `PropertyValue`'s `Serialize` impl so CLI/core never depend on `serde_json::Value`.

### 6. Crate dependency graph

| Crate    | `serde_json` after change                                      |
|----------|----------------------------------------------------------------|
| graph    | No — digest uses binary encoding only; no `Value`              |
| storage  | Yes — `from_str` / `to_string_pretty` on file                  |
| core     | No                                                             |
| cli      | No — use graph helpers or `Serialize` via graph re-export      |

## Risks / Trade-offs

- **[Risk] All existing digests invalid** → Mitigation: Bump `format_version` to `2`; reject v1 files at load; document that dev knowledge bases must be re-initialized or re-saved after implementation.

- **[Risk] `f64` bitwise equality in digests** → Mitigation: Digest hashes the raw 8-byte LE representation after JSON deserialize; `-0.0` vs `0.0` and NaN payload bits are distinguished as stored in `PropertyValue`.

- **[Risk] Existing files with array/object properties** → Mitigation: Deserialize fails with clear error. Files using JSON `null` continue to load as `PropertyValue::Null` under v2.

- **[Risk] String keys with invalid UTF-8** → Mitigation: `Properties` keys remain `String` (UTF-8); digest uses `key.as_bytes()` as today.

## Migration Plan

1. Implement `PropertyValue` and switch `Properties` alias.
2. Update digest, version records, tests.
3. Remove `serde_json::Value` usages in graph/core/cli tests (`json!` → `PropertyValue::Number(1.0)` etc.).
4. Bump `FORMAT_VERSION` to `2` in storage; update tests.
5. Run full test suite; re-init or regenerate any local knowledge-base JSON files (v1 digests are not migrated).
6. No automatic migration for files with array/object property values; users must fix data or accept load failure.

## Open Questions

- None.
