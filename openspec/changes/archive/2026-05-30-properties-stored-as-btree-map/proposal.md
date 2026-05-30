## Why

Block and edge `properties` are currently stored as `HashMap<String, serde_json::Value>`, so digest computation must allocate a temporary `BTreeMap` on every hash to sort keys before feeding them to BLAKE3. That extra sort step is redundant work and separates the in-memory representation from the documented digest field order ("properties, keys sorted, then values"). Storing properties in a `BTreeMap` aligns storage with digest iteration, removes per-hash sorting overhead, and yields stable key order when properties are serialized (e.g. CLI output).

## What Changes

- Change the `Properties` type alias from `HashMap<String, serde_json::Value>` to `BTreeMap<String, serde_json::Value>`.
- Simplify `hash_properties` in `digest.rs` to iterate properties in native key order without collecting into a temporary map.
- Update imports and any code that relied on `HashMap`-specific APIs (if any); `BTreeMap` supports the same insert/get/iter patterns used today.
- Add tests confirming digest stability is unchanged and that property key order in hashing matches lexicographic order regardless of insertion order.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `block-model`: Properties maps on blocks and edges are ordered maps with lexicographic string key ordering.
- `version-history`: Digest computation over properties explicitly hashes keys in lexicographic order as part of the in-memory field order.

## Impact

- **`crates/graph`**: `model.rs` (`Properties` type), `digest.rs` (simplified hashing), possible test updates in `digest.rs`, `snapshot.rs`, `version.rs`.
- **`crates/core`**, **`crates/storage`**, **`crates/cli`**: No API changes; callers using `Properties::new()` and standard map operations continue to work. CLI JSON output for block properties will use stable lexicographic key order.
- **No storage format change**: Version history persistence is unchanged; only the in-memory and serialized JSON ordering of property keys is affected.
- **No digest change**: Existing digests remain valid because hashing already sorted keys; behavior is preserved, representation is simplified.
