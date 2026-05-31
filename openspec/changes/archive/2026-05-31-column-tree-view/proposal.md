## Why

The current default Block View renders the selected block's children as a single recursive expand/collapse tree. It shows descent but not ascent: the user cannot see where the current block sits relative to its parent and siblings, and there is no keyboard navigation. A three-column view makes the block's position in the hierarchy legible at a glance and lets the user move through the graph with arrow keys.

## What Changes

- **BREAKING** Replace the default `tree` Block View renderer (recursive expand/collapse tree) with a three-column **TreeView**:
  - Left column: the **single parent** of the current selected block (empty when the current block is the root).
  - Center column: the current selected block together with its siblings (those above and below it), the current block highlighted.
  - Right column: the **children** of the current selected block (empty when it has none).
- Add **arrow-key navigation** that moves the current selected block and re-centers the columns: `←` selects the parent, `→` selects the first child, `↑`/`↓` select the previous/next sibling. Moves are no-ops at edges (e.g. `→` with no children, `←` at the root, `↑`/`↓` past the ends of the sibling list).
- Keep **click-to-select**: activating a block's inline preview in any column makes it the current selected block (the TreeView's selection policy), re-centering the columns.
- Add a `parent` read command to the desktop shell IPC surface (thin wrapper over the `parent:<uuid>` query) so the shell can resolve the left column. Remove the now-unused per-node expand/collapse state from the shell.
- Add presentational, session-agnostic column/grid building blocks to `@pu-erh/ui`; remove or repurpose the recursive `TreeNode` expand affordance. Styling is minimalist, using spacing and typography to communicate structure.

## Capabilities

### New Capabilities

<!-- None: TreeView is a renderer within the existing block-view capability. -->

### Modified Capabilities

- `block-view`: The **Default Block View** requirement changes from a recursive expand/collapse children tree to the three-column TreeView; its selection policy gains arrow-key navigation alongside activation. The **Block View component and state placement** requirement changes from expand/collapse state to column-derived state.
- `desktop-shell`: The **Block-view IPC commands** requirement gains a `parent` read command. The **Anti-default UI shell** requirement's description of the Default Block View is updated to the three-column TreeView with arrow-key navigation.
- `design-showcase`: The **Showcase displays all base components** requirement gains a live TreeView column example rendered with static sample data.

## Impact

- `apps/desktop/src/views/blockView.tsx` — replace `DefaultBlockView`/`TreeBranch` with the three-column TreeView; keep the renderer registered under `tree`.
- `apps/desktop/src/shell.tsx` — drop `expandedIds`/`toggleExpanded`; add arrow-key navigation wiring against the column data.
- `apps/desktop/src/ipc.ts`, `apps/desktop/src-tauri/src/commands.rs`, `crates/desktop/src/state.rs`, `apps/desktop/src-tauri/src/lib.rs`, plus a Tauri permission file — add the `parent` command.
- `packages/ui/src/components.tsx`, `packages/ui/src/styles.css` — add column layout components; retire/repurpose `TreeNode`.
- `apps/showcase/src/App.tsx` — add a live TreeView column example (static sample data) to the Components section; update/remove any entry referencing the recursive tree node.
- No changes to `core`, `graph`, `storage`, or the query language (the `parent:` query already exists).
