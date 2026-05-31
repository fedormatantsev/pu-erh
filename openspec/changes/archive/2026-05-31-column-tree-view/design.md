## Context

The default Block View renderer (`apps/desktop/src/views/blockView.tsx`, registered as `tree`) renders the current selected block's children as a recursive, lazily-loaded, expand/collapse tree. Expand/collapse state lives in the shell (`expandedIds`/`toggleExpanded` in `apps/desktop/src/shell.tsx`). The presentational `TreeNode` component in `@pu-erh/ui` owns the expand affordance.

This change replaces that renderer with a three-column TreeView. The relevant constraints:

- **Anti-default policy** (`AGENTS.md`): selection and navigation are product behavior. The behavior implemented here is exactly what the proposal/specs state — single-parent left column, current+siblings center, children right; activation + the four arrow keys with no-op edges — and nothing more.
- **Thin adapters**: the renderer and shell coordinate `core::Session` via IPC only; no graph/trie logic in the UI. Presentational pieces in `@pu-erh/ui` stay session-agnostic (props + callbacks, no IPC).
- The backend query language already supports `parent:<uuid>`; only an IPC command and an `AppState` method are missing.

## Goals / Non-Goals

**Goals:**
- Render the current selected block as parent | current+siblings | children.
- Move the current selected block via click and via `←`/`→`/`↑`/`↓`, re-centering each time.
- Keep all derivable state out of the shell: columns derive from the current selected block; no expand/collapse state.
- Minimalist visual structure driven by spacing and typography.

**Non-Goals:**
- No recursive/multi-level expansion within a column (the right column shows one level of children only).
- No mutations (move/create/delete) from the TreeView.
- No persistence of selection or scroll position.
- No new Block View renderers; `tree` remains the only registered renderer.
- No `parent:`/`children:` query-language changes.

## Decisions

### D1: TreeView replaces the `tree` renderer in place

`DefaultBlockView`/`TreeBranch` are removed; the column TreeView is registered under the existing `tree` key, so `BLOCK_VIEW_NAMES`, the Properties `display` dropdown, and stored `display: "tree"` values keep working without migration. **Alternative considered:** register under a new key (e.g. `columns`) and keep both. Rejected per the locked decision to replace the recursive view; avoids carrying dead code and a second renderer the spec does not require.

### D2: Columns derive entirely from the current selected block

Given the current selected block `C`, the shell resolves three reads: `parent(C)` (left), `children(parent(C))` or, when `C` is root, `[C]` (center siblings), and `children(C)` (right). The current block is found within the center list to compute previous/next sibling and to highlight it. There is no separate cursor or expand state — the single source of truth is `currentBlockId` in the shell. **Alternative considered:** an ephemeral cursor distinct from the current selected block. Rejected per the locked decision (arrows move the current selected block and re-center).

Sibling resolution requires the parent's children list. When `C` is the root it has no parent, so the center column contains just the root and the parent column is empty.

### D3: New `parent` IPC command, mirroring `children`

Add `AppState::parent(&str) -> Result<Option<BlockDto>, String>` wrapping `session.query("parent:<id>")` and mapping the (zero-or-one) result through `BlockDto::new` with `kb.has_children`. Expose it as a `parent` Tauri command (registered in `lib.rs`, with a capability/permission file mirroring `allow-children.toml`), and add `getParent(id): Promise<BlockDto | null>` to `ipc.ts`. Returning `null` for the root keeps the left column empty without a special error. **Alternative considered:** derive the parent on the frontend from a `children` walk — rejected; the backend already answers `parent:` directly and the adapter must mirror the session, not reinvent traversal.

### D4: Arrow-key navigation owned by the shell, listening at the Block View surface

The TreeView host attaches a `keydown` handler (on a focusable container, not `window`) that maps `←/→/↑/↓` to the next current block id and calls `selectBlock`. Edge cases resolve to no-ops: `←` with `null` parent, `→` with empty children, `↑`/`↓` at the ends of the sibling list. Keeping the mapping in the shell/host (which already has the column data) honors the "shell owns Block View state" requirement and keeps `@pu-erh/ui` components dumb. **Alternative considered:** per-column roving-tabindex handlers inside `@pu-erh/ui`. Rejected: it would push selection policy into the session-agnostic design system.

### D5: `@pu-erh/ui` gains a column-grid primitive; `TreeNode` is retired

Add presentational components for the three-column layout (a column container and a column-of-inline-blocks list with a "current" highlight) and style them in `styles.css` using spacing/typography for structure. `TreeNode` (and its expand toggle) is removed or repurposed since no view uses it after this change. `InlineBlock` is reused unchanged for each cell. The showcase is updated if it references `TreeNode`.

### D6: Data loading and ordering

Each column is loaded via the existing async IPC pattern (`useEffect` + cancellation flag) keyed on the relevant id. The `children:` query result order is **unspecified** per the `query-language` spec, so the TreeView does not impose its own ordering rule. Instead, `↑`/`↓` navigate the center column **in the order it was returned and is currently rendered** — previous/next sibling is defined relative to the displayed list, which is internally consistent within a render. Loading/empty states render nothing (no invented placeholder copy); IPC errors render the returned `CoreError` string via `Text`, consistent with existing views.

## Risks / Trade-offs

- **Extra IPC round-trips per selection (parent + two children reads)** → Acceptable for an in-process Tauri session; reads are cheap and already used per-render elsewhere. Revisit with batching only if profiling shows a problem.
- **Keyboard focus management** → The host container must be focusable and receive focus so arrow keys work without stealing browser/OS shortcuts; mitigate by scoping the handler to the Block View container and only intercepting the four arrow keys.
- **Sibling index correctness depends on stable `children:` ordering** → Rely on the documented query order; if ordering is unspecified/unstable this would make `↑`/`↓` jumpy. Surface as an open question if order is not deterministic.
- **Removing `TreeNode` is a breaking change to `@pu-erh/ui`'s public surface** → Internal monorepo consumer only; update showcase in the same change.

## Open Questions

- None outstanding. (The `children:` order question is resolved by D6: navigation is defined relative to the rendered order, not an invented sort.)
