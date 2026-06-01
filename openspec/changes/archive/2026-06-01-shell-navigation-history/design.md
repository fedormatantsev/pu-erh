## Context

The shell (`apps/desktop/src/shell.tsx`) currently holds `currentBlockId` as a single state value. Selection changes via `selectBlock(id)`, which does a bare `setCurrentBlockId`. There is no concept of where the user came from or how to get back. The action bar (`App.tsx`) builds its action list from the active view's descriptor factory — currently block-toggle and create-child for the Tree View, toggle-only for the Properties View. Backward/forward are shell-level, not view-specific, and require a new composition point in the action list.

## Goals / Non-Goals

**Goals:**
- Record a backward/forward history stack in the shell on every `selectBlock` call
- Expose `navigateBack` and `navigateForward` operations from the shell
- Surface backward/forward as action bar entries, always visible, disabled when unavailable
- Forward stack clears on any new block navigation

**Non-Goals:**
- Persisting history across sessions
- History for view mode changes (Block View ↔ Properties View is not block navigation)
- Keyboard shortcuts for back/forward (no spec for shortcuts exists)
- Limiting history depth (unbounded at v0)

## Decisions

### 1. History state: two stacks in ShellProvider

**Decision**: Add `backStack: string[]` and `forwardStack: string[]` to `ShellProvider` state alongside `currentBlockId`.

**Data model**:
- `backStack` — block IDs visited before the current one, oldest first; `back` pops from the end.
- `forwardStack` — block IDs vacated by going back, most-recently-vacated first; `forward` pops from the front.

**Navigate to new block** (via `selectBlock`):
1. Push `currentBlockId` to end of `backStack`
2. Clear `forwardStack`
3. Set `currentBlockId` to new id

**Navigate back** (via `navigateBack`):
1. Pop last entry from `backStack` → `target`
2. Push `currentBlockId` to front of `forwardStack`
3. Set `currentBlockId` to `target`

**Navigate forward** (via `navigateForward`):
1. Pop first entry from `forwardStack` → `target`
2. Push `currentBlockId` to end of `backStack`
3. Set `currentBlockId` to `target`

**Initial root load does not push history**: `setCurrentBlockId` is called directly from the `useEffect` that resolves the root — this is bootstrap, not user navigation.

**Rationale**: All selection state lives in the shell today. Adding the stacks there keeps the pattern consistent and requires no new context or abstraction.

### 2. Action composition: shell prepends navigation actions

**Decision**: `App.tsx` prepends backward/forward action descriptors to the view-specific action list before passing to `ActionBar`. The `viewRouter` is not modified.

**Alternative considered**: Thread `navigateBack`/`navigateForward` through `ViewActionContext` and have each view descriptor include them. Rejected — view descriptors should describe view-specific actions; shell navigation is always the same regardless of view mode.

**Alternative considered**: A separate dedicated navigation bar. Rejected — adds surface chrome not specified by any existing spec; the action bar is the right place for compact shell-level actions.

**Rationale**: Composing in `App.tsx` is the minimal, local change. The action-bar spec is updated to permit this shell-level prepend as a first-class pattern.

### 3. Back/forward entries are always in the action bar

**Decision**: Backward and forward entries are always rendered in the action bar. When no history is available, they are rendered as disabled. They are never hidden.

**Rationale**: Showing disabled states communicates the capability exists and gives users a consistent UI anchor. Hiding them entirely would cause the action bar to change width unpredictably on navigation.

## Risks / Trade-offs

- **Stale block IDs in history** → If a block is deleted, its ID remains in the history stacks. Navigating back to it will fail when the view tries to load the block (it will be tombstoned). The `desktop-shell-ui` spec already requires re-resolving to root when the current block becomes invalid — the same recovery path applies here. No special handling is needed at v0.
- **Forward stack cleared unexpectedly** → A user who goes back and then creates a child loses their forward stack. This is standard browser behavior and matches user expectations.
- **Unbounded history** → Memory impact is negligible for block ID strings; no cap is needed at v0.
