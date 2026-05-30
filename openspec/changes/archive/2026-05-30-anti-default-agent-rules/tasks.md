## 1. OpenSpec capability

- [x] 1.1 Verify change delta at `openspec/changes/anti-default-agent-rules/specs/agent-anti-default/spec.md` matches proposal capability name
- [x] 1.2 After archive, confirm main spec exists at `openspec/specs/agent-anti-default/spec.md` with Purpose + Requirements

## 2. Repository agent guidance

- [x] 2.1 Add root `AGENTS.md` with anti-default checklist, infrastructure vs product default table, and link to `openspec/specs/agent-anti-default/spec.md`
- [x] 2.2 Document required responses when specs are silent (disable / stub / escalate — not guessed UX)
- [x] 2.3 Document thin-adapter rule: UI calls `Session` only; no graph logic in UI crates
- [x] 2.4 Wire Cursor: `.cursor/rules/agent-anti-default.mdc` (`alwaysApply: true` → `AGENTS.md`)
- [x] 2.5 Wire Claude Code: root `CLAUDE.md` points to `AGENTS.md` as canonical policy

## 3. OpenSpec project context

- [x] 3.1 Update `openspec/config.yaml` `context` with an **Agent policy** paragraph referencing anti-default rules for UI and Desktop work
- [x] 3.2 Keep paragraph concise (under ~15 lines) so proposal generation stays readable

## 4. Verification

- [x] 4.1 Read `AGENTS.md` and spec side-by-side; ensure no contradictory guidance
- [x] 4.2 Grep repo for placeholder paths; confirm `AGENTS.md` is linked or discoverable from README (one line in README optional if not already present)
