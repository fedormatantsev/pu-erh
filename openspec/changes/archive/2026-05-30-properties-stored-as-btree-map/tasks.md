## 1. Type change

- [x] 1.1 Change `Properties` type alias in `crates/graph/src/model.rs` from `HashMap` to `BTreeMap<String, serde_json::Value>`
- [x] 1.2 Update imports in `model.rs` (replace `HashMap` with `BTreeMap`)

## 2. Digest simplification

- [x] 2.1 Simplify `hash_properties` in `crates/graph/src/digest.rs` to iterate `properties` directly without collecting into a temporary `BTreeMap`
- [x] 2.2 Remove unused `BTreeMap` import from `digest.rs` if no longer needed

## 3. Tests and verification

- [x] 3.1 Add test in `digest.rs` that properties inserted in different orders produce identical digests
- [x] 3.2 Run `cargo test` across the workspace and fix any failures from the type change
