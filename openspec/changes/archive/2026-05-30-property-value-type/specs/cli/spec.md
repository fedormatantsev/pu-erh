## MODIFIED Requirements

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
