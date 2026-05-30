## Context

pu-erh is a Rust monorepo (`cli`, `core`, `graph`, `storage`) with a working CLI that opens a short-lived `core::Session` per invocation. The target architecture includes a Desktop/App layer that owns a long-running session. This change introduces that host using Tauri 2 and a Bun/TS/React frontend monorepo, following `agent-anti-default`: infrastructure scaffolding only, no invented product UX.

Current state: no JavaScript tooling, no desktop crate, no IPC. Core session API is stable (`open`, `query`, mutations, `save`).

## Goals / Non-Goals

**Goals:**

- Establish the Desktop → Core adapter pattern mirroring CLI → Core
- Run Tauri with a React frontend in dev and release builds from the repo
- Split frontend into `packages/ui` (design system) and `apps/desktop` (app composition)
- Hold one `core::Session` in the Tauri process for the app lifetime
- Expose minimal Tauri commands to prove Rust ↔ frontend wiring
- Keep domain logic in `core`; Rust desktop code is a thin coordinator

**Non-Goals:**

- File open/save dialogs, auto-save, save-on-close, or any save policy beyond what tasks specify
- Navigation UI (trees, sidebars, inspectors), block editors, query builders
- Keyboard shortcuts, menus beyond Tauri defaults, system tray, themes
- Full session API over IPC (query/create/move/delete) — deferred to follow-up changes
- CI release artifacts for all platforms (optional stub only)
- npm registry publishing of `@pu-erh/ui`

## Decisions

### 1. Repository layout: Rust crate + Bun workspaces

**Choice:**

```
pu-erh/
├── Cargo.toml                 # workspace + crates/desktop member
├── package.json               # Bun workspaces root
├── crates/
│   └── desktop/               # lib: session host + Tauri command handlers
├── packages/
│   └── ui/                    # @pu-erh/ui — design-system components
└── apps/
    └── desktop/               # @pu-erh/desktop — React app + Tauri shell
        ├── src/               # Vite React entry
        ├── index.html
        └── src-tauri/         # Tauri binary; depends on crates/desktop
```

**Rationale:** Matches proposal split: design system in `/packages`, app in `/apps`. `crates/desktop` keeps session-host logic unit-testable without launching a window. Tauri convention places `src-tauri` under the app that owns the webview.

**Alternatives considered:**

- Single `apps/desktop` Rust crate only (no `crates/desktop`) — simpler but harder to test adapter logic
- `crates/desktop` as the Tauri binary root with frontend elsewhere — non-standard Tauri layout, worse DX
- Frontend inside `crates/desktop/ui/` — mixes Rust and JS boundaries; rejected

### 2. Tauri 2 + Vite + React + Bun

**Choice:** Tauri 2 for the native shell; Vite as the frontend bundler; React 19 (or current stable) for UI; Bun as package manager and script runner.

**Rationale:** User requested Bun + TS + React. Tauri 2 is the standard Rust desktop webview host and integrates cleanly with Vite. Bun workspaces match the monorepo layout.

**Alternatives considered:**

- npm/pnpm — user specified Bun
- egui/iced native UI — out of scope; frontend must be React design system

### 3. Session lifecycle: in-process, single instance, fixed data path

**Choice:** On Tauri startup, `crates/desktop` opens exactly one `Session` at `{app_data_dir}/pu-erh/kb.json` (create if missing). The session lives in Tauri managed state (`State<AppState>`) for the process lifetime. No file picker; path is infrastructure, not user-facing workflow.

**Rationale:** `Session::open` requires a path. A deterministic app-data path lets the shell compile and run without inventing open-file UX. Dirty/save behavior is not wired unless a task explicitly adds it (anti-default).

**Alternatives considered:**

- Lazy session (open on first command) — equivalent complexity; eager open proves init at startup
- In-memory-only session — would require core API changes; rejected
- CLI-style explicit `--file` flag on desktop binary — acceptable for dev but not required for v0 shell

### 4. IPC surface: scaffold commands only

**Choice:** Initial Tauri commands:

| Command | Purpose |
|---------|---------|
| `ping` | Returns a fixed string; proves invoke works |
| `root_id` | Calls `Session::root_id()`; proves core wiring (errors propagate as Tauri error strings) |

No mutation or query commands in this change.

**Rationale:** Proves thin adapter without shipping unspecified product flows. Errors return `CoreError` display/debug text unchanged (per agent-anti-default).

**Alternatives considered:**

- Mirror full CLI over IPC now — scope creep; defer
- No IPC, Rust-only window — doesn't validate React integration

### 5. Design system package (`packages/ui`)

**Choice:** Package name `@pu-erh/ui`. Exports a minimal set of presentational components (e.g. `Text`, `Stack`, `Button`) with co-located CSS modules or a single base stylesheet. No routing, no data fetching, no session awareness.

**Rationale:** Establishes the split the user requested. Components are building blocks only; app logic stays in `apps/desktop`.

**Alternatives considered:**

- shadcn/Radix full kit — product styling choices; defer
- No components, empty package — doesn't validate the split

### 6. App shell UI (`apps/desktop`)

**Choice:** Root `App` renders a bare layout using `@pu-erh/ui` primitives: app title label (`pu-erh`) and the result of `ping` (and optionally `root_id` when session has a root after save). No sidebars, no block lists, no welcome copy.

**Rationale:** Satisfies anti-default empty state: neutral label + wiring proof only.

### 7. Build and dev entry points

**Choice:**

- Root `package.json` scripts: `dev:desktop` → `bun --filter @pu-erh/desktop tauri dev`
- `apps/desktop` owns `tauri.conf.json` with `beforeDevCommand` / `beforeBuildCommand` running Vite via Bun
- Cargo workspace adds `crates/desktop`; `apps/desktop/src-tauri/Cargo.toml` depends on `desktop` path crate and `pu-erh-core`

**Rationale:** Single documented path for contributors; Rust and JS toolchains stay in their conventional locations.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Fixed app-data path surprises users expecting explicit file choice | Document as infrastructure; file-open UX is a follow-up change |
| Tauri + Bun CI complexity | Optional CI job; local `tauri dev` is the primary gate for this change |
| `@pu-erh/ui` premature abstraction | Keep package tiny; only components needed by the shell |
| Session open fails on permissions | Surface `CoreError` in dev console / optional error text in UI |
| Dual toolchain (Rust + Bun) onboarding | README section with prerequisites and one dev command |

## Migration Plan

Greenfield addition — no migration. Existing CLI and core workflows unchanged.

Deploy steps:

1. Merge change; contributors install Bun and Tauri prerequisites
2. Run `bun install` at repo root
3. Run `bun run dev:desktop` (or equivalent) to launch the shell

Rollback: remove `apps/desktop`, `packages/ui`, `crates/desktop`, and root `package.json`; revert Cargo workspace member.

## Open Questions

- **Platform targets for CI:** macOS-only initially vs matrix — recommend macOS-only stub in tasks unless user specifies
- **Root id before first save:** `root_id` command may error until first save; UI should display error as-is or show neutral placeholder — recommend displaying error string when invoke fails
- **Package scope name:** `@pu-erh/ui` vs `@pu-erh/design-system` — using `@pu-erh/ui` for brevity
