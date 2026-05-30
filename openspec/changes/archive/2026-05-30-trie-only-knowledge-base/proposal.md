## Why

The system currently keeps every version record twice in memory: an append-only `VersionHistory` (`Vec`) and a trie-backed `Snapshot` rebuilt on every mutation. Winner resolution is duplicated (linear scan on the vec for append, trie descent for reads), and `rematerialize()` re-walks the full history after each change. Now that radix tries are the read index and CRDT keys encode winner ordering, the vec is redundant — one trie-backed store should be authoritative in memory.

## What Changes

- Replace the dual `VersionHistory` + `Snapshot` model with a single trie-backed **`KnowledgeBase`** as the in-memory authoritative store.
- Move append, merge, and winner resolution (`next version`, `previous_digest`, CRDT winner, active reads) onto the trie; remove `VersionHistory` and `Snapshot::materialize` / session `rematerialize`.
- **BREAKING (internal API):** Remove `VersionHistory`, `Session::history()`, and `append_*` functions that take `&mut VersionHistory`; mutations operate on `&mut KnowledgeBase`.
- Keep **storage format v1** unchanged: JSON file still uses `block_versions` / `edge_versions` arrays; `load` inserts records into the trie, `save` exports trie contents to arrays (order unspecified or key-sorted for stability).
- Rename or alias `Snapshot` → `KnowledgeBase` in public graph API (reads, diff, prefix queries unchanged semantically).
- Add parity tests: trie CRDT winner matches explicit max(version, digest) comparison for all entities.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `immutable-snapshot`: The materialized store is the authoritative in-memory representation; no separate version-history vec or read-time rebuild from vec.
- `version-history`: Append and merge operate on the trie store; version/previous_digest derived from trie CRDT winner, not vec scan.
- `session`: Session owns one `KnowledgeBase`; mutations append directly to tries; no rematerialize step.
- `storage`: Load builds trie from file arrays; save exports trie to file arrays; round-trip preserves record set (not vec identity).

## Impact

- **`crates/graph`**: New or renamed `KnowledgeBase` type; delete `VersionHistory`, `materialize`/`materialize_from`; relocate append/merge; update `lib.rs` exports.
- **`crates/core`**: `Session` holds `KnowledgeBase` only; mutation/query/session tests updated.
- **`crates/storage`**: `load`/`save` take `KnowledgeBase`; no `VersionHistory` in public storage API.
- **`crates/cli`**: Unchanged behavior if session API surface stays equivalent.
- **Specs/README**: Clarify architecture (trie = store, JSON arrays = persistence envelope).
- **No user-facing format migration**: v1 JSON shape preserved; existing knowledge base files remain loadable.
