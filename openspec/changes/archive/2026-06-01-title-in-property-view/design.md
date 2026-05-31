## Context

The Properties View today has one well-known slot (`display`) and a Save button. `title` is already reserved by the `block-view` spec as the label shown by the default inline renderer, but there is no way to set it. Adding an edit slot for `title` in `PropertiesView.tsx` follows the same pattern already established for `display`.

Current `PropertiesView` state:
- Loads `display` from the block on mount.
- Writes `display` changes immediately via `setProperty` IPC.
- Persists on explicit Save.

## Goals / Non-Goals

**Goals:**
- A `title` text input slot in the Properties View, rendered before `display`.
- Reads the block's current `title` string on mount; empty string when absent or non-string.
- Writes changes immediately through `setProperty` IPC (same as `display`).
- `title` excluded from any future generic properties list.

**Non-Goals:**
- Inline editing of `title` outside the Properties View.
- Validation (length, uniqueness, non-empty).
- Persisting `title` implicitly — only on explicit Save.

## Decisions

### Decision: Mirror the `display` pattern exactly

Load the current value in a `useEffect` on `blockId` change; write to `setProperty` in the `onChange` handler; persist on Save. No separate save-per-field button.

**Alternative considered**: debounced auto-save on title change. Rejected — inconsistent with the desktop shell's explicit-save-only policy.

### Decision: Render a plain `<input type="text">` with a "Title" label

Unlike `display` (which needs no label because the control is self-explanatory as a view-name selector), a bare text input needs a label for clarity. A `<label>` wrapping `<input type="text">` is the accessible minimum.

**Alternative considered**: no label (matching `display`). Rejected — a text box without a label is ambiguous.

### Decision: Empty string when `title` is absent

When the block has no `title` property, or its value is not a string, the input shows an empty string. Writing an empty string back through `setProperty` is a valid no-op semantically (the inline renderer will fall back to the block id); it does not need special-casing.

## Risks / Trade-offs

- **Writing empty title**: if the user clears the title field and saves, `setProperty("title", "")` will persist an empty string rather than removing the key. The inline renderer treats this as a non-string value and falls back to the block id, so the observable behavior is correct. A future cleanup can add a "remove property" operation if needed.
