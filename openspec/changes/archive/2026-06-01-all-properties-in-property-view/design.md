## Context

The Properties View today filters its generic list against an in-component constant `RESERVED_KEYS = ["title", "display", "body"]`. The same constant gates the add-form's reserved-key error. Two specs reinforce this:

- `properties-view`, Requirement: **Well-known property layout slots** → generic list excludes "any property key with a dedicated layout slot or reserved semantics in `property-registry` (including `body`)".
- `property-registry`, Requirement: **body property** → `body` "MUST be excluded from generic user-editable property lists per `properties-view`".

The motivating bug: opening a `display=document` block's child paragraph in Properties View shows zero generic rows even though the paragraph carries `body`. A user can neither inspect nor remove that value from Properties View. The block-graph data model has only one kind of property storage (`BTreeMap<String, PropertyValue>`), so the user's mental model — "every property is on the block; let me see them" — is correct, and the UI is failing it.

The change is small, local to the Properties View surface, and orthogonal to write semantics (`set_property` / `remove_property` already work on `body`).

## Goals / Non-Goals

**Goals:**
- The generic properties list shows every property that does not have a dedicated layout slot (`title`, `display`).
- `body` is visible and removable from the Properties View on any block that carries it.
- The add-property form accepts any non-slot key, including `body`.
- The property-registry stops asserting Properties-View visibility; that concern moves entirely to the `properties-view` spec.

**Non-Goals:**
- No new editing UX for `body` in the Properties View. The value is shown using the same read-only `key: value` row template as other generic rows. Edits to rich text still happen in the Document View.
- No truncation, formatting, or pretty-printing of long values. Anti-default: the spec does not call for it; ship the raw stringified value.
- No new mutation, IPC, or session API.
- No change to write-side semantics of `body` — the Document View still owns its production.
- No keyboard shortcut or focus polish.

## Decisions

### Decision: Inclusion rule keyed on "dedicated slot", not "reserved"

The spec moves from two-condition exclusion (dedicated slot OR reserved semantics) to single-condition (dedicated slot only). Reasons:

- The user's question "what properties does this block have?" has exactly one correct answer: the keys in `block.properties`. The UI must not silently subset that answer.
- "Reserved semantics" is a domain concept; UI visibility is an introspection concern. Coupling them in two specs (`property-registry` mentioning generic-list exclusion) was a leak. The change cleans that up.
- A future reserved key without a dedicated slot (say a future `created_at`) will get free correct behaviour: visible in generic list, blockable in add form only if we add a slot.

Alternative considered: give `body` a dedicated slot showing the raw string. Rejected — it would force a layout decision (where does the third slot sit? above/below `display`?) and add a permanent UI affordance for what is, on most blocks, opaque payload. The Properties View already renders generic rows; surfacing `body` there is free.

### Decision: Add-form check filters on dedicated-slot keys only

The form must still prevent adding `title` and `display`, because dedicated slots own them and a duplicate add would create incoherent UI state (two surfaces, one value). Other keys, including `body`, may be added freely. The user might create `body` on a non-document block — that is just a string property with no Document View consumer, which is fine.

Alternative considered: also block `body`. Rejected — the user explicitly asked to relax the rule, and `body` on a non-document block has no harm.

### Decision: Source of truth for "dedicated slot" set lives in the Properties View code

The spec lists `title` and `display` as the dedicated-slot keys. The component already encodes both — the title input and the display dropdown are slot one and slot two. To implement the new rule, replace `RESERVED_KEYS = ["title", "display", "body"]` with `SLOT_KEYS = ["title", "display"]` (constant rename). No registry import needed; the design intentionally keeps the slot set static in the view.

Alternative considered: derive the slot set from `property-registry` metadata. Rejected — there is no slot metadata in the registry today, and inventing one would expand scope. If a third slot is ever added, the constant grows by one line; that is cheaper than a registry refactor.

### Decision: Generic row template unchanged

Each generic row still renders `<span>{key}: {value}</span>` plus a Remove button. `body` will render as `body: {long string}` in line. This is anti-default-compliant (the spec is silent on display formatting) and trivially implementable.

## Risks / Trade-offs

- **Long `body` strings make the panel ugly.** → Mitigation: out of scope; if it becomes annoying, propose a follow-up to add a "show full / truncate" affordance. The anti-default principle says we don't preemptively decorate.
- **User removes `body` from a paragraph block via Properties View, then the Document View shows an empty paragraph.** → This is correct behaviour. `remove_property` is the documented way to clear a property; the Document View already handles missing `body` per its spec ("renders empty with no placeholder copy"). No mitigation needed.
- **Risk: spec drift between `properties-view` and `property-registry`.** → Mitigation: the modified `property-registry` body requirement now defers visibility entirely to `properties-view` and adds a scenario explicitly cross-referencing it. The two specs are kept consistent by the deltas, not by code review of future edits.
- **Trade-off: no editing of `body` from Properties View.** Users with corrupt `body` JSON cannot fix it from this surface, only remove it. Acceptable for now — fixing rich-text payload by hand is an escape hatch, not a primary flow.

## Migration Plan

No data migration. No persisted state changes. Implementation is a single component edit plus two spec deltas.
