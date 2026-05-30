## MODIFIED Requirements

### Requirement: Read-time materialization

The system MUST derive a read-only graph snapshot from version history at query time, not persist the snapshot as authoritative state. Materialization MUST populate radix-trie-backed version sets only; it MUST NOT require a separate eager pass that pre-computes cached active block and edge maps before reads.

#### Scenario: Snapshot built on read

- **WHEN** a query or validation requires graph state
- **THEN** the system provides a snapshot whose authoritative state is the merged block and edge version tries

#### Scenario: No eager active map rebuild

- **WHEN** version history is materialized into a snapshot after a mutation
- **THEN** the system updates version tries without building separate active block or edge hash maps upfront

### Requirement: Invariant filtering

After selecting winners, read operations MUST apply only cheap, local invariant checks rather than global graph analysis. Successful mutation operations are assumed not to introduce invariant violations; read paths MUST NOT perform cycle detection or graph-wide exclusion passes.

#### Scenario: Invalid edge ignored on point read

- **WHEN** a winning edge is returned by a point edge lookup
- **AND** either endpoint block is absent or tombstoned
- **THEN** that edge is treated as absent for that lookup

#### Scenario: No global cycle filtering at read time

- **WHEN** active parent edges contain a cycle
- **THEN** read operations do not run a global cycle detection pass to omit cycle participants

## ADDED Requirements

### Requirement: Pure per-call active reads

Each active read MUST recompute its result from version tries at call time without using cached active maps or cross-call invalidation state from prior reads.

#### Scenario: Point block lookup uses trie winner

- **WHEN** `block(id)` is called on a snapshot
- **THEN** the system resolves the winner under the block entity prefix in the block version trie on that call and returns active block fields or absence

#### Scenario: Point edge lookup uses trie winner

- **WHEN** `get_edge(source, target, edge_type)` is called
- **THEN** the system resolves the winner under the edge entity prefix in the edge version trie on that call and returns active edge fields or absence

#### Scenario: Children query uses trie prefix scan

- **WHEN** `children_of(parent)` is called
- **THEN** the system scans the edge version trie by parent navigation prefix on that call and resolves per-child winners without a pre-built edge map

### Requirement: Mutation-presumed invariant validity

The system MUST validate graph invariants before appending mutation version records. Read operations MAY assume the active view for session-scoped mutation sequences does not require global invariant repair.

#### Scenario: Successful mutation preserves readable graph

- **WHEN** a mutation succeeds through the session mutation path
- **THEN** subsequent point reads for entities affected by that mutation reflect the intended graph structure without global read-time filtering
