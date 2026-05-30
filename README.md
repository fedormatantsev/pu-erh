# pu-erh

Block-based graph knowledge base with append-only version history.

## Quick start

```bash
cargo build
cargo test
```

Initialize a knowledge base and discover the auto-created root id:

```bash
FILE=/tmp/kb.json
ROOT=$(cargo run -q -p pu-erh-core --example show-root -- "$FILE")
cargo run -q -p pu-erh -- --file "$FILE" create --parent "$ROOT"
cargo run -q -p pu-erh -- --file "$FILE" query "children:$ROOT"
```

## Storage format (v1)

Knowledge bases are stored as JSON with append-only version records:

```json
{
  "format_version": 1,
  "block_versions": [...],
  "edge_versions": [...]
}
```

Each mutation appends new `BlockVersion` or `EdgeVersion` records with BLAKE3 content digests. Reads materialize a snapshot by selecting the winning version per entity (highest version, digest tie-break) and excluding tombstones.

This replaces the earlier walking-skeleton snapshot format (`{ "blocks", "edges" }`). There is no migration path — create a new knowledge base file.

Replication merges histories by union with digest deduplication; conflict resolution happens at read-time materialization.
