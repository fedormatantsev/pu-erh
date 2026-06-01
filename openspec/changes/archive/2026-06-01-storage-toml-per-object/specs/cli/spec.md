## MODIFIED Requirements

### Requirement: File path argument

The CLI MUST accept a `--file` argument specifying the knowledge base storage path (a directory root for format version 3).

#### Scenario: Explicit file path

- **WHEN** the user runs any subcommand with `--file /path/to/kb`
- **THEN** the session loads from or saves to that storage directory

### Requirement: Init subcommand

The CLI MUST provide an `init` subcommand that persists a new knowledge base with a root block and prints the root block's UUID.

#### Scenario: Init new knowledge base

- **WHEN** the user runs `init` with `--file /path/to/kb` on a missing or empty knowledge base
- **THEN** the CLI saves a root block version record and prints the root UUID to stdout
- **AND** exits with success
