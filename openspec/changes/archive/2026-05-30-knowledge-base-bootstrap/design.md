## Context

`AppState::open_at` is the single entry point for the desktop shell to open a knowledge base. It delegates to `Session::open`, which already handles a missing file by returning an empty session with the dirty flag set. However `open_at` does not call `save()`, so `root_id()` fails until something else triggers a save. There is no "first launch" flow in the desktop shell today.

Current stack:
- `AppState::open_at(path)` → `Session::open(path)` → empty KB, dirty
- Frontend calls `root_id()` → error (no root yet)

## Goals / Non-Goals

**Goals:**
- `AppState::open_at` returns a session where `root_id()` is always usable.
- New knowledge bases are persisted (root block written to disk) before the app accepts frontend calls.
- No change to `Session`, `Storage`, `graph`, or CLI crates.

**Non-Goals:**
- Migration or repair of corrupt/invalid KB files — that remains an error path.
- Auto-bootstrap in the CLI (the `init` subcommand covers that use case explicitly).
- Any UX beyond having a root block available.

## Decisions

### Check file existence before opening, then conditionally save

**Decision:** In `AppState::open_at`, check whether the KB file exists before calling `Session::open`. If it does not exist, call `session.save()` immediately after opening.

**Rationale:**
- `Session` already sets the dirty flag when opened for a missing file; `save()` is the documented path to materialize the root block.
- An existence check is cleaner than exposing `Session::is_dirty()` or a new `Session::is_new()` API — those would leak internal state through the layer boundary.
- Calling `save()` unconditionally on every open would be safe (non-dirty sessions no-op), but is unnecessarily wasteful on every app start for an existing KB.

**Alternative considered:** Add a `Session::bootstrap()` that does open + conditional save in one step. Rejected — adds API surface to core for a concern that belongs in the shell layer.

## Risks / Trade-offs

- [TOCTOU on file existence] Path could be created between the `!path.exists()` check and `Session::open`. Mitigation: acceptable — the race window is tiny and the worst outcome is a double-open on the same pre-existing file, not data loss.
- [Test inversion] The existing test `open_at_missing_file_creates_empty_session` asserts the old (broken) behavior and must be replaced.
