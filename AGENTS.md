# Agent guide — pu-erh

Canonical policy for coding agents (Cursor, Claude Code, and others). Normative spec: [`openspec/specs/agent-anti-default/spec.md`](openspec/specs/agent-anti-default/spec.md).

## Anti-default principle

When a task or OpenSpec requirement is **unclear**, do **not** invent “reasonable” or “good default” product behavior. Stop, stub, disable, or ask — especially for UI and Desktop.

## Infrastructure vs product defaults

| Kind | Examples | Rule |
|------|----------|------|
| **Infrastructure** | Toolkit minimum window size, serde, test fixtures | Allowed when required to compile or run |
| **Product** | Auto-save, tree sidebar, welcome text, themes, Cmd+S, “current block”, drag-drop reparent | **Forbidden** unless a spec or explicit task line defines it |

## Checklist (UI / Desktop)

- [ ] Save timing: only what the spec/task says (no auto-save, no save-on-close unless specified)
- [ ] Navigation: no tree/outline/inspector unless specified
- [ ] Mutations: explicit UUIDs (parent, block) — mirror CLI; no “selected node” inference
- [ ] Queries: `parent:` / `children:` strings or documented `Session` APIs — no implicit cursor
- [ ] Empty / welcome states: bare or neutral label only unless specified
- [ ] Errors: show `CoreError` (etc.) as returned — no friendly rewrite
- [ ] Shortcuts / menus / tray / theme: none unless specified
- [ ] Close with dirty session: stub or spec gap — no assumed “Save changes?” dialog

## When requirements are silent

Use **one** of:

1. **Disabled** control or action
2. **Stub** — `unimplemented!()` or visible “not specified”
3. **Escalate** — comment or OpenSpec task for the missing requirement

Do **not** fill the gap with industry-standard UX.

## Thin adapters

- **CLI / Desktop / App** crates call `core::Session` only.
- Do **not** duplicate graph, CRDT, trie, or mutation validation in UI crates.
- Domain rules live in `graph`, `storage`, `core`.

## Architecture reminder

```
CLI / Desktop / App  →  core::Session  →  graph + storage
```

## OpenSpec

- UI and Desktop changes MUST respect `agent-anti-default`.
- Before implementing unspecified interaction, read the active change’s specs and tasks — do not extend scope.
