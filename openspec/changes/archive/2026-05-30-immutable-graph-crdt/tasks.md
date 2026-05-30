## 1. Version record model (`graph` crate)

- [x] 1.1 Define `BlockVersion`, `EdgeVersion` with `version`, `digest` (32-byte BLAKE3), `previous_digest`, `tombstoned`
- [x] 1.2 Implement BLAKE3 in-place digest over in-memory record fields (fixed field order; no serialization)
- [x] 1.3 Define `VersionHistory` (append-only block/edge version vectors) and append API
- [x] 1.4 Add unit tests for digest stability and version chain linking

## 2. Read-time snapshot (`graph` crate)

- [x] 2.1 Implement winner selection: max version, max digest per entity id
- [x] 2.2 Implement `Snapshot::materialize(history)` with tombstone exclusion
- [x] 2.3 Implement invariant filtering (drop invalid blocks/edges silently)
- [x] 2.4 Port read API from `Graph` (`block`, `parent`, `children`, `root_id`)
- [x] 2.5 Add tests: linear history, branching, digest tie-break, invariant filtering

## 3. Version-history storage (`storage` crate)

- [x] 3.1 Replace snapshot JSON with `format_version`, `block_versions`, `edge_versions`
- [x] 3.2 Implement load/save and history union for replication merge
- [x] 3.3 Add round-trip and materialization parity tests

## 4. Session refactor (`pu-erh-core`)

- [x] 4.1 Refactor `Session` to hold `VersionHistory` and materialized `Snapshot`
- [x] 4.2 On open: load history, materialize snapshot; on new file: empty history
- [x] 4.3 On save: append root block v1 if history empty, persist history
- [x] 4.4 Update session tests

## 5. Mutations refactor (`pu-erh-core`)

- [x] 5.1 Refactor `create_block` to append block + edge version records
- [x] 5.2 Refactor `move_block` to append tombstone + new edge version records
- [x] 5.3 Refactor `delete_block` to append tombstoned block version record
- [x] 5.4 Update mutation tests; failed mutations append nothing

## 6. Query and CLI integration

- [x] 6.1 Update query executor to read from `Snapshot`
- [x] 6.2 Update CLI smoke test and `show-root` example for version-history format
- [x] 6.3 Update README with version-history storage and breaking format change

## 7. Verification

- [x] 7.1 Run `cargo test` — all tests pass
- [x] 7.2 Manual walkthrough: new kb → create → query → move → delete → reload and verify history persists
