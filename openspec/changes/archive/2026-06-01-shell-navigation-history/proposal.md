## Why

The current selected block changes whenever the user activates a different block, but there is no way to go back to where they just were. As users build deeper block hierarchies, recovering the previous context requires manually navigating back through the tree — a friction point that breaks focused attention.

## What Changes

- The shell records a navigation history stack each time the current selected block changes
- A backward action navigates to the most recently visited block before the current one
- A forward action navigates to the next entry when the user has gone back
- Backward and forward actions appear in the action bar as shell-level actions (always present, enabled only when history allows)
- The history is ephemeral: it resets when the session is closed or re-opened
- Navigating to a new block while positioned mid-history clears the forward stack

## Capabilities

### New Capabilities

- `navigation-history`: Defines the navigation history stack — recording policy, backward/forward semantics, forward-stack invalidation, and the action descriptors

### Modified Capabilities

- `action-bar`: Add requirement that the action bar renders a shell-level navigation section (backward/forward) in addition to the view-specific action list
- `desktop-shell-ui`: Add requirement that the shell records each selected-block change into the navigation history and exposes navigate-back and navigate-forward operations

## Impact

- `apps/desktop` shell — new ephemeral state: history stack (array of block IDs) and forward stack; no backend changes
- Action bar component — new backward/forward actions (design-system presentational additions)
- No Rust/core/storage changes required
