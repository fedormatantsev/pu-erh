# navigation-history Specification

## Purpose

Defines the ephemeral back/forward navigation history stacks maintained by the application shell, the navigate-back and navigate-forward operations, and the guarantee that history is never persisted.
## Requirements
### Requirement: Navigation history stacks

The shell MUST maintain two ephemeral stacks — a back stack and a forward stack — that together record the sequence of current selected block changes driven by user navigation. Both stacks MUST be held in the application shell as in-memory state and MUST NOT be persisted to storage. Both stacks MUST be empty on application open.

#### Scenario: Stacks empty on open

- **WHEN** the application opens and resolves the root block as the current selected block
- **THEN** the back stack is empty
- **AND** the forward stack is empty

#### Scenario: New navigation pushes to back stack and clears forward stack

- **WHEN** the user navigates to a new block via `selectBlock`
- **THEN** the previous current block id is pushed onto the back stack
- **AND** the forward stack is cleared
- **AND** the new block becomes the current selected block

### Requirement: Navigate back

The shell MUST expose a `navigateBack` operation. When invoked, it MUST pop the most recent entry from the back stack, push the current block id onto the front of the forward stack, and set the popped entry as the current selected block. `navigateBack` MUST be a no-op when the back stack is empty.

#### Scenario: Navigate back with history available

- **WHEN** the user invokes `navigateBack` and the back stack is non-empty
- **THEN** the top of the back stack becomes the current selected block
- **AND** the previous current block id is pushed onto the forward stack
- **AND** the back stack is one entry shorter

#### Scenario: Navigate back with empty back stack is a no-op

- **WHEN** the user invokes `navigateBack` and the back stack is empty
- **THEN** the current selected block does not change
- **AND** no stack is modified

### Requirement: Navigate forward

The shell MUST expose a `navigateForward` operation. When invoked, it MUST pop the most recent entry from the forward stack, push the current block id onto the back stack, and set the popped entry as the current selected block. `navigateForward` MUST be a no-op when the forward stack is empty.

#### Scenario: Navigate forward with forward history available

- **WHEN** the user invokes `navigateForward` and the forward stack is non-empty
- **THEN** the top of the forward stack becomes the current selected block
- **AND** the previous current block id is pushed onto the back stack
- **AND** the forward stack is one entry shorter

#### Scenario: Navigate forward with empty forward stack is a no-op

- **WHEN** the user invokes `navigateForward` and the forward stack is empty
- **THEN** the current selected block does not change
- **AND** no stack is modified

### Requirement: History is ephemeral

The navigation history MUST reset to empty stacks when the application is opened or restarted. No part of the navigation history MUST be written to disk or read from storage.

#### Scenario: History absent after restart

- **WHEN** the application is closed and re-opened
- **THEN** both the back stack and the forward stack are empty
- **AND** the current selected block resolves to the root block per **`desktop-shell-ui`**, Requirement: **Current selected block**
