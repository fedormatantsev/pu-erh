## MODIFIED Requirements

### Requirement: Desktop open policy (interim)

Until a storage-engine capability with autosave is specified, when `AppState` is opened at a path where no knowledge base storage directory exists, the desktop adapter MUST automatically save the session so the root block is materialized before any frontend calls are served. After `open_at` returns successfully, `root_id()` MUST succeed. This policy is adapter-specific and MUST NOT be assumed by **`session`**, **`cli`**, or future REPL mode.

The deterministic storage path under application data MUST be the directory `{app_data_dir}/pu-erh/kb/` (not a `kb.json` file).

#### Scenario: First launch creates root block

- **WHEN** `AppState::open_at` is called with a storage directory path that does not exist on disk
- **THEN** `open_at` saves the session automatically
- **AND** `root_id()` succeeds immediately after `open_at` returns

#### Scenario: Existing knowledge base is not re-saved on open

- **WHEN** `AppState::open_at` is called with a path to an existing knowledge base storage directory
- **THEN** no additional save is performed during `open_at`
- **AND** the storage directory on disk is not modified by the open operation alone
