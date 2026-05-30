## ADDED Requirements

### Requirement: File path argument

The CLI MUST accept a `--file` argument specifying the knowledge base storage path.

#### Scenario: Explicit file path

- **WHEN** the user runs any subcommand with `--file /path/to/kb.json`
- **THEN** the session loads from or saves to that path

### Requirement: Query subcommand

The CLI MUST provide a `query` subcommand that accepts a query expression and prints results.

#### Scenario: Query parent

- **WHEN** the user runs `query parent:<uuid>`
- **THEN** the CLI prints the parent block (or nothing if the block is the root) and exits with success

#### Scenario: Query children

- **WHEN** the user runs `query children:<uuid>`
- **THEN** the CLI prints all direct child blocks and exits with success

### Requirement: Create subcommand

The CLI MUST provide a `create` subcommand with a required `--parent` flag.

#### Scenario: Create and print id

- **WHEN** the user runs `create --parent <uuid>`
- **THEN** the CLI creates a block under the parent, saves if needed, prints the new block's UUID, and exits with success

#### Scenario: Create without parent fails

- **WHEN** the user runs `create` without `--parent`
- **THEN** the CLI prints an error to stderr and exits non-zero

### Requirement: Move subcommand

The CLI MUST provide a `move` subcommand accepting a block id and a required `--parent` flag.

#### Scenario: Move block via CLI

- **WHEN** the user runs `move <uuid> --parent <parent-uuid>`
- **THEN** the CLI reparents the block, saves, and exits with success

### Requirement: Delete subcommand

The CLI MUST provide a `delete` subcommand accepting a block id.

#### Scenario: Delete block via CLI

- **WHEN** the user runs `delete <uuid>` for a leaf block that is not the root
- **THEN** the CLI removes the block, saves, and exits with success

### Requirement: Error reporting

When a command fails, the CLI MUST print an error message to stderr and exit with a non-zero status code.

#### Scenario: Invalid command

- **WHEN** a subcommand fails due to validation or not-found error
- **THEN** the CLI prints the error to stderr and exits non-zero
