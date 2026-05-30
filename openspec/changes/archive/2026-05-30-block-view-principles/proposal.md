## Why

The desktop frontend currently ships only a bare, neutral shell: `frontend-scaffold` and `desktop-shell` deliberately forbid any main content surface, current-block concept, or selection model, and `agent-anti-default` forbids inventing one. Before any block can be rendered or edited, the project needs a normative contract for how blocks are shown and configured — a *defining spec* for the "current block" session state and the main interaction surface that the anti-default policy explicitly defers to ("MUST NOT add implicit 'current block' query context **unless a spec defines that session state**").

This change defines those principles only. It ships no React/Tauri code; implementation lands in a later change against this contract.

## What Changes

- Introduce the **Block View** as the primary UI surface, which renders the **current selected block** and chooses a representation (e.g. Document Editor, Chart, Calendar) per the block's `display` property.
- Define the **current selected block**: exactly one always exists, it dictates what the UI renders, and on open with no prior selection it resolves to the unique root block.
- Define `display` as a **reserved string property key** on a block (value `PropertyValue::String` naming a renderer), edited only through the Properties View via existing `core::Session` property mutations — no new mutation path.
- Define the **raw fallback renderer** used when the current block has no `display` set (an explicit, specified default — not an invented one).
- Define the **Properties View** as the settings surface for the current Block View, including where `display` is changed.
- Define **mode exclusivity**: Block View and Properties View are switchable but never shown at once; the mode is ephemeral UI state.
- Define **primary vs inline** block rendering (current block = primary; child/collection member = inline, represented by the containing Block View).
- Define that **selection policy is Block-View-specific**: each Block View implementation owns its own selection/navigation policy; no global "click child = navigate" rule is mandated.
- **BREAKING (contract-level)**: amend `desktop-shell` and `frontend-scaffold` so their "bare/neutral shell, no main surface" requirements are superseded by the Block View contract, while their other anti-default prohibitions (no auto-save, no shortcuts/menus/themes, design-system stays presentational) are retained.

## Capabilities

### New Capabilities

- `block-view`: The current-selected-block session concept, the Block View main surface and its `display`-driven renderer dispatch, the raw fallback renderer, the Properties View settings surface, mode exclusivity, primary-vs-inline rendering, and Block-View-owned selection policy.

### Modified Capabilities

- `desktop-shell`: The "Anti-default UI shell / bare shell content" requirement is amended so the desktop window's main content surface is the Block View defined by `block-view`; the no-auto-save, no-shortcut-menu, no-theme, and no-welcome-copy prohibitions are retained.
- `frontend-scaffold`: The "Neutral empty shell in app" requirement is amended so the app's main surface follows `block-view`; the "No product UX in design system" boundary (`packages/ui` stays presentational, assumes no current block/selection) is retained unchanged.

## Impact

- **Specs**: new `openspec/specs/block-view/`; delta edits to `desktop-shell` and `frontend-scaffold`. Relies on (does not change) `block-model` (`display` is an ordinary reserved property key) and the `agent-anti-default` escape clause for defined session state.
- **Code**: none in this change. A follow-up implementation change will build Block View / Properties View / display dispatch in `apps/desktop`, coordinating `core::Session` only.
- **Deferred**: the concrete renderer set (Document Editor, Chart, Calendar, …), persistence of current selection and view mode, and multi-window behavior are out of scope and left as explicit gaps for future changes.
