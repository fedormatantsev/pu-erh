## 1. Author the block-view capability spec

- [x] 1.1 Write `specs/block-view/spec.md` with the current-selected-block requirement (always one, resolves to a valid block, defaults to root on open)
- [x] 1.2 Add the Block View primary-surface and `display`-driven renderer-dispatch requirements
- [x] 1.3 Add the `display` reserved-string-property requirement (existing properties map, existing `core::Session` mutation path, changed only via Properties View)
- [x] 1.4 Add the raw fallback renderer requirement (unset `display` → raw; unrecognized value → surface value then raw)
- [x] 1.5 Add the Properties View, mode-exclusivity, primary-vs-inline rendering, Block-View-specific selection, and thin-adapter requirements

## 2. Amend the shell specs it supersedes

- [x] 2.1 Write the `desktop-shell` delta MODIFYING "Anti-default UI shell" so the main surface is the Block View while retaining no-auto-save / no-shortcut / no-theme / no-welcome guards
- [x] 2.2 Write the `frontend-scaffold` delta MODIFYING "Neutral empty shell in app" so the main surface follows `block-view`, retaining the design-system presentational boundary unchanged

## 3. Validate and reconcile

- [x] 3.1 Run `openspec validate block-view-principles --strict` and resolve any errors
- [x] 3.2 Confirm the MODIFIED requirement headers match the existing `desktop-shell` and `frontend-scaffold` specs exactly (whitespace-insensitive)
- [x] 3.3 Confirm `block-model`, `mutations`, and `session` are only referenced, not modified (no new mutation API introduced for `display`)
- [x] 3.4 Confirm the change relies on the `agent-anti-default` escape clause for defined "current block" session state without weakening any anti-default requirement

## 4. Implementation hand-off (no code in this change)

- [x] 4.1 Record that Block View / Properties View / renderer dispatch implementation in `apps/desktop` is a separate future change validated against `block-view`
- [x] 4.2 Record the deferred gaps as future work: concrete renderer registry, selection/view-mode persistence, inline hierarchy recursion limits, multi-window behavior
