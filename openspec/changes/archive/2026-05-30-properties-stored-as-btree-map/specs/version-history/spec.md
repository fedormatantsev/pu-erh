## MODIFIED Requirements

### Requirement: Digest computation

Each version record MUST include a `digest` computed as a BLAKE3 hash over the record's in-memory fields using in-place incremental hashing. Field values MUST be fed to the hasher in a fixed, documented order. Serialization MUST NOT be used as an intermediate step when computing digests. Property keys MUST be hashed in lexicographic order, followed by each key's JSON-encoded value.

#### Scenario: Same in-memory content produces same digest

- **WHEN** two records have identical in-memory field values hashed in the defined order
- **THEN** their `digest` values are equal

#### Scenario: Digest is stable across hash invocations

- **WHEN** the same record is hashed multiple times
- **THEN** the resulting `digest` is identical each time

#### Scenario: Property insertion order does not affect digest

- **WHEN** two property maps contain the same keys and values but entries were inserted in different orders
- **THEN** hashing either map produces the same digest contribution
