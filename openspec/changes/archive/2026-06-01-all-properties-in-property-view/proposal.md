## Why

The Properties View hides any property whose key has "reserved semantics" — currently the `body` property — even when that property has no dedicated layout slot. Opening a paragraph block (a child of a `display=document` block, which carries its rich-text payload in `body`) shows an empty generic list with no add form context for that property, and a user inspecting their data cannot see or remove the `body` payload from this surface. The Properties View is the only general-purpose introspection surface, so any property that lives on a block must be discoverable there.

## What Changes

- Redefine the generic properties list inclusion rule: exclude **only** keys with dedicated layout slots (`title`, `display`). All other properties on the block — including `body` and any future reserved-but-not-slotted key — appear in the generic list.
- Drop the property-registry assertion that `body` MUST be excluded from generic property lists. `body` keeps its reserved domain semantics (string payload owned by the Document View), but visibility in introspection surfaces is no longer the registry's concern.
- Relax the add-property form's reserved-key check to block only dedicated-slot keys (`title`, `display`). The user may add a `body` property to any block via the form; behaviour on non-document blocks remains "just a string property" and the Document View is unchanged.
- Update `apps/desktop/src/views/PropertiesView.tsx` to remove `body` from `RESERVED_KEYS`, which by construction makes it appear in `userProps` and unblocks it in the add form.

No new schema. No new IPC. No persistence change. The rendering of a generic row continues to be `key: value` with a Remove action; rows are read-only for now.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `properties-view`: change the generic-list inclusion rule and the add-form reserved-key check from "dedicated slot OR reserved semantics" to "dedicated slot only".
- `property-registry`: remove the requirement that `body` be excluded from generic user-editable property lists; keep the reserved-key + string-payload semantics.

## Impact

- Affected code: `apps/desktop/src/views/PropertiesView.tsx` (`RESERVED_KEYS`, `isReserved` callsites for generic-list filtering and add-form validation).
- Affected specs: `openspec/specs/properties-view/spec.md`, `openspec/specs/property-registry/spec.md`.
- No change to `core`, `graph`, `storage`, or IPC surfaces.
- No change to the Document View — it continues to own write-side handling of `body`.
- No migration: existing knowledge-base files already store `body` as a string; the change is purely UI-surface visibility.
