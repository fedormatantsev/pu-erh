## 1. KnowledgeBase type

- [x] 1.1 Rename/refactor `Snapshot` to `KnowledgeBase` in `crates/graph/src/snapshot.rs` (or new module), keeping trie fields and read APIs
- [x] 1.2 Add `crdt_winner_block` / `crdt_winner_edge` (tombstones included) and clarify `active_*` vs CRDT winner
- [x] 1.3 Move `append_block_version`, `append_edge_version`, and `create_root_block_version` onto `KnowledgeBase` with trie-derived version/previous_digest
- [x] 1.4 Implement `KnowledgeBase::merge` (trie union by full key) replacing `merge_histories`
- [x] 1.5 Add trie iteration helpers to collect all block/edge version records for export

## 2. Remove VersionHistory

- [x] 2.1 Delete `VersionHistory` struct and vec-based winner/next methods from `crates/graph/src/version.rs`
- [x] 2.2 Remove `Snapshot::materialize` and `Snapshot::materialize_from`
- [x] 2.3 Update `crates/graph/src/lib.rs` exports (`KnowledgeBase`, optional `Snapshot` type alias)
- [x] 2.4 Add parity test: trie CRDT winner matches explicit max(version, digest) for sample histories

## 3. Storage layer

- [x] 3.1 Change `storage::load` to return `KnowledgeBase` (insert each record into tries)
- [x] 3.2 Change `storage::save` to accept `&KnowledgeBase` and export trie records to JSON arrays (stable key order)
- [x] 3.3 Update `merge_histories_from_paths` to use `KnowledgeBase::merge`
- [x] 3.4 Update storage tests for trie-only load/save/merge

## 4. Core session and mutations

- [x] 4.1 Refactor `Session` to hold `KnowledgeBase` only; remove `history`, `rematerialize`, and `history()`
- [x] 4.2 Update `mutation.rs` to take `&mut KnowledgeBase` instead of `(&mut VersionHistory, &Snapshot)`
- [x] 4.3 Update `query.rs` and session tests to use `KnowledgeBase`
- [x] 4.4 Fix `ensure_root` to check empty knowledge base instead of empty vec

## 5. Graph tests and cleanup

- [x] 5.1 Update `snapshot.rs`, `version.rs`, and graph integration tests to use `KnowledgeBase` directly
- [x] 5.2 Remove dead code and unused imports across graph/core/storage
- [x] 5.3 Run `cargo test` across workspace; fix any failures

## 6. Documentation

- [x] 6.1 Update README architecture section: trie-only in-memory store, JSON arrays as persistence envelope
