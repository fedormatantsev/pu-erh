## Context

`block-view` is a published-but-unimplemented UI contract; the desktop app is the bare `ping`/`root_id` scaffold. The available surface today:

- `core::Session`: `root_id()`, `query("children:<uuid>")` / `query("parent:<uuid>")` → `Vec<Block>`, `knowledge_base().block(id) -> Option<Block>`, `create/move/delete_block`, `save()` (writes only when dirty, calls `ensure_root`). **No property-set mutation exists.**
- `graph::KnowledgeBase`: `children(id)`, `has_children(id)`, `block(id)`, `append_block_version(id, tombstone, Properties)`.
- `graph::Block { id, properties: BTreeMap<String, PropertyValue> }`; `PropertyValue` is scalar (String/Number/Boolean/Null).
- IPC: only `ping`, `root_id`.

The five clarifying answers drive the design: reserved `title` property; Default children-tree **replaces** the raw fallback; tree is recursive with expand/collapse and click-to-select; Properties View does full `display` editing; persistence is via an **explicit Save control**; the backing mutation is a **generic `set_property`**.

This change must respect `agent-anti-default`: the only product behaviors introduced (tree expand/collapse, click-navigation, a save trigger) are exactly the ones the user explicitly specified — nothing more is invented.

## Goals / Non-Goals

**Goals:**

- A generic `set_property(block, key, value)` mutation in core, exposed on `Session`.
- Read/write IPC commands sufficient for the framework: read block, read children, set property, save.
- Presentational Block View components in `packages/ui`; all state in `apps/desktop`.
- Block View router + Inline View router dispatching on `display`, each with a default.
- A working Default Block View (recursive tree, expand/collapse, click-selects) and default Inline view (title, id fallback).
- Properties View editing `display`, persisted via an explicit Save control.

**Non-Goals:**

- Non-default renderers (chart, calendar, document editor).
- Property removal/clearing semantics, multi-value or structured properties.
- Persisting current selection or view mode; save-on-close; keyboard shortcuts.
- Drag-and-drop reparent or any mutation beyond `set_property` from the UI.

## Decisions

**D1 — `set_property` is a generic core mutation, not a display-only path.**
Per the user's choice. Implement in `crates/core/src/mutation.rs`: validate the block exists and is not tombstoned, clone its current `properties`, insert/replace `key -> value`, and `append_block_version(id, false, merged)`. Expose `Session::set_property(id, key, value)` marking the session dirty (mirrors `create/move/delete`). Root is a valid target (block-model's empty-root is a creation-time state, not a permanent invariant). *Alternative*: narrow `set_display`. Rejected — the user opted for the reusable, principled mutation.

**D2 — Value typing at the IPC boundary.**
`set_property` IPC accepts `(block_id, key, value)` where `value` deserializes into `PropertyValue`. For this change only `display`/`title` strings are written from the UI, but the command stays generic. Errors propagate as `CoreError`-derived strings without friendly rewriting.

**D3 — Read DTO carries `properties` + `has_children`.**
`block(id)` and `children(parent_id)` return blocks as `{ id, properties }` plus a `has_children` flag (from `KnowledgeBase::has_children`) so the tree can show an expand affordance before expanding. The frontend reads `display`/`title` from `properties`; the desktop crate owns the DTO and does not reimplement graph logic.

**D4 — Default Block View renders the recursive tree; children load lazily on expand.**
The Default view shows the current block's children; each node is collapsed by default and expands to its own children via a `children` IPC call. Expand/collapse state is a shell-held set of expanded ids. Clicking a node sets it as the current selected block (re-rooting the Block View). This is the Default view's selection policy under `block-view`'s "Block-View-specific selection policy". Collapsed-by-default avoids unbounded eager recursion and is the specified interaction, not an invented one.

**D5 — Two routers, both keyed on `display`, both with a default.**
The **Block View router** maps the current block's `display` to a primary renderer; the **Inline View router** maps a block's `display` to an inline renderer used when it appears inside another Block View (e.g. tree nodes). Only the Default Block View and default Inline view ship. Any `display` value (set or unset) currently resolves to the default, and an unrecognized non-empty `display` is surfaced (shown) before falling back — preserving the anti-default "surface unrecognized value" behavior from the original spec while changing the fallback target from raw to the default view.

**D6 — `title` is a UI-reserved string property; id is the unset placeholder.**
The default Inline view reads `properties.title`; when absent or non-string it shows the block id. The id is factual data, not invented copy — consistent with anti-default (no "Untitled"-style marketing label). `title` needs no `block-model` change; it is a reserved key by UI convention, exactly like `display`.

**D7 — Component/state split.**
`packages/ui` gets presentational, session-agnostic components: a tree row (label + expand toggle + click handler via props), an inline block label, a view-mode toggle, and a properties-panel layout. They take data and callbacks as props and import no Tauri APIs (preserves `frontend-scaffold`'s design-system boundary). `apps/desktop` holds all state in a React context/reducer (no new dependency): `currentBlockId` (init from `root_id`), `viewMode` (`block` | `properties`), and `expandedIds`. The shell wires IPC calls to the components and implements both routers.

**D8 — Explicit Save control; no auto-save.**
A Save button in the shell calls a `save` IPC command → `Session::save`. `set_property` mutates in memory immediately (so the view reflects the edit) but nothing is written until Save. This amends `desktop-shell`'s "No unspecified save policy" to permit this one control; timer/mutation/close auto-save remain forbidden, and Save is a button only (no Cmd+S, per the no-shortcuts requirement).

**D9 — No-root edge case surfaces the error.**
On open the shell resolves `currentBlockId` from `root_id`. A fresh, never-saved KB has no root and `root_id` returns a `CoreError`; the shell surfaces that error (per `desktop-shell` "Root id before root exists") rather than inventing an empty state. There is no current block to render until a root exists.

## Risks / Trade-offs

- **Reserved-key collisions (`title`, `display`)** → a user could intend these as data. *Mitigation*: document both as UI-reserved in `block-view`; namespacing reserved keys is future work. Acceptable for v0.
- **Lazy-expand request volume** → expanding deep trees issues many `children` calls. *Mitigation*: collapsed-by-default and per-node lazy loading bound work to what the user opens; no eager full-tree fetch.
- **Generic `set_property` enables writing arbitrary keys from a generic IPC command** → broader write surface than display-only. *Mitigation*: validation stays in core (`set_property`), the UI layer only calls it; the command surfaces `CoreError` and adds no validation of its own.
- **In-memory edit vs unsaved state divergence** → edits are visible but not persisted until Save, so closing loses them. *Mitigation*: this is the explicit, specified behavior (no save-on-close); a dirty-close prompt remains an open `block-view`/`desktop-shell` gap, not invented here.
- **Amending two requirements widens blast radius** → relaxing raw-fallback and save-policy could read as licensing more product UX. *Mitigation*: deltas are scoped to exactly the children-tree default and the single Save control; all other anti-default guards are restated.

## Open Questions

- Dirty-session close behavior (warn / discard / save) — left as an explicit gap, not resolved here.
- How a future renderer registers supported `display` values, and the concrete renderer set — deferred to renderer-specific changes.
- Whether `title`/`display` should move from convention to a typed/namespaced reserved-key scheme in `block-model` — deferred.
