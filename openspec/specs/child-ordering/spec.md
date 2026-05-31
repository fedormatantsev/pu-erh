# child-ordering Specification

## Purpose

Defines the `order` well-known property on parent edges, fractional-index key generation rules, `PositionHint` semantics for sibling placement in create and move mutations, and the `children_ordered` query API including tie-breaking behavior.
## Requirements
### Requirement: order property on parent edge

Every parent edge MUST carry a well-known `order` property whose value is a `PropertyValue::String` containing a valid fractional-index string. The `order` property MUST be set when the edge is first written (on block creation or reparenting). The `order` value is stored in the edge's `properties` map under the key `"order"` and participates in BLAKE3 digest computation as a regular edge property.

#### Scenario: New block has order on its parent edge

- **WHEN** a block is created with a parent and a position hint
- **THEN** the new parent edge version record has `"order"` in its properties map
- **AND** the value is a non-empty fractional-index string

#### Scenario: Move sets order on the new parent edge

- **WHEN** a block is moved to a new parent with a position hint
- **THEN** the new parent edge version record has `"order"` in its properties map
- **AND** the value reflects the requested sibling position

### Requirement: Fractional-index key generation

The system MUST implement `generate_key_between(left: Option<&str>, right: Option<&str>) -> Result<String, OrderError>` based on the jitl fractional-indexing algorithm. The function MUST satisfy: if `left` is Some and `right` is Some then `left < result < right` under lexicographic string comparison. The function MUST return `Err(OrderError)` when inputs are invalid (e.g. `left >= right`, or either value is not a valid fractional-index string).

#### Scenario: Key between two existing keys

- **WHEN** `generate_key_between(Some("a0"), Some("b0"))` is called
- **THEN** the result is a string `k` where `"a0" < k < "b0"` under lexicographic comparison

#### Scenario: Key before the first child

- **WHEN** `generate_key_between(None, Some("a0"))` is called
- **THEN** the result is a string `k` where `k < "a0"` under lexicographic comparison

#### Scenario: Key after the last child

- **WHEN** `generate_key_between(Some("a0"), None)` is called
- **THEN** the result is a string `k` where `k > "a0"` under lexicographic comparison

#### Scenario: Key for the first and only child

- **WHEN** `generate_key_between(None, None)` is called
- **THEN** the result is the canonical starting key for the algorithm (e.g. `"a0"`)

#### Scenario: Invalid input returns error

- **WHEN** `generate_key_between(Some("b0"), Some("a0"))` is called with `left >= right`
- **THEN** the function returns `Err(OrderError)`

### Requirement: PositionHint for sibling placement

Create and move mutations MUST accept a `PositionHint` argument specifying where to insert the block among its siblings. Valid variants are `First` (before all siblings), `Last` (after all siblings, the default), `Before(BlockId)` (immediately before the referenced sibling), and `After(BlockId)` (immediately after the referenced sibling). The graph layer MUST resolve the hint to `(left_order, right_order)` by querying the target parent's ordered children, then call `generate_key_between` to produce the `order` value for the new or moved edge.

#### Scenario: Insert last (default)

- **WHEN** a block is created with `PositionHint::Last` and the parent has existing children with order values `"a0"` and `"b0"`
- **THEN** the new block's parent edge has an `order` value greater than `"b0"`

#### Scenario: Insert first

- **WHEN** a block is created with `PositionHint::First` and the parent has an existing child with order `"a0"`
- **THEN** the new block's parent edge has an `order` value less than `"a0"`

#### Scenario: Insert before a sibling

- **WHEN** a block is created with `PositionHint::Before(sibling_id)` where `sibling_id` has order `"b0"` and its predecessor has order `"a0"`
- **THEN** the new block's parent edge has an `order` value `k` where `"a0" < k < "b0"`

#### Scenario: Insert after a sibling

- **WHEN** a block is created with `PositionHint::After(sibling_id)` where `sibling_id` has order `"a0"` and its successor has order `"b0"`
- **THEN** the new block's parent edge has an `order` value `k` where `"a0" < k < "b0"`

#### Scenario: Before/after references nonexistent sibling

- **WHEN** a create or move specifies `PositionHint::Before(id)` or `PositionHint::After(id)` with an `id` that is not a child of the target parent
- **THEN** the mutation returns an error and no version records are appended

### Requirement: children_ordered query

The graph layer MUST expose `children_ordered(parent_id: BlockId) -> Vec<BlockId>` returning the direct children of `parent_id` sorted ascending by their parent-edge `order` property value under lexicographic comparison. Blocks whose parent edge has no `order` property MUST be treated as having `order = ""` (empty string) for sorting purposes. Ties (two children sharing the same `order` value) MUST be broken by ascending child `BlockId` bytes.

#### Scenario: Children returned in order

- **WHEN** a block has three children with parent-edge order values `"b0"`, `"a0"`, `"c0"`
- **THEN** `children_ordered` returns them in the sequence `"a0"`, `"b0"`, `"c0"`

#### Scenario: Legacy blocks (no order) sort first

- **WHEN** a block has a child whose parent edge has no `"order"` property alongside a child with `order = "a0"`
- **THEN** the legacy child appears before the `"a0"` child in `children_ordered`

#### Scenario: Tie broken by UUID

- **WHEN** two children share the same `order` value
- **THEN** they are sorted ascending by their block id bytes
