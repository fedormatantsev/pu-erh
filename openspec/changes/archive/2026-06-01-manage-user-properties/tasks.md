## 1. Core mutation: remove_property

- [x] 1.1 Add `remove_property(kb, block_id, key)` function to `crates/core/src/mutation.rs` — appends a new block version record with the given key absent from properties; reject if block not in active view or key not present
- [x] 1.2 Add unit tests in `mutation.rs` covering: remove existing key, remove on nonexistent block (error), remove nonexistent key on existing block (error)
- [x] 1.3 Expose `session.remove_property(id, key)` in `crates/core/src/session.rs` delegating to `mutation::remove_property`

## 2. Desktop backend: Tauri command

- [x] 2.1 Add `remove_property` Tauri command to `crates/desktop/src/state.rs` following the pattern of `set_property` — takes `id: &str` and `key: String`, calls `session.remove_property`, maps error to String
- [x] 2.2 Register the new command in the Tauri app builder (same file or `lib.rs`) alongside the existing commands

## 3. IPC layer

- [x] 3.1 Add `removeProperty(id: string, key: string): Promise<void>` to `apps/desktop/src/ipc.ts` invoking the `"remove_property"` Tauri command

## 4. Properties View UI

- [x] 4.1 In `PropertiesView.tsx`, load the block's full properties map on mount and extract user properties (all keys not in `RESERVED_KEYS`) into a state variable
- [x] 4.2 Render the user properties list: each row shows the key and value (read-only display at v0) with a remove button; render nothing if the list is empty
- [x] 4.3 Wire the remove button: call `removeProperty(blockId, key)` and update local state to drop the removed key on success; surface errors via the existing `error` state
- [x] 4.4 Add the inline add-property form below the user properties list: two text inputs (key, value) and a confirm button
- [x] 4.5 Validate the add form on submit: reject empty key or key in `RESERVED_KEYS` by setting an inline form error state (do not call `setProperty`); valid submission calls `setProperty(blockId, key, value)` and appends the new entry to local state on success
