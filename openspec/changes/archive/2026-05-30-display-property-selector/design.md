## Context

`PropertiesView` currently renders `display` as a free-text `<input>`. `blockView.tsx` already has a `blockRenderers` registry (currently empty — only the default view exists), and `App.tsx` already uses `resolveBlockView` which falls back to the default when the value is unrecognized. The gap is: the Properties View doesn't constrain entry to the known set, and doesn't recover gracefully from absent/unknown values.

Current flow:
1. Component loads `block.properties.display` (may be absent, a known string, or an unknown string).
2. User types a value in a free-text input.
3. "Apply display" → `setProperty(id, "display", typedValue)`.
4. "Save" → `save()`.

## Goals / Non-Goals

**Goals:**
- `display` in Properties View is a dropdown populated from registered view names + "default", rendered as a dedicated first-class layout element — not an item inside the generic properties list.
- Absent or unrecognized `display` values silently resolve to "default" in the UI; the resolved default is written to storage on save.
- The well-known predefined properties concept is defined as: properties with dedicated, fixed-position UI components in the `PropertiesPanel` layout, separate from the generic property list. `display` is the first such property.
- No property title label for `display` — the dropdown is the entire `display` element.

**Non-Goals:**
- Validating `display` at the backend / core layer.
- A runtime registry for routing property keys to controls within a list (not needed — `display` has a fixed slot, not a list position).
- Auto-save or eager-write on dropdown change.
- Any changes outside `apps/desktop/src/`.

## Decisions

### `display` as a dedicated layout slot, not a list item

**Decision:** `PropertiesPanel` layout explicitly positions a `DisplaySelector` component (or equivalent inline `<select>`) in a fixed slot — before the generic properties list. `display` is excluded from the generic properties list. The `PropertiesView` renders:
1. The `display` dropdown slot (always present, always first).
2. The generic properties list (all properties except `display`).
3. The Save button.

**Rationale:** `display` is a first-class property of the block view itself — it governs what the entire Properties View is the settings panel for. Treating it as just another list item with a special renderer would obscure that structural role. A fixed layout slot makes the primacy explicit and avoids any per-item dispatch logic.

**Alternative:** Well-known property registry consulted per item in the list, with `display` rendered differently in-place. Rejected — conflates layout structure with rendering style; `display` belongs outside the list.

### Options sourced from `blockView.tsx` export

**Decision:** `blockView.tsx` exports `BLOCK_VIEW_NAMES: string[]` — `["default", ...Object.keys(blockRenderers)]`. `PropertiesView` imports this directly to populate the dropdown.

**Rationale:** Single source of truth; no separate registry module needed for one property.

**Alternative:** Hard-code `["default"]` in `PropertiesView`. Works today but silently ignores any future registered view, so rejected.

### Absent/unknown → resolve to "default", overwrite on save

**Decision:** On load, if `block.properties.display` is absent or not in `BLOCK_VIEW_NAMES`, the component sets local state to `"default"` and tracks `needsWrite = true`. The save handler calls `setProperty(id, "display", resolved)` before `save()` when `needsWrite` is true.

**Rationale:** Matches the specified behavior: implicit fallback shown to the user, storage corrected on the next deliberate save. Does not silently clobber storage on load.

**Alternative:** Write the default on load (eagerly). Rejected — produces a surprise mutation every time a block with no `display` is viewed, even when the user never touches the Properties View.

## Risks / Trade-offs

- [BLOCK_VIEW_NAMES is static at module load] If renderers were registered dynamically at runtime (not today), the options list would be stale. Mitigation: acceptable for v0; all renderers are registered at module parse time.
- [Only "default" exists today] Dropdown has one option. This is correct and expected — the UI reflects what's actually available.
- [Generic list excludes display] Code that iterates `block.properties` to render the list must filter out `"display"`. Mitigation: a `LAYOUT_SLOT_PROPERTIES = new Set(["display"])` constant makes the exclusion explicit.
