## Why

The Properties View currently shows a block's existing user properties in a read-only generic list with no way to add or remove them. Users need to define their own named properties on blocks so those properties can later be targeted by queries and automations.

## What Changes

- The generic properties list in the Properties View gains add and remove controls for user-defined properties
- A new property key/value entry form (key input + value input + confirm) is accessible inline from the list
- Each existing user property row gains a remove action
- Reserved and well-known property keys (`title`, `display`, `body`, `order`) remain excluded from user management controls
- The system defines what counts as a "user property" vs. a reserved/well-known property, so add/remove controls can filter the allowed key space

## Capabilities

### New Capabilities

- `user-property-management`: Spec governing the rules for add/remove user property operations in the Properties View — which keys are allowed, validation constraints, and mutation path.

### Modified Capabilities

- `properties-view`: Add requirements for the add-property form and remove action on user property rows.

## Impact

- `crates/core` / `graph` — no new mutations needed; `set_property` (add) and `remove_property` (or `set_property` with removal semantics) cover both operations
- `properties-view` UI component — new inline form and delete affordance on list rows
- `property-registry` — no spec change; its reserved key list is the authoritative exclusion set
