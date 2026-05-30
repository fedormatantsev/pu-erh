## Why

pu-erh has a working CLI and core session layer but no desktop host. A Tauri shell is the next vertical slice: a long-running process that owns `core::Session`, plus a Bun/TS/React frontend scaffold split into reusable design-system components and app-specific code — without inventing product UX before specs define it.

## What Changes

- Add a `desktop` Tauri application crate that initializes and holds a `core::Session` for the process lifetime
- Wire minimal Tauri commands (or equivalent IPC) so the frontend can invoke session operations through the thin Rust adapter — no duplicated domain logic
- Scaffold a Bun monorepo workspace with TypeScript and React
- Introduce `packages/ui` for design-system components (tokens, primitives, shared layout building blocks)
- Introduce `apps/desktop` for the Tauri frontend app (Vite + React entry, Tauri API bindings, app-specific composition)
- Add build/dev scripts to run the desktop app (`tauri dev` / `tauri build`) from the repo root
- Document the layout and anti-default constraints for future UI changes

**Non-goals for this change:**

- Product UX: navigation trees, auto-save, file open dialogs, keyboard shortcuts, themes, welcome flows, block editors
- Full session API surface over IPC (only scaffold-level commands, e.g. health/ping or root id if needed to prove wiring)
- DataFusion or analytical query UI
- Mobile or web deployment targets
- Publishing design-system packages to npm

## Capabilities

### New Capabilities

- `desktop-shell`: Tauri Rust wrapper crate, workspace membership, session initialization in-process, minimal IPC bridge to `core::Session`, dev/build entry points
- `frontend-scaffold`: Bun workspace root, `packages/ui` design-system package, `apps/desktop` React app consumed by Tauri, TypeScript project references, infrastructure-only UI (bare shell)

### Modified Capabilities

(none — core session, CLI, and storage behavior unchanged)

## Impact

- New Rust crate: `crates/desktop` (Tauri host, depends on `pu-erh-core`)
- New JS/TS workspace: root `package.json` with Bun workspaces (`packages/*`, `apps/*`)
- New packages: `packages/ui`, `apps/desktop` (with `src-tauri` symlink or co-location per design)
- New dependencies: Tauri 2.x, Vite, React, TypeScript (frontend); `tauri` crate (Rust)
- Cargo workspace extended with `crates/desktop`
- CI may need optional desktop build job (deferred unless tasks require it)
- Follows `agent-anti-default`: infrastructure defaults only; no invented product behavior
