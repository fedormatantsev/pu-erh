## ADDED Requirements

### Requirement: Bootstrap root block on first open

When `AppState` is opened at a path where no knowledge base file exists, the system MUST automatically save the session so the root block is materialized before any frontend calls are served. After `open_at` returns successfully, `root_id()` MUST succeed.

#### Scenario: First launch creates root block

- **WHEN** `AppState::open_at` is called with a path that does not exist on disk
- **THEN** `open_at` saves the session automatically
- **AND** `root_id()` returns a valid UUID immediately after `open_at` returns

#### Scenario: Existing knowledge base is not re-saved on open

- **WHEN** `AppState::open_at` is called with a path to an existing knowledge base file
- **THEN** no additional save is performed during `open_at`
- **AND** the file on disk is not modified by the open operation alone
