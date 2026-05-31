## 1. PropertiesView — title slot

- [x] 1.1 Add `title` state (`string`) to `PropertiesView` in `apps/desktop/src/views/PropertiesView.tsx`, initialized to `""`
- [x] 1.2 In the `blockId` `useEffect`, read `block.properties.title` and set the title state (empty string when absent or non-string)
- [x] 1.3 Add an `onTitleChange` handler that calls `setProperty(blockId, "title", value)` immediately (same pattern as `onDisplayChange`)
- [x] 1.4 Render a `<label>` + `<input type="text">` for the title slot above the `display` dropdown in the JSX
