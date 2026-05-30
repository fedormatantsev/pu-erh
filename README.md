# pu-erh

Block-based graph knowledge base with append-only version history.

Coding agents: see [AGENTS.md](AGENTS.md) (Cursor and Claude Code are wired via `.cursor/rules/` and [CLAUDE.md](CLAUDE.md)).

## Quick start

```bash
cargo build
cargo test
```

Initialize a knowledge base and discover the auto-created root id:

```bash
FILE=/tmp/kb.json
ROOT=$(cargo run -q -p pu-erh -- --file "$FILE" init)
cargo run -q -p pu-erh -- --file "$FILE" create --parent "$ROOT"
cargo run -q -p pu-erh -- --file "$FILE" query "children:$ROOT"
```

## Architecture

In memory, a knowledge base is a single **trie-backed store** (`KnowledgeBase`): two radix tries holding all block and edge version records. Mutations insert new records directly into the tries. Active blocks and edges are resolved at read time via CRDT winner selection (highest version, digest tie-break), excluding tombstones.

There is no separate in-memory append log or rematerialization step.

## Storage format (v1)

On disk, version records are stored as JSON arrays — a persistence envelope, not the in-memory structure:

```json
{
  "format_version": 1,
  "block_versions": [...],
  "edge_versions": [...]
}
```

Load inserts each record into the trie; save exports trie contents (sorted by full CRDT key). Each mutation appends new `BlockVersion` or `EdgeVersion` records with BLAKE3 content digests.

This replaces the earlier walking-skeleton snapshot format (`{ "blocks", "edges" }`). There is no migration path — create a new knowledge base file.

Replication merges knowledge bases by trie union (identical records share the same full key); conflict resolution happens at read-time winner selection.
