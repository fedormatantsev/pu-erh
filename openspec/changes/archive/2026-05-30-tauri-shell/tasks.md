## 1. Rust desktop adapter crate

- [x] 1.1 Add `crates/desktop` to the root Cargo workspace with dependency on `pu-erh-core`
- [x] 1.2 Implement `AppState` (or equivalent) holding `Mutex<Session>` opened at `{app_data_dir}/pu-erh/kb.json`
- [x] 1.3 Implement `ping` Tauri command handler returning a fixed string
- [x] 1.4 Implement `root_id` Tauri command handler calling `Session::root_id()` and mapping `CoreError` to invoke error without rewriting
- [x] 1.5 Add unit test for session open path helper (temp dir) without launching Tauri

## 2. Tauri application shell

- [x] 2.1 Scaffold `apps/desktop/src-tauri` with Tauri 2 binary depending on `crates/desktop`
- [x] 2.2 Register managed state and invoke handlers in Tauri `setup` / `generate_handler!`
- [x] 2.3 Configure `tauri.conf.json` (window title, minimum size if required by toolkit, dev/build hooks)
- [x] 2.4 Verify `cargo build` succeeds for the Tauri crate from `apps/desktop/src-tauri`

## 3. Bun monorepo and design system

- [x] 3.1 Add root `package.json` with Bun workspaces (`packages/*`, `apps/*`) and `dev:desktop` script
- [x] 3.2 Scaffold `packages/ui` as `@pu-erh/ui` with TypeScript, public `src/index.ts`, and minimal presentational components (`Text`, `Stack`, `Button`)
- [x] 3.3 Ensure `packages/ui` has no Tauri or session imports; export components only

## 4. Desktop React app

- [x] 4.1 Scaffold `apps/desktop` with Vite + React + TypeScript; depend on `@pu-erh/ui`
- [x] 4.2 Implement bare `App` shell: neutral `pu-erh` label using design-system components
- [x] 4.3 On mount, invoke `ping` and display result or raw error string in the UI
- [x] 4.4 Optionally invoke `root_id` and display UUID or raw error (no friendly rewrite)
- [x] 4.5 Wire Tauri `beforeDevCommand` / `beforeBuildCommand` to Vite via Bun

## 5. Documentation and verification

- [x] 5.1 Add README section: prerequisites (Rust, Bun, Tauri CLI), `bun install`, `bun run dev:desktop`
- [x] 5.2 Document layout: `crates/desktop` (Rust adapter), `packages/ui` (design system), `apps/desktop` (app + Tauri)
- [x] 5.3 Verify `cargo test` still passes for existing crates
- [x] 5.4 Manual smoke test: launch desktop app, confirm window shows label and ping response; confirm no auto-save on close
