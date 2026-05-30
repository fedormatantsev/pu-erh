## Why

pu-erh is a greenfield project with a clear architecture but no running code. We need a minimal end-to-end slice that proves the core layers work together — blocks in memory, persistence on disk, navigation queries, and mutations via CLI — before building analytical query features (DataFusion) or a GUI.

## What Changes

- Introduce a Rust monorepo with four crates: `cli`, `core`, `graph`, `storage`
- Define a block model (UUID id, properties map) and edge model (source, target, edge type, properties map)
- Implement an in-memory graph with separate block and edge maps
- Add on-disk storage: load and save a knowledge base to a single file
- Add read-only query language: `parent:<uuid>` and `children:<uuid>`
- Add mutation commands: create block (under a parent), move block (reparent), delete block
- Auto-create a single root block when initializing a new knowledge base
- Wire a CLI that opens a session, runs queries and mutations, and persists on exit

**Non-goals for this change:**
- DataFusion integration and analytical queries
- GUI / App layer
- Non-hierarchical links and refs
- Property filters, search, saved views
- Multi-user, sync, migrations

## Capabilities

### New Capabilities

- `block-model`: Block identity (UUID v4) and property bag; edge model with edge type and property bag
- `session`: User session that owns the in-memory graph and coordinates load/save
- `storage`: Serialize and deserialize a knowledge base to/from a single on-disk file
- `query-language`: Parse and execute read-only navigation queries (`parent:`, `children:`)
- `mutations`: Apply create, move, and delete operations to the graph
- `cli`: Command-line interface for opening a knowledge base, querying, and mutating

### Modified Capabilities

(none — greenfield project)

## Impact

- New Rust crates: `cli`, `core`, `graph`, `storage`
- New dependencies: UUID generation/parsing, serialization (e.g. serde + JSON or similar), CLI parsing (e.g. clap)
- No existing APIs or users affected
- Establishes project layout and patterns for future changes (DataFusion bridge, query language v2, links)
