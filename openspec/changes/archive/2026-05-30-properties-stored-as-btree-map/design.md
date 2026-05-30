## Context

The graph crate defines `Properties` as `HashMap<String, serde_json::Value>` in `model.rs`. Block and edge version records, materialized snapshots, and digest computation all use this type. Digest hashing (`digest.rs`) already requires lexicographic key order when feeding properties to BLAKE3, implemented today by collecting `properties.iter()` into a temporary `BTreeMap` on every hash invocation.

The immutable-graph-crdt design documents digest field order as: fixed identity fields → `tombstoned` → `properties` (keys sorted, then values). Storage type and hash input order should match to avoid redundant work and keep the in-memory representation faithful to the digest contract.

## Goals / Non-Goals

**Goals:**

- Store `Properties` as `BTreeMap<String, serde_json::Value>` so native iteration order is lexicographic by key.
- Remove the temporary sort/allocation in `hash_properties`.
- Preserve existing digest values (no breaking change to version chains or conflict resolution).
- Keep the public `Properties` alias and existing call sites (`Properties::new()`, insert, clone, serde) working without API churn.

**Non-Goals:**

- Changing property value semantics, key types, or JSON encoding of values.
- Migrating on-disk storage formats or existing persisted histories.
- Replacing other `HashMap` uses in the codebase (e.g. snapshot trie indexes) — only the `Properties` bag type changes.
- Exposing ordered-map semantics as a new public API beyond what serde and digest already imply.

## Decisions

### 1. Use `BTreeMap` as the `Properties` backing type

**Choice:** `pub type Properties = BTreeMap<String, serde_json::Value>;`

**Rationale:** `BTreeMap` guarantees sorted iteration by key, matching digest requirements. Lookups and inserts remain O(log n), which is acceptable for small property bags on blocks/edges. Serde serializes `BTreeMap` in key order, giving stable JSON output.

**Alternatives considered:**

- Keep `HashMap` and sort at hash time — current approach; works but allocates and sorts on every digest.
- Custom newtype wrapper around `BTreeMap` — adds boilerplate without benefit; the type alias is sufficient.
- `IndexMap` — preserves insertion order, not lexicographic order; wrong semantics for digest.

### 2. Simplify `hash_properties` to direct iteration

**Choice:** Iterate `for (key, value) in properties` directly; remove the `BTreeMap` collect step.

**Rationale:** With `Properties` already ordered, iteration order equals digest order. Same BLAKE3 input bytes as before when keys were sorted at hash time.

### 3. Rely on standard `BTreeMap` serde behavior

**Choice:** No custom `Serialize`/`Deserialize` impl for `Properties`; keep derived serde on structs containing `properties: Properties`.

**Rationale:** `BTreeMap`'s serde impl emits keys in sorted order. Deserialization accepts any JSON object key order and stores entries in sorted order internally. Existing persisted JSON remains compatible.

**Alternatives considered:**

- Custom serde to preserve HashMap-like arbitrary key order in JSON — contradicts stability goal.

### 4. Verify digest equivalence with tests

**Choice:** Add a test that builds properties with out-of-order inserts and asserts digest matches a manually ordered reference; keep existing stability test.

**Rationale:** Guards against regressions if someone reintroduces unsorted iteration.

## Risks / Trade-offs

- **[Slightly slower inserts/lookups vs HashMap]** → Property bags are tiny (often empty); O(log n) is negligible compared to hashing and I/O.
- **[Accidental reliance on HashMap random iteration order in tests]** → Run full test suite; fix any test assuming unstable ordering.
- **[Developers might expect HashMap O(1) lookups]** → Document in code comment on `Properties` alias that it is intentionally ordered for digest stability.

## Migration Plan

No migration required. Change is internal to the graph crate type alias:

1. Swap `HashMap` → `BTreeMap` in `model.rs`.
2. Simplify `digest.rs`.
3. Run `cargo test` across workspace.
4. No data backfill; existing digests unchanged.

Rollback: revert the two-file change.

## Open Questions

_(none — scope is a straightforward type swap with test verification)_
