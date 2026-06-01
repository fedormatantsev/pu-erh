## 1. Update PropertiesView component

- [x] 1.1 In `apps/desktop/src/views/PropertiesView.tsx`, rename the `RESERVED_KEYS` constant to `SLOT_KEYS` and change its value to `["title", "display"] as const`. Update the type alias and the guard function name (`isReserved` → `isSlotKey`) at the same time.
- [x] 1.2 Verify the generic-list builder loop (`for (const [k, v] of Object.entries(block.properties))`) now collects `body` into `userProps` because the guard no longer excludes it.
- [x] 1.3 Verify the add-form handler (`onAddProperty`) now produces the error only for empty keys and dedicated-slot keys (`title`, `display`). The `body` key MUST pass through to `setProperty`.
- [x] 1.4 Update the user-facing error message for the slot-key case to reflect the new rule (e.g., `"\"${trimmedKey}\" has a dedicated slot."`).

## 2. Manual verification in the desktop shell

- [x] 2.1 Run `bun run dev:desktop`, open a fresh kb, create a `display=document` block, add at least one paragraph child, edit the paragraph in Document View to populate `body`.
- [x] 2.2 Toggle to Properties View on the paragraph block. Confirm a row `body: <serialized string>` appears in the generic list and the Remove button is visible.
- [x] 2.3 Activate Remove on the `body` row. Confirm the row disappears, and switching back to Document View shows the paragraph as empty (no placeholder copy).
- [x] 2.4 In Properties View, use the add form to add `body` with a small value on a non-document block. Confirm no error, the row appears in the generic list, and `setProperty` is called.
- [x] 2.5 In Properties View, use the add form to attempt to add `title` and `display`. Confirm each is rejected with the slot-key error and `setProperty` is NOT called.

## 3. Validation and cleanup

- [x] 3.1 Run `openspec validate all-properties-in-property-view` and confirm `valid`.
- [x] 3.2 Run `cargo build` and `cargo test` to confirm no Rust regression (no Rust code changes expected; this is a smoke check). — `cargo build` clean; `cargo test` 10/10 pass.
- [x] 3.3 Type-check the desktop app (`bun --filter @pu-erh/desktop tsc --noEmit` or equivalent project script) to confirm no TypeScript regression from the constant rename. — `bun x tsc -p tsconfig.app.json --noEmit` in `apps/desktop` exited 0.
- [x] 3.4 Confirm the description text in `RESERVED_KEYS` callsites elsewhere in the codebase (if any) reflects the rename; if there are none outside `PropertiesView.tsx`, note that in the task as a no-op. — `grep -r RESERVED_KEYS|isReserved` finds only `PropertiesView.tsx`; no-op confirmed.
