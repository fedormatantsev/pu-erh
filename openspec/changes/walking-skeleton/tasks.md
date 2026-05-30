## 1. Project setup

- [ ] 1.1 Create Rust workspace with four crates: `graph`, `storage`, `core`, `cli`
- [ ] 1.2 Configure crate dependencies: `cli` → `core`; `core` → `graph`, `storage`; `storage` → `graph`
- [ ] 1.3 Add dependencies per crate: `uuid`, `serde`/`serde_json`, `clap`, `thiserror` (or `anyhow` in `cli`)
- [ ] 1.4 Configure workspace `Cargo.toml` and verify `cargo build` succeeds

## 2. Block and edge model (`graph` crate)

- [ ] 2.1 Define `Properties` type and `Block` struct (id, properties) and `Edge` struct (source, target, edge_type, properties) with serde support
- [ ] 2.2 Define `EdgeKey` as composite `{target_id}{type}{source_id}` and `Graph` with `HashMap<Uuid, Block>` + `HashMap<EdgeKey, Edge>`; implement `Graph::new()` that creates the single root block
- [ ] 2.3 Implement edge insert/remove and prefix lookup by target id + type (for children)
- [ ] 2.4 Implement parent lookup via edge with type `parent` and matching source id
- [ ] 2.5 Add unit tests for blocks, edges, children (prefix scan), and parent lookup

## 3. Storage (`storage` crate)

- [ ] 3.1 Define on-disk JSON format (e.g. `{ "blocks": [...], "edges": [...] }`) and serialize/deserialize functions
- [ ] 3.2 Implement load: missing file → new graph with root block; invalid JSON → error
- [ ] 3.3 Implement save: write all blocks and edges to file
- [ ] 3.4 Add round-trip integration test (save → load → assert equality)

## 4. Session (`core` crate)

- [ ] 4.1 Implement `Session` struct holding graph, file path, and dirty flag
- [ ] 4.2 Implement `Session::open(path)` — load from storage or initialize graph with root block
- [ ] 4.3 Implement `Session::save()` — persist when dirty
- [ ] 4.4 Add unit/integration tests for session load, mutate, save cycle

## 5. Query language (`core` crate)

- [ ] 5.1 Implement query parser: dispatch on `parent:` and `children:` prefix, parse UUID suffix
- [ ] 5.2 Implement query executor using graph edge lookups (return blocks or errors for not-found / invalid syntax)
- [ ] 5.3 Add tests for parent/children queries, empty results, and error cases

## 6. Mutations (`core` crate)

- [ ] 6.1 Implement `create_block(parent)` — require parent, generate UUID v4, empty properties, create `parent` edge
- [ ] 6.2 Implement `move_block(id, new_parent)` — require parent, update `parent` edge, validate existence, reject cycles and move-to-root
- [ ] 6.3 Implement `delete_block(id)` — reject root block and blocks with children, else remove block and incident edges
- [ ] 6.4 Add tests for create, move, delete including error cases (nonexistent parent, cycle, delete with children)

## 7. CLI (`cli` crate)

- [ ] 7.1 Set up clap CLI with global `--file` argument and subcommands: `query`, `create`, `move`, `delete`
- [ ] 7.2 Wire `query` subcommand to session + query executor; print results (id and properties per line or JSON)
- [ ] 7.3 Wire `create` subcommand with required `--parent`; print new block UUID; save on success
- [ ] 7.4 Wire `move` and `delete` subcommands; save on success; print errors to stderr with non-zero exit
- [ ] 7.5 Add CLI smoke test or manual test script documented in crate README (optional one-liner in root README)

## 8. Verification

- [ ] 8.1 Run `cargo test` — all unit and integration tests pass
- [ ] 8.2 Manual end-to-end walkthrough: open new kb (root auto-created) → create child under root → query children → query parent → move → delete leaf
