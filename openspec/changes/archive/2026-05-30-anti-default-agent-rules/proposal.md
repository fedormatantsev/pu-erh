## Why

pu-erh is moving toward a Desktop surface and richer UI work, where coding agents routinely fill gaps with “sane defaults” (auto-save, tree navigation, welcome screens, implicit selection). Those choices hide unspecified product decisions and bloat the codebase with behavior that was never agreed. We need durable, testable conventions so agents stop when requirements are unclear instead of inventing UX.

## What Changes

- Introduce a first-class **agent anti-default policy** as an OpenSpec capability with explicit requirements
- Add project-level agent guidance (`AGENTS.md`) that mirrors the spec and applies to all surfaces (CLI, Desktop, future App)
- Extend OpenSpec project context so proposal/apply flows inherit the policy without re-litigating it each change
- Define operational responses when UI or lifecycle behavior is unspecified: stub, disable, or document a spec gap — never ship guessed product behavior
- Separate **infrastructure defaults** (window toolkit, serde, etc.) from **product defaults** (navigation, save timing, empty states, shortcuts)

**Non-goals for this change:**

- Implementing the Desktop app or any UI crate
- Choosing a UI framework (Tauri, egui, etc.)
- Changing `Session`, graph, storage, or CLI behavior
- Auto-save, tree views, themes, or keyboard shortcut suites

## Capabilities

### New Capabilities

- `agent-anti-default`: Requirements and scenarios governing how agents (and humans following the same rules) must behave when implementing UI, session lifecycle in long-running apps, and unspecified interaction design

### Modified Capabilities

(none — no existing product capability requirements change)

## Impact

- New spec: `openspec/specs/agent-anti-default/spec.md` (via change delta)
- New `AGENTS.md` at repository root
- `openspec/config.yaml` project context updated to reference the policy
- Future Desktop and UI-related changes depend on this capability; apply tasks for those changes should cite it
