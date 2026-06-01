## 1. Storage crate — format version 3 layout

- [x] 1.1 Replace `serde_json` with `toml` in `crates/storage/Cargo.toml`; set `FORMAT_VERSION` to `3`
- [x] 1.2 Add helpers: trie-key hex filename, `format_version.toml` read/write, enumerate `blocks/*.toml` and `edges/*.toml`
- [x] 1.3 Implement `load`: missing directory → empty KB; reject if path is a regular file; read manifest + all record files; digest verify; trie insert
- [x] 1.4 Implement `save`: write manifest, write all record files, delete stale `*.toml` in `blocks/` and `edges/`
- [x] 1.5 Remove `KnowledgeBaseFile` monolith type (or restrict to tests only if still needed)
- [x] 1.6 Rewrite `crates/storage` tests for directory paths: round-trip, merge, corrupt TOML, digest mismatch, unsupported format version, legacy JSON file rejection, property round-trip / rejection scenarios

## 2. Callers and integration

- [x] 2.1 Update `crates/desktop/src/state.rs`: `kb_path` → `.../pu-erh/kb/`; fix unit tests
- [x] 2.2 Update `crates/core` session tests and `examples/show_root.rs` to use a storage directory
- [x] 2.3 Update `crates/cli/tests/smoke.rs` and any CLI help text referencing `kb.json`
- [x] 2.4 Update `README.md` with directory layout and breaking-change note

## 3. Verification

- [x] 3.1 Run `cargo test -p storage -p core -p cli -p desktop`
- [x] 3.2 Run `openspec validate storage-toml-per-object` (or project validation command if defined)
