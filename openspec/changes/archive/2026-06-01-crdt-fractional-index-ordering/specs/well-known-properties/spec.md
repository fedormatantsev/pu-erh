## ADDED Requirements

### Requirement: Well-known edge property registry

The system MUST maintain a registry of well-known edge property keys that carry semantic meaning and are reserved for system use. The registry MUST include `"order"` as a well-known key for `EdgeType::Parent` edges. Well-known edge properties MUST NOT be surfaced in generic, user-editable property lists for edges. The normative specification for each well-known edge property key's behavior lives in the capability spec named alongside it.

| Key | Edge type | Capability spec |
|-----|-----------|-----------------|
| `"order"` | `Parent` | `child-ordering` |

#### Scenario: order is a reserved property key for parent edges

- **WHEN** the system writes a parent edge version record
- **THEN** the `"order"` key in the edge properties carries the fractional-index position value as defined by the child-ordering spec
- **AND** the key is not exposed as a generic user-editable property
