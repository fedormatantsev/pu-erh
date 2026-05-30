## Context

pu-erh's desktop frontend exists only as a bare scaffold. `desktop-shell` ("Anti-default UI shell") and `frontend-scaffold` ("Neutral empty shell in app") forbid any main content surface, and `agent-anti-default` forbids inventing a "current block" or selection model — but it carves out an explicit escape clause: such session state is allowed once *a spec defines it* ("MUST NOT add implicit 'current block' query context **unless a spec defines that session state**").

This change is that defining spec. It establishes the UI contract — current selected block, Block View, Properties View, the `display` property, primary/inline rendering — without shipping code. The constraints come from the user's six stated principles plus the answers gathered during proposal:

- **Scope**: spec/principles only, plus amendments to `desktop-shell` and `frontend-scaffold`.
- **`display`**: a reserved string key in the existing block properties map; value is a `PropertyValue::String` renderer id, edited via existing `core::Session` property mutations.
- **Unset `display`**: explicitly falls back to a built-in raw renderer (an opted-in specified default).
- **Selection**: each Block View implementation owns its own selection/navigation policy.

## Goals / Non-Goals

**Goals:**

- Define the current-selected-block session concept and that it always resolves to exactly one valid block.
- Define the Block View as the main surface and `display`-driven renderer dispatch.
- Define the Properties View, mode exclusivity, the raw fallback renderer, and primary-vs-inline rendering.
- Establish `display` as an ordinary reserved property key requiring no new mutation API.
- Amend the two shell specs so their "bare shell" prohibitions defer to this contract while keeping their other anti-default guards.

**Non-Goals:**

- No React/Tauri/`apps/desktop` implementation in this change.
- No concrete renderer set (Document Editor, Chart, Calendar are illustrative only).
- No persistence of current selection or active view mode.
- No global selection/navigation mechanism, no multi-window behavior.
- No change to `block-model`, `mutations`, or `session` (this change only *uses* their existing surfaces).

## Decisions

**D1 — `block-view` is a new capability, not edits to existing UI specs.**
The principles are a cohesive contract that the anti-default escape clause requires to live *somewhere normative*. A dedicated capability is the cleanest home and lets `desktop-shell`/`frontend-scaffold` simply defer to it. *Alternative considered*: fold the principles into `desktop-shell`. Rejected — it conflates the platform shell (Tauri host, IPC, save policy) with the cross-surface rendering model, and would also need to apply to a future web `apps/*`.

**D2 — `display` is a reserved key in the block properties map, value `PropertyValue::String`.**
`block-model` already gives every block a string-keyed property map with scalar `PropertyValue`s. Reusing it means no model change and no new mutation path: the Properties View reads/writes `display` through existing `core::Session` property mutations, preserving CRDT/versioning semantics for free. *Alternative considered*: a structured display config (renderer + options). Rejected for this change — `PropertyValue` is scalar-only, so it would force a `block-model` change; renderer-specific options are deferred to a future change.

**D3 — Initial current selected block resolves to the root block.**
Principle 1 ("there is always a current selected block") makes an initial value mandatory, so leaving it unspecified would violate the user's own principle. `block-model` guarantees exactly one root block, making it the only structurally-determined, non-invented choice — consistent with anti-default (we are not picking a "nice default," we are picking the single guaranteed entry point). *Alternative considered*: last-selected-block persistence. Deferred — selection persistence is a non-goal here.

**D4 — Raw fallback renderer for unset/unrecognized `display`.**
The user opted into a specified default rather than a stub. The raw renderer presents the block's stored properties as-is (no domain interpretation), satisfying "always render the current block" without inventing a rich default. An *unrecognized* `display` value MUST surface the value rather than silently substituting a renderer (mirrors the anti-default error-presentation rule), then fall back to raw. *Alternative considered*: hard error / empty stub on unset. Rejected per the user's explicit choice.

**D5 — Selection policy is delegated to the active Block View.**
Per the user's answer, no global "activate child = navigate" rule. Each Block View implementation defines how (or whether) interaction changes the current selected block. This keeps the contract free of invented navigation while still allowing rich per-renderer behavior later.

**D6 — Mode (Block View ↔ Properties View) is ephemeral, mutually exclusive UI state.**
Principle 4 mandates one-at-a-time. Persistence is a non-goal, so the active mode is runtime-only unless a future spec says otherwise.

**D7 — Block View / Properties View / dispatch live in the app adapter, not `packages/ui`.**
`frontend-scaffold`'s design-system boundary (no current-block/selection assumptions in `packages/ui`) is retained unchanged. Renderer dispatch and `display` reads/writes coordinate `core::Session` only — no duplicated domain logic — satisfying the thin-adapter requirement.

## Risks / Trade-offs

- **Reserved key collision** → `display` becomes a reserved property name a user could otherwise set as data. *Mitigation*: document it as reserved in the `block-view` spec; future model work may namespace reserved keys. Acceptable for v0.
- **Contract without implementation can drift** → the spec may not match what a later implementation finds practical. *Mitigation*: the implementation change must re-validate against these requirements and amend via OpenSpec, not silently diverge (anti-default escalation).
- **Amending two shell specs widens blast radius** → relaxing "bare shell" could be read as licensing other product UX. *Mitigation*: amendments explicitly retain the no-auto-save / no-shortcut / no-theme / no-welcome guards and only defer the *main surface* question to `block-view`.
- **Raw renderer scope creep** → "raw" could grow into a de-facto default editor. *Mitigation*: spec scopes it to presenting stored properties as-is, no editing affordances beyond what a future change specifies.

## Open Questions

- Concrete renderer registry and how a Block View advertises which `display` values it supports — deferred to the implementation change.
- Whether current selection and active view mode should persist across sessions — deferred.
- Inline rendering depth/recursion limits when a Block View displays hierarchy — deferred to per-renderer specs.
