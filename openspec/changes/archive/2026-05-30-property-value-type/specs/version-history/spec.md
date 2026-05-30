## MODIFIED Requirements

### Requirement: Digest computation

Each version record MUST include a `digest` computed as a BLAKE3 hash over the record's in-memory fields using in-place incremental hashing. Field values MUST be fed to the hasher in a fixed, documented order. Full-record JSON serialization MUST NOT be used as an intermediate step when computing digests. Property keys MUST be hashed in lexicographic order; each key MUST be followed by that property's binary-encoded `PropertyValue` bytes (see Property value digest encoding).

#### Scenario: Same in-memory content produces same digest

- **WHEN** two records have identical in-memory field values hashed in the defined order
- **THEN** their `digest` values are equal

#### Scenario: Digest is stable across hash invocations

- **WHEN** the same record is hashed multiple times
- **THEN** the resulting `digest` is identical each time

#### Scenario: Property insertion order does not affect digest

- **WHEN** two property maps contain the same keys and values but entries were inserted in different orders
- **THEN** hashing either map produces the same digest contribution

### Requirement: Digest field order

Digest computation MUST hash field values in the following fixed order using BLAKE3 incremental hashing.

**Block version record hash input order:**

1. `id` — 16 raw UUID bytes
2. `version` — u64 little-endian (8 bytes)
3. `tombstoned` — single byte (`0` or `1`)
4. `properties` — for each key in lexicographic order: key UTF-8 bytes, then binary-encoded `PropertyValue` bytes

**Edge version record hash input order:**

1. `source` — 16 raw UUID bytes
2. `target` — 16 raw UUID bytes
3. `edge_type` — single u8 byte
4. `version` — u64 little-endian (8 bytes)
5. `tombstoned` — single byte (`0` or `1`)
6. `properties` — for each key in lexicographic order: key UTF-8 bytes, then binary-encoded `PropertyValue` bytes

Trie CRDT key suffixes encode `version` as **big-endian** u64 for lexicographic ordering; digest hashing uses **little-endian** u64 as specified above. These encodings serve different purposes and MUST NOT be conflated.

#### Scenario: Block digest matches field order

- **WHEN** a block version record is hashed
- **THEN** the digest equals BLAKE3 over the block field order specified above

#### Scenario: Edge digest matches field order

- **WHEN** an edge version record is hashed
- **THEN** the digest equals BLAKE3 over the edge field order specified above

## ADDED Requirements

### Requirement: Property value digest encoding

Each `PropertyValue` MUST have a fixed binary encoding used only for digest hashing. Encoding MUST start with a one-byte type tag, followed by the payload for that tag:

| Tag (u8) | Variant | Payload |
|----------|---------|---------|
| `0` | `Null` | (none) |
| `1` | `Boolean` | 1 byte: `0` = false, `1` = true |
| `2` | `Number` | 8 bytes: IEEE 754 `f64` little-endian |
| `3` | `String` | `u64` little-endian UTF-8 byte length, then UTF-8 bytes |

Digest computation MUST use only these encodings for property values. JSON text MUST NOT be used for property value bytes in digests.

#### Scenario: Null encodes as tag only

- **WHEN** a property value is `PropertyValue::Null`
- **THEN** its digest bytes are the single byte `0x00`

#### Scenario: Boolean encodes as tag and byte

- **WHEN** a property value is `PropertyValue::Boolean(true)`
- **THEN** its digest bytes are `0x01 0x01`
- **AND** `PropertyValue::Boolean(false)` encodes as `0x01 0x00`

#### Scenario: Number encodes as tag and f64 LE

- **WHEN** a property value is `PropertyValue::Number(n)` for a given `f64` `n`
- **THEN** its digest bytes are `0x02` followed by the 8-byte little-endian IEEE 754 representation of `n`

#### Scenario: String encodes as tag length and UTF-8

- **WHEN** a property value is `PropertyValue::String(s)`
- **THEN** its digest bytes are `0x03` followed by `u64` LE byte length of `s` UTF-8, followed by those UTF-8 bytes

### Requirement: Property value type

Property values in version records and the active view MUST be represented by a `PropertyValue` enum with variants for string (`String`), number (`f64`), boolean (`bool`), and null (`Null`). The in-memory graph model MUST NOT use `serde_json::Value` for properties.

#### Scenario: Supported JSON values deserialize

- **WHEN** a property value in persisted JSON is a string, number, boolean, or `null`
- **THEN** it deserializes to the corresponding `PropertyValue` variant

#### Scenario: Nested composite JSON rejected

- **WHEN** a property value in persisted JSON is an array or an object
- **THEN** loading the knowledge base fails with an error describing the unsupported value
