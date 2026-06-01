# cli Specification

## Purpose

Defines the pu-erh command-line interface: storage path argument, init/query/create/move/delete subcommands, output formats, and error reporting.
## Requirements
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

### Requirement: Query subcommand

The CLI MUST provide a `query` subcommand that accepts a query expression and prints results.

#### Scenario: Query parent

- **WHEN** the user runs `query parent:<uuid>`
- **THEN** the CLI prints the parent block (or nothing if the block is the root) and exits with success

#### Scenario: Query children

- **WHEN** the user runs `query children:<uuid>`
- **THEN** the CLI prints all direct child blocks and exits with success

#### Scenario: Query output format

- **WHEN** the user runs a query that returns one or more blocks
- **THEN** each result line is printed as `<uuid> <json-properties>` (UUID, space, JSON object whose values are only JSON strings, numbers, booleans, or `null`)
- **AND** lines are separated by newlines

### Requirement: Create subcommand

The CLI MUST provide a `create` subcommand with a required `--parent` flag.

#### Scenario: Create and print id

- **WHEN** the user runs `create --parent <uuid>`
- **THEN** the CLI creates a block under the parent, saves, prints the new block's UUID alone on stdout, and exits with success

#### Scenario: Mutating subcommands always save

- **WHEN** the user runs `create`, `move`, or `delete` successfully
- **THEN** the CLI persists changes to the storage file before exiting

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

