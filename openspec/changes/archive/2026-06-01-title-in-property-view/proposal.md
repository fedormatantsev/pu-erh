## Why

`title` is already a reserved well-known property used by the default inline renderer to label blocks, but there is currently no way for a user to set or edit it — the Properties View only exposes `display`. Adding a dedicated `title` slot to the Properties View closes this gap and makes `title` a first-class editable property.

## What Changes

- **New dedicated slot in the Properties View** for the `title` well-known property: a text input that reads the current block's `title` string value and writes changes through the `set_property` mutation (same pattern as `display`).
- `title` MUST NOT appear in the generic properties list; its slot is its only representation.
- The `title` slot is rendered before the `display` slot (title is the most user-visible property).
- Writes to `title` go through `setProperty` IPC immediately on change; they are persisted on the next explicit Save.

## Capabilities

### New Capabilities

*(none)*

### Modified Capabilities

- `well-known-properties`: `title` is added as a second well-known property with a dedicated layout slot (text input, rendered before `display`).

## Impact

- `apps/desktop/src/views/PropertiesView.tsx` — add `title` state, load from block, render a text input in a dedicated slot above `display`, call `setProperty` on change.
- `openspec/specs/well-known-properties/spec.md` — new requirement for the `title` slot.
