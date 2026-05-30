## 1. Block View — export available view names

- [x] 1.1 In `blockView.tsx`, export `BLOCK_VIEW_NAMES: string[]` computed as `["default", ...Object.keys(blockRenderers)]`

## 2. Properties View — display dedicated layout slot

- [x] 2.1 In `PropertiesView.tsx`, on load resolve `display`: if `block.properties.display` is absent or not in `BLOCK_VIEW_NAMES`, set local state to `"default"` and `needsWrite = true`; otherwise set to the stored value and `needsWrite = false`
- [x] 2.2 Replace the existing `<label>` field for `display` with a standalone `<select>` element (no wrapping label, no title text) bound to the resolved local state, populated from `BLOCK_VIEW_NAMES`
- [x] 2.3 On dropdown change, call `setProperty(blockId, "display", newValue)` and set `needsWrite = false`
- [x] 2.4 In the save handler, if `needsWrite` is true call `setProperty(blockId, "display", resolvedValue)` before `save()`, then set `needsWrite = false`
- [x] 2.5 Add `LAYOUT_SLOT_PROPERTIES = new Set(["display"])` constant and filter it out when rendering the generic properties list (the list currently has no items but the exclusion must be explicit for future properties)
