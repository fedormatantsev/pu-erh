## Why

When the app starts for the first time (or after a fresh install), the knowledge base file does not exist yet, so `root_id()` fails and the app has no usable state. The app needs to detect this and auto-save to materialize the root block so every downstream operation (query, tree view, mutations) has a valid starting point without requiring an explicit `init` step from the user.

## What Changes

- `AppState::open_at` bootstraps the knowledge base automatically when the storage file is absent: after opening the session it calls `save()` so the root block is immediately present.
- The test `open_at_missing_file_creates_empty_session` is inverted: after `open_at`, `root_id()` MUST succeed.
- The CLI `init` subcommand can remain as-is (explicit init for the file-path workflow), but the desktop path no longer needs it.

## Capabilities

### New Capabilities

- `knowledge-base-bootstrap`: When `AppState` is created for a path that has no existing knowledge base file, the session is automatically saved so the root block is present and `root_id()` is immediately usable.

### Modified Capabilities

- `session`: The requirement "New session has empty knowledge base before first save / `root_id()` fails until the root block version record is inserted" remains correct at the `Session` layer; the new behavior lives one layer up in `AppState`, not in `Session` itself.

## Impact

- `crates/desktop/src/state.rs`: `AppState::open_at` gains a conditional `session.save()` call.
- Existing `AppState` tests need updating: `open_at_missing_file_creates_empty_session` becomes `open_at_missing_file_bootstraps_root`.
- No changes to `crates/core`, `crates/storage`, or `crates/cli`.
