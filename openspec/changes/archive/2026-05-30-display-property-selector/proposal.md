## Why

The `display` property is currently a free-text input in the Properties View, but it is constrained to a closed set of registered block views — a free-text field invites typos and exposes no guidance to the user. Introducing a typed, constrained selector makes the `display` property self-documenting and eliminates invalid values at the point of entry.

## What Changes

- The Properties View renders the `display` property as a dropdown (one-of selector) populated by the registered block views available in the app. The property title label is suppressed — only the dropdown is shown.
- If a block has no `display` property, the dropdown implicitly shows the default view. If the stored value is unrecognized, the dropdown silently falls back to the default and overwrites the invalid value on the next save.
- A new concept of **well-known predefined properties** is introduced in the desktop adapter layer. Well-known properties are rendered by specialized UI controls rather than generic text inputs. `display` is the first well-known property.
- No changes to core, graph, storage, or CLI crates.

## Capabilities

### New Capabilities

- `well-known-properties`: Defines the concept of well-known predefined properties in the desktop adapter, their registry, and the rule that they are rendered by specialized UI controls in the Properties View rather than generic inputs.

### Modified Capabilities

- `block-view`: The requirement "Properties View exposes display" changes: `display` MUST now be presented as a constrained one-of selector (not a free-text field), with absent/unrecognized values resolved to the default implicitly and overwritten on save.

## Impact

- `apps/desktop/src/views/PropertiesView.tsx`: Replace the free-text `display` input with a dropdown; add absent/unknown fallback and overwrite-on-save logic.
- `apps/desktop/src/views/blockView.tsx`: Export the list of registered view names (including the default) so the Properties View can populate the dropdown.
- `apps/desktop/src/wellKnownProperties.ts` (new): Registry mapping well-known property keys to their display metadata (type, options, default, render style).
- No changes outside `apps/desktop/src/`.
