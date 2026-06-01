## ADDED Requirements

### Requirement: Active read semantics reference

Logical blocks and edges defined in this capability are materialized at read time from version history. Active entity resolution, tombstone exclusion, conflict resolution, and per-call reads MUST follow **`immutable-snapshot`**. Version record layout and digests MUST follow **`version-history`**.

#### Scenario: Point read uses immutable snapshot

- **WHEN** a consumer reads a block or edge through the knowledge base API
- **THEN** the active entity is derived per **`immutable-snapshot`**, not from a separate cached materialization layer defined in this capability
