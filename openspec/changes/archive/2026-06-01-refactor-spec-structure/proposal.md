## Why

The product core is formed, but OpenSpec capabilities grew chronologically from walking-skeleton changes rather than by mental model. The `block-view` spec is a kitchen-sink (selection, routers, tree renderer, properties panel), property semantics are duplicated across three specs, and cross-cutting UI rules are copy-pasted with real conflicts (e.g. `title` placeholder behavior). With more Block View renderers coming, the spec tree needs restructuring before further development.

## What Changes

- Add **`property-registry`** — canonical domain semantics for reserved property keys (`title`, `display`, `body`, `order` registry table).
- Add **`desktop-shell-ui`** — app navigation framework: selection, view modes, unified renderer registry (primary + inline dispatch by `display`), default inline renderer, View Router.
- Add **`tree-view`** — default primary Block View renderer (3-column Tree View + selection policy); registers as default primary.
- Add **`properties-view`** — Properties View layout and well-known property editing slots.
- **Retire `block-view`** — requirements migrate to the four capabilities above.
- **Retire `well-known-properties`** — domain parts → `property-registry`; UI parts → `properties-view`.
- **Retire `knowledge-base-bootstrap`** — single requirement moves to `desktop-shell` as interim Desktop open policy.
- **Modify `agent-anti-default`** — add consolidated UI adapter requirements (Session-only, adapter-defined persistence); dedupe from view specs.
- **Modify `desktop-shell`** — absorb bootstrap; document interim open policy until future autosave storage engine.
- **Modify `document-view`, `action-bar`, `block-model`, `ui-direction`** — slim to normative cross-refs; fix hygiene.
- Add **`openspec/CAPABILITIES.md`** — informative reading order and layer map (non-normative).
- Establish normative cross-reference convention: `**capability**`, Requirement: **Name**.
- **BREAKING**: Capability names `block-view`, `well-known-properties`, and `knowledge-base-bootstrap` are removed; agents and docs must use new names.

## Capabilities

### New Capabilities

- `property-registry`: Reserved block and edge property keys, types, and semantic rules (including absent-`title` behavior by context).
- `desktop-shell-ui`: Current selected block, Block/Properties mode exclusivity, unified renderer registry keyed by `display` (primary + inline dispatch, primary-only entries, default inline renderer), View Router, thin-adapter placement rules.
- `tree-view`: Default primary renderer — 3-column Tree View, activation and arrow-key selection policy; inline blocks via `desktop-shell-ui` renderer registry.
- `properties-view`: Properties View surface, well-known property layout slots (`title`, `display`), generic property list exclusions.

### Modified Capabilities

- `agent-anti-default`: Add consolidated requirements for UI Session-only coordination and adapter-defined persistence timing.
- `desktop-shell`: Add Desktop open policy (interim bootstrap on first open); note future storage-engine autosave supersedes current save/bootstrap story.
- `document-view`: Register as primary-only renderer (`display=document`); slim duplicated save/adapter/title rules to cross-refs.
- `action-bar`: Deduplicate requirements; reference `tree-view` and `desktop-shell-ui` by capability name.
- `block-model`: Trim Purpose — active-read semantics defer to `immutable-snapshot`.
- `ui-direction`: Fix malformed header (add title and Purpose section).
- `session`: Add note that adapters MAY define open-time persistence policies in adapter capabilities.

## Impact

- **Specs only** — no application code changes in this change.
- Retired capability folders removed from `openspec/specs/` after migration.
- `AGENTS.md`, `openspec/config.yaml` context, and agent rules should reference new capability names and reading order.
- Stale `openspec/changes/structured-document-view/` folder deleted.
- Archived changes under `openspec/changes/archive/` remain historical (no rewrite required).
