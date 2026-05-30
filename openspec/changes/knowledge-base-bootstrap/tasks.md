## 1. Implementation

- [x] 1.1 In `AppState::open_at`, check `!path.exists()` before calling `Session::open`; if true, call `session.save()` after opening and propagate any error

## 2. Tests

- [x] 2.1 Replace test `open_at_missing_file_creates_empty_session` with `open_at_missing_file_bootstraps_root`: assert `root_id()` succeeds after `open_at` on a new path
- [x] 2.2 Add test `open_at_existing_file_does_not_overwrite`: save a KB, record its modified time, re-open it, assert modified time is unchanged
