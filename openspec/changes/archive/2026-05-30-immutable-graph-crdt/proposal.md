## Why

The walking-skeleton graph is a mutable in-memory structure mutated in place. That makes history, replication, and concurrent reads hard to reason about. pu-erh needs an **append-only version history** for blocks and edges — each mutation inserts a new version with `digest` / `previous_digest` links — and an immutable snapshot **materialized at read time** with deterministic conflict resolution.

## What Changes

- Introduce versioned block and edge records (`version`, `digest`, `previous_digest`) — no mutation envelopes
- Replace in-place graph mutation with **append new version** semantics
- Materialize read-only snapshot at query time: **max version, max digest wins** per block/edge id
- Allow history **branching** from concurrent edits; replication unions version records without resolving conflicts
- Filter out blocks/edges that violate graph invariants at read time (ignore, do not fail)
- **BREAKING:** Change on-disk storage from snapshot JSON to version-history JSON
- Refactor `graph`, `storage`, and `core`; preserve existing CLI commands and query/mutation behavior for the linear (single-writer) case

**Non-goals for this change:**
- Network transport or replication protocol (only local append + union merge of histories)
- Conflict UI or manual resolution workflows
- DataFusion integration
- Property-level merge semantics beyond the winning version's property bag
- Automatic migration from snapshot JSON format
- History compaction / garbage collection

## Capabilities

### New Capabilities

- `version-history`: Append-only block/edge version records, digest chain, branching, replication union
- `immutable-snapshot`: Read-time materialization, conflict resolution, invariant filtering

### Modified Capabilities

- `block-model`: Blocks and edges are versioned entities; active view is derived at read time
- `storage`: Persist and load version history instead of authoritative snapshot JSON
- `session`: Track version history, rematerialize snapshot on read and after each mutation
- `mutations`: Append new block/edge versions instead of mutating graph in place

### Unchanged specs

- `query-language`: reads materialized snapshot (no requirement changes)
- `cli`: same commands (no requirement changes)

## Impact

- **BREAKING** `graph` crate: version records, snapshot materialization, remove mutable in-place API
- **BREAKING** `storage` crate: new file format; no migration from snapshot JSON
- `pu-erh-core`: session and mutations refactored around version append + read-time snapshot
- `cli`: minimal wiring changes
- All existing tests updated; new tests for branching, conflict resolution, and invariant filtering
