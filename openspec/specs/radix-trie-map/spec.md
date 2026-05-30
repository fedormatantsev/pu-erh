# radix-trie-map Specification

## Purpose
TBD - created by archiving change radix-trie-map. Update Purpose after archive.
## Requirements
### Requirement: Inner and leaf node structure

The radix trie map MUST represent non-empty maps as a tree of inner nodes and leaf nodes. Each inner node MUST hold a 256-bit children occupancy bitmask and a sparse vector of child nodes corresponding to set bits in ascending byte-index order. Each leaf node MUST store the full key byte sequence and an associated value.

#### Scenario: Empty map has no nodes

- **WHEN** a radix trie map is constructed with no insertions
- **THEN** it contains no root node and reports zero entries

#### Scenario: Inner node stores sparse children

- **WHEN** an inner node has children at byte indices 0 and 255
- **THEN** its children vector contains exactly two child nodes in ascending index order
- **AND** its bitmask reflects occupancy at indices 0 and 255 only

### Requirement: Compact single-entry representation

The radix trie map MUST NOT allocate inner nodes that exist only to represent unshared prefix bytes. Inserting the first key into an empty map MUST produce a structure with a single leaf node (no intermediate inner chain).

#### Scenario: First insertion is a lone leaf

- **WHEN** one key-value pair is inserted into an empty radix trie map
- **THEN** the root is a leaf node holding that key and value
- **AND** no inner nodes are allocated

#### Scenario: Inner nodes appear at divergence

- **WHEN** a second key is inserted that shares a prefix with an existing key
- **THEN** inner nodes are created only at byte positions where keys diverge or branch

### Requirement: Exact key lookup

The radix trie map MUST support lookup by full key bytes with time proportional to key length, independent of total map size.

#### Scenario: Existing key found

- **WHEN** a key that was previously inserted is looked up
- **THEN** the associated value is returned

#### Scenario: Missing key absent

- **WHEN** a key that was never inserted is looked up
- **THEN** no value is returned

### Requirement: Prefix iteration

The radix trie map MUST support iterating all entries whose keys begin with a given prefix byte sequence without scanning unrelated keys.

#### Scenario: Prefix selects matching entries

- **WHEN** entries with keys `ab`, `ac`, and `xy` exist
- **AND** iteration is requested with prefix `a`
- **THEN** exactly the entries for keys `ab` and `ac` are yielded

#### Scenario: Empty prefix iterates all entries

- **WHEN** iteration is requested with an empty prefix
- **THEN** all entries in the map are yielded

### Requirement: Persistent structural sharing

Updates to the radix trie map MUST use copy-on-write semantics so unchanged subtrees are shared between prior and updated versions. Cloning a map handle MUST be shallow and MUST NOT deep-copy the entire tree.

#### Scenario: Unchanged subtree shared after insert

- **WHEN** a new key is inserted into a map with existing entries
- **THEN** subtrees not on the insertion path are shared by reference with the prior map version

#### Scenario: Clone shares root structure

- **WHEN** a map is cloned without mutation
- **THEN** the clone's root node is the same shared node as the original's root node

### Requirement: Structural diff via pointer equality

The radix trie map MUST provide a lazy diff iterator between two map versions that treats subtrees as equal when their root nodes are pointer-identical, and otherwise yields entries for keys added, removed, or whose values changed without materializing a full diff collection upfront.

#### Scenario: Identical maps yield no diff entries

- **WHEN** diff iteration is performed between two maps that share the same root node
- **THEN** the iterator produces no entries

#### Scenario: Divergent subtree keys yielded lazily

- **WHEN** diff iteration is performed between maps that differ only under one byte branch
- **THEN** only keys in that divergent subtree are yielded as added, removed, or changed

#### Scenario: Diff iteration supports early exit

- **WHEN** a caller stops diff iteration after the first yielded entry
- **THEN** the implementation does not require enumerating all remaining differences

### Requirement: Insert replace and remove

The radix trie map MUST support inserting a key (replacing any existing value for the same key) and removing a key, returning a new map version without mutating the prior version.

#### Scenario: Insert replaces existing value

- **WHEN** a key is inserted twice with different values
- **THEN** lookup returns the second value
- **AND** exactly one entry exists for that key

#### Scenario: Remove deletes entry

- **WHEN** an existing key is removed
- **THEN** subsequent lookup for that key returns no value

### Requirement: CRDT fields in trie keys

Radix trie keys for graph version records MUST append CRDT metadata after the entity identity prefix: `version` as big-endian 8-byte integer, `digest` as 32 raw bytes, and `previous_digest` as 32 raw bytes (all zeros when absent).

#### Scenario: Block key includes CRDT suffix

- **WHEN** a block version record is inserted into a radix trie map
- **THEN** its key is 88 bytes consisting of the 16-byte block id followed by the 72-byte CRDT suffix

#### Scenario: Edge key includes CRDT suffix

- **WHEN** an edge version record is inserted into a radix trie map
- **THEN** its key is 105 bytes consisting of the 33-byte edge identity (target, type byte, source) followed by the 72-byte CRDT suffix

#### Scenario: Version ordering matches numeric order

- **WHEN** two keys share the same entity identity prefix and differ only in the version field
- **THEN** the key with the larger version number compares lexicographically greater

#### Scenario: Digest tie-breaks equal version

- **WHEN** two keys share the same entity identity prefix and version but differ in digest
- **THEN** the key with the lexicographically greater digest compares lexicographically greater

### Requirement: Winner lookup by prefix descent and last-child descent

When resolving the winning version record for an entity, the system MUST select the leaf with the lexicographically greatest full key under that entity's identity prefix. The implementation MUST locate this leaf in at most trie-depth node lookups by (1) descending one child per prefix byte, then (2) at each subsequent inner node following the last child (highest occupied byte index) until a leaf is reached.

#### Scenario: Higher version wins via key order

- **WHEN** two block version records share the same id prefix with versions 2 and 3
- **THEN** `winner_under_prefix` on the id prefix returns the version 3 record

#### Scenario: Digest tie-break via key order

- **WHEN** two block version records share the same id prefix and version with different digests
- **THEN** `winner_under_prefix` on the id prefix returns the record with the lexicographically greater digest

#### Scenario: Winner lookup is bounded by trie depth

- **WHEN** `winner_under_prefix` is called with an entity identity prefix
- **THEN** the number of node lookups is at most the trie depth (≤ full key length)
- **AND** the implementation does not scan all entries sharing the prefix

