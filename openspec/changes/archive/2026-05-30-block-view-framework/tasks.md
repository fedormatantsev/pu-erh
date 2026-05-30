## 1. Core: set_property mutation

- [x] 1.1 Add `set_property(kb, block, key, value)` to `crates/core/src/mutation.rs`: reject if block absent from the active view; clone current properties, set `key = value`, append a new block version record
- [x] 1.2 Add `Session::set_property(id, key, value)` in `crates/core/src/session.rs` that calls the mutation and marks the session dirty
- [x] 1.3 Add unit tests: set on existing block, overwrite existing key (other keys unchanged), set on nonexistent block errors and appends nothing

## 2. Desktop crate: state methods and DTOs

- [x] 2.1 Add a serializable block DTO in `crates/desktop` (`id`, `properties`, `has_children`) built from `KnowledgeBase`
- [x] 2.2 Add `AppState::block(id)`, `AppState::children(id)` (mirroring the `children:` query), `AppState::set_property(id, key, value)`, and `AppState::save()`, each locking the session and mapping `CoreError` to a string without rewriting
- [x] 2.3 Add tests for `children` and `set_property` round-trips against a temp session

## 3. IPC commands

- [x] 3.1 Add `block`, `children`, `set_property`, and `save` Tauri commands in `apps/desktop/src-tauri/src/commands.rs` as thin wrappers over `AppState`
- [x] 3.2 Register the new commands in the Tauri handler in `apps/desktop/src-tauri/src/lib.rs`

## 4. Design system components (packages/ui)

- [x] 4.1 Add a presentational tree-node component (label, expand/collapse toggle, activate callback) taking data and callbacks as props — no Tauri, no current-block assumption
- [x] 4.2 Add a presentational inline-block label component
- [x] 4.3 Add a presentational view-mode toggle and a properties-panel layout component
- [x] 4.4 Export the new components from `packages/ui/src/index.ts`

## 5. Shell state management (apps/desktop)

- [x] 5.1 Add a typed IPC client module wrapping the `block`/`children`/`set_property`/`save` invokes, surfacing errors as returned
- [x] 5.2 Add a shell state container (React context/reducer, no new dependency) holding `currentBlockId` (initialized from `root_id`, surfacing the `CoreError` when no root exists), `viewMode` (`block` | `properties`), and `expandedIds`

## 6. Routers and views

- [x] 6.1 Implement the Block View router: map current block `display` → renderer, default fallback, surface unrecognized value
- [x] 6.2 Implement the Inline View router with the same unset/unrecognized handling and a default inline renderer
- [x] 6.3 Implement the default Inline view: show `title` string property, fall back to the block id
- [x] 6.4 Implement the Default Block View: recursive children tree using the ui tree-node components, lazy children load on expand via IPC, expand/collapse via `expandedIds`, click a node → set `currentBlockId`
- [x] 6.5 Implement the Properties View: show and edit the current block's `display`, calling `set_property`; add an explicit Save control calling the `save` IPC
- [x] 6.6 Wire the view-mode toggle so exactly one of Block View / Properties View is shown; replace the ping/root_id scaffold in `App.tsx` with the framework

## 7. Verify

- [x] 7.1 `cargo test` passes (core + desktop crate)
- [x] 7.2 Frontend typechecks and the Vite/Tauri dev build runs without manual bundler config
- [x] 7.3 `openspec validate block-view-framework --strict` passes
