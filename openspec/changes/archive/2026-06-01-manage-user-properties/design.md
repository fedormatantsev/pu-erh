## Context

The Properties View today renders well-known slots (title, display) and a read-only generic list of any remaining block properties. Users have no way to add new key/value pairs to a block or delete existing ones. User-defined properties are important because they will be the target keys for future query predicates and automation rules.

The block model stores all properties in a single `HashMap<String, PropertyValue>` per block version record. CRDT winner selection picks the latest version record as the active view of a block. No explicit property-removal primitive exists today — only `set_property` (single-key upsert).

## Goals / Non-Goals

**Goals:**
- Users can add arbitrary key/value pairs (user properties) to a block from the Properties View
- Users can remove any user-defined property from a block
- Reserved keys (`title`, `display`, `body`, and well-known edge keys) are excluded from add/remove operations
- Mutations go through `core::Session` (no UI-layer property logic)

**Non-Goals:**
- Editing existing user property values inline (value editing is a future enhancement)
- Renaming user property keys
- Bulk property operations
- Property type selection beyond the default string type at add time

## Decisions

### 1. Property removal via new `remove_property` mutation

**Decision**: Add `remove_property(block_id, key)` to the mutations capability rather than reusing `set_property` with `PropertyValue::Null` as a tombstone.

**Rationale**: `PropertyValue::Null` already has defined semantics in the block model (a valid value type). Using it as a deletion sentinel conflates "this property has a null value" with "this property is deleted." A dedicated `remove_property` mutation appends a block version record with the key absent from the properties map — clean and unambiguous. The extra surface area is minimal (one function, same validation pattern as `set_property`).

**Alternative considered**: Null-sentinel approach — rejected because it leaks deletion semantics into the value type and complicates queries that may want to match `null` as a meaningful value.

### 2. Reserved key set is derived from `property-registry`

**Decision**: The "user property" classification is simply "any key not in the reserved set defined by `property-registry`." No separate user-property registry is needed.

**Rationale**: The reserved set is already authoritative and small (`title`, `display`, `body`). Adding a parallel registry for user properties would duplicate concern. The UI layer checks the reserved set at add-time (to block the key) and at render-time (to exclude reserved keys from user property rows).

### 3. Add form is inline in the generic properties list

**Decision**: The add form appears at the bottom of the generic properties list as an inline row: a key input, a value input (string, the only type at v0), and a confirm button.

**Rationale**: Keeps the interaction in-context with the list it affects. An external modal or separate panel would be disproportionate for a simple key/value entry. The form is shown persistently (not behind a toggle) per the simplest possible UX; spec says nothing about collapsing it, so we leave that unspecified.

**Anti-default note**: The spec does NOT define the confirm button label, placeholder text, or exact visual treatment. Tasks must stub or leave those unspecified unless a future spec adds them.

### 4. Remove action on each user property row

**Decision**: Each user property row renders a remove action (visually a small button or icon). Activating it calls `remove_property` immediately through `core::Session`.

**Rationale**: Consistent with how `set_property` is triggered in the title slot — mutations are immediate, persistence timing follows the desktop-shell save policy.

## Risks / Trade-offs

- **Key collision at add time** → If the user types a reserved key (e.g., `title`), the UI MUST reject the add with a validation message. The spec defines this; the task must implement the check.
- **Duplicate key at add time** → If the user types a key that already exists on the block, `set_property` will overwrite it. This is acceptable at v0; no warning is required by the spec.
- **No value type selection** → All user-added properties are strings at v0. This limits the eventual query expressiveness but avoids premature type-picker UX.
- **`remove_property` on a non-existent key** → The mutation MUST be rejected at the core layer, consistent with the validation pattern in `set_property`.
