## Context

pu-erh uses OpenSpec for product requirements and a thin-adapter architecture (CLI → `core::Session` → graph/storage). Desktop work is planned but not implemented. Coding agents implementing UI routinely import template opinions (auto-save, tree sidebars, Cmd+S, welcome text) when tasks or specs are silent. The CLI already embodies explicitness (`--parent` required, literal query strings, minimal stdout). This change codifies that stance for agents without shipping a Desktop binary.

## Goals / Non-Goals

**Goals:**

- Single normative OpenSpec capability (`agent-anti-default`) agents can be pointed at during apply
- Repository-root `AGENTS.md` readable by humans and tools (Cursor, Claude, etc.)
- OpenSpec `config.yaml` context so new changes inherit the policy in artifact generation
- Clear split between acceptable infrastructure choices and forbidden product guesses
- Actionable responses when behavior is unspecified (disable, stub, spec gap — not “reasonable UX”)

**Non-Goals:**

- Desktop crate, windowing, or UI framework selection
- Changing runtime behavior of `Session`, CLI, graph, or storage
- Enforcing policy via automated linters or CI (documentation and spec only in this change)
- Defining specific Desktop screens, layouts, or save policies (left for a future Desktop change)

## Decisions

### 1. Normative spec + `AGENTS.md` mirror

**Choice:** OpenSpec `agent-anti-default` holds testable requirements; `AGENTS.md` restates the same rules in imperative prose for agents, with a pointer to the spec path.

**Rationale:** OpenSpec is the contract for archive/apply; `AGENTS.md` is the discovery path most agents read first.

**Alternatives considered:**

- `AGENTS.md` only — not archived into main specs on change completion
- Cursor rule only — tool-specific, weaker for OpenSpec apply workflow

### 2. Two classes of “defaults”

**Choice:** Policy explicitly allows minimal **infrastructure** defaults (e.g. a window opens with toolkit-required size) while forbidding **product** defaults (navigation, save timing, empty-state copy, shortcuts, selection model).

**Rationale:** Eliminates debate where agents claim “every app needs auto-save.”

### 3. Unspecified UI → stop, don’t guess

**Choice:** When a task or spec does not define interaction behavior, implementation MUST use one of: disabled control, explicit `todo!()` / `unimplemented!()` in Rust UI code, or a visible “not specified” placeholder — never inferred workflows.

**Rationale:** Forces gaps to surface in OpenSpec instead of ossifying agent inventions.

**Alternatives considered:**

- “Simplest reasonable UX” — rejected; contradicts project direction
- Block apply entirely without spec — too heavy; stub/disabled is enough for walking slices

### 4. Long-running app lifecycle is never implicit

**Choice:** Save-on-close, auto-save, multi-file, and “current block” cursor are **product** decisions. Agents MUST NOT add them unless a change spec defines them.

**Rationale:** `Session` already has explicit `dirty` + `save()`; Desktop must not layer policy without requirements.

### 5. Surface adapters stay thin

**Choice:** Future Desktop (and any App) MUST call `Session` / `core` APIs for mutations and queries; agents MUST NOT duplicate graph invariants or trie logic in UI crates.

**Rationale:** Matches existing CLI pattern and walking-skeleton architecture.

### 6. OpenSpec context injection

**Choice:** Append a short **Agent policy** paragraph to `openspec/config.yaml` `context` referencing anti-default rules and Desktop caution.

**Rationale:** `openspec instructions` includes context when generating proposals — reduces regression on new changes.

## Risks / Trade-offs

- **[Risk] Policy feels verbose for small tasks** → Mitigation: `AGENTS.md` uses a short checklist; full detail lives in spec
- **[Risk] Agents still ignore docs** → Mitigation: Future Desktop change tasks reference `agent-anti-default`; user can add Cursor rules later
- **[Risk] Over-stubbing blocks progress** → Mitigation: Spec allows minimal explicit UI when requirements ARE written (e.g. “Save button calls `session.save()` only”)
- **[Trade-off] No CI enforcement** → Accept for this change; compliance is social/tooling until needed

## Migration Plan

1. Land `AGENTS.md` and updated `openspec/config.yaml`
2. Archive this change so `openspec/specs/agent-anti-default/spec.md` becomes main spec
3. Future `desktop-*` proposals list `agent-anti-default` as dependency in Impact / tasks

No rollback beyond reverting docs; no data migration.

## Open Questions

- Whether to add a `.cursor/rules/pu-erh-ui.mdc` in a follow-up (optional; not required for apply-ready)
- Whether Desktop first slice should reference this change by name in every UI task (recommend yes when Desktop change is proposed)
