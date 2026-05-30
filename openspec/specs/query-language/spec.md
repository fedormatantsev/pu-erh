# query-language Specification

## Purpose

Defines the v0 read-only query language for navigating the block hierarchy via `parent:` and `children:` expressions with UUID arguments.
## Requirements
### Requirement: Parent query

The query language MUST support `parent:<uuid>` to return the parent block of the block with the given id.

#### Scenario: Block with parent

- **WHEN** query `parent:<uuid>` is executed and the block with `<uuid>` has a parent
- **THEN** the result contains exactly the parent block

#### Scenario: Root block has no parent

- **WHEN** query `parent:<uuid>` is executed and the block with `<uuid>` has no parent
- **THEN** the result is empty

#### Scenario: Unknown block

- **WHEN** query `parent:<uuid>` is executed and no block with `<uuid>` exists
- **THEN** the system returns an error indicating the block was not found

### Requirement: Children query

The query language MUST support `children:<uuid>` to return all direct child blocks of the block with the given id.

#### Scenario: Block with children

- **WHEN** query `children:<uuid>` is executed and the block has direct children
- **THEN** the result contains all blocks that are the source of a `parent` edge whose target is `<uuid>`

#### Scenario: Children result order unspecified

- **WHEN** query `children:<uuid>` returns multiple blocks
- **THEN** result order is unspecified

#### Scenario: Block with no children

- **WHEN** query `children:<uuid>` is executed and the block has no direct children
- **THEN** the result is empty

#### Scenario: Unknown block for children

- **WHEN** query `children:<uuid>` is executed and no block with `<uuid>` exists
- **THEN** the system returns an error indicating the block was not found

### Requirement: Invalid query syntax

The system MUST reject query expressions that do not match a supported form.

#### Scenario: Unrecognized prefix

- **WHEN** a query string does not start with `parent:` or `children:`
- **THEN** the system returns an error indicating invalid query syntax

#### Scenario: Invalid UUID in query

- **WHEN** a query has a valid prefix but the suffix is not a valid UUID
- **THEN** the system returns an error indicating invalid UUID

