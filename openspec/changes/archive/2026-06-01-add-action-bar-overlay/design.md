## Context

The desktop shell (`apps/desktop`) renders the current selected block through the Block View / Properties View, holding all view state in the shell (`shell.tsx`). The view-mode toggle is currently rendered inline at the top of `Workspace` via the design-system `ViewModeToggle`. There is no UI affordance to mutate the graph; `create_block` exists in `core::Session` and the `mutations` capability, but no IPC command exposes it.

Two policies bound this change:
- **Anti-default** (`AGENTS.md`, `agent-anti-default`): no invented product UX. Selection model, save timing, and any surface beyond Block/Properties View must be specified, not assumed.
- **`desktop-shell`**: forbids surfaces beyond the Block View and Properties View unless an explicit requirement introduces them; forbids any automatic/implicit save.
- **`ui-direction`**: chrome must be subordinate (low contrast, compact, no decorative borders), content stays dominant.

## Goals / Non-Goals

**Goals:**
- A floating, upper-left overlay surface that does not displace or compete with content.
- Exactly two actions: toggle view mode, and create a child of the current selected block.
- Reuse the existing `create_block` mutation through a thin IPC command and adapter method.
- Presentational `ActionBar` lives in the design system and is session-agnostic; all state and wiring live in the shell.

**Non-Goals:**
- No selection change on create (the new child does not become current).
- No save behavior change — create is held in memory until explicit Save.
- No additional actions (move, delete, rename), no drag/drop, no keyboard shortcuts for the bar.
- No "adaptive" behavior beyond reflecting current shell state (active mode; presence of a current selected block).

## Decisions

**1. New `action-bar` capability rather than folding into `block-view`.**
The bar is a shell-level overlay surface, not a Block View renderer. A dedicated capability keeps its action set and adaptivity explicit. `block-view`'s mode-exclusivity requirement is unchanged; only where the toggle is presented moves. Alternative considered: extend `block-view` — rejected because the bar also hosts a mutation action unrelated to rendering.

**2. Toggle moves into the bar; inline `ViewModeToggle` placement is removed.**
Avoids two competing toggle surfaces. The `ViewModeToggle` design-system component may be reused inside `ActionBar` or the bar may render its own toggle control; either way only one toggle is visible. The shell continues to own `viewMode`.

**3. Create-child semantics — explicit, anti-default-safe.**
- Calls `Session::create_block(Some(currentBlockId))` via a new `create_block` IPC command.
- Does **not** change the current selected block (no "selected node" inference).
- Does **not** save (consistent with `desktop-shell` no-save policy); the version record is appended in memory and persisted only on explicit Save.
- Disabled when there is no current selected block (e.g. unresolved root). Per anti-default, a silent gap becomes a disabled control, not a guessed root-creation flow.
- After a successful create, the Block View must re-read the current block's children so the new child appears in the children column. The shell exposes a refresh signal (e.g. a bump counter passed to the view host) rather than the bar reaching into the view.

**4. Surfacing errors.** The `create_block` command returns `CoreError`-derived strings as-is (no friendly rewrite), surfaced the same way `rootError` is.

**5. Positioning.** `position: fixed` upper-left, compact, low-contrast chrome per `ui-direction`; no decorative border. It overlays whichever view is active without changing the content layout.

## Risks / Trade-offs

- [Overlay occludes content in the upper-left] → Keep the bar compact and low-contrast; it is a small panel, not a sidebar. Content remains full-width beneath it.
- [User expects the new child to be selected/focused] → Explicitly specified as no selection change; the child is visible in the children column. Revisit only via a future requirement.
- [User expects create to persist] → No-save policy is retained and surfaced through the existing explicit Save control; documented in the spec scenario.
- [Two toggle surfaces during transition] → The inline placement is removed in the same change so only the bar's toggle remains.

## Migration Plan

Additive UI change; no data migration. New IPC command and capability/permission entry are additive. Rollback is removal of the `ActionBar` wiring and `create_block` command.

## Open Questions

None blocking. Future changes may add more actions or a selection-on-create policy, each gated by an explicit requirement.
