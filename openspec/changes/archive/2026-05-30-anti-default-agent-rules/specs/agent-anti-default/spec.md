## ADDED Requirements

### Requirement: Distinguish infrastructure defaults from product defaults

Agent-authored changes MUST treat **infrastructure defaults** (toolkit-required window creation, serde formats, test harness setup) as separate from **product defaults** (navigation layout, save policy, selection model, themes, shortcuts, onboarding copy). Product defaults MUST NOT be added unless an OpenSpec requirement or an explicit task line defines them.

#### Scenario: Toolkit requires a window size

- **WHEN** an agent adds a Desktop shell using a UI toolkit that needs initial window dimensions to compile
- **THEN** the agent MAY set the minimum dimensions the toolkit requires
- **AND** the agent MUST NOT add sidebars, trees, toolbars, or workflows not specified in the change

#### Scenario: Save policy unspecified

- **WHEN** a change implements a long-running Desktop host but does not specify when `Session::save` runs
- **THEN** the agent MUST NOT implement auto-save, save-on-mutation, or save-on-window-close
- **AND** the agent MUST either leave save unimplemented/disabled or implement only what a task explicitly describes (e.g. a Save button calling `save()`)

### Requirement: No invented interaction when requirements are silent

When a change task or spec does not define user interaction for a feature, agents MUST NOT invent “reasonable” or “good default” UX. Agents MUST use one of: disabled UI, explicit not-specified placeholder, or `unimplemented!()` / equivalent stub that fails loudly if invoked.

#### Scenario: Navigation metaphor unspecified

- **WHEN** a task says “show blocks from a query” but does not specify tree vs list vs outline
- **THEN** the agent MUST NOT add a hierarchical tree view with expand/collapse as the default
- **AND** the agent MAY render a flat list of query results or leave the view stubbed

#### Scenario: Create flow without parent selection UX

- **WHEN** a task wires create block but does not specify how the user chooses a parent
- **THEN** the agent MUST NOT assume “selected node in sidebar” as parent
- **AND** the agent MUST require an explicit parent identifier (e.g. text field) or leave create disabled

#### Scenario: Welcome or empty state unspecified

- **WHEN** no file is open and no empty-state requirement exists
- **THEN** the agent MUST NOT add marketing welcome text, tutorials, or sample knowledge bases
- **AND** the agent MAY show a bare window or a neutral “no file open” label without calls to action

### Requirement: Explicit mutation and query arguments

UI and Desktop adapters MUST mirror CLI explicitness: mutations require explicit identifiers (parent id, block id); read paths use the existing query language or typed `Session`/`KnowledgeBase` APIs — not inferred graph operations.

#### Scenario: Reparent via drag-and-drop without spec

- **WHEN** drag-and-drop reparent is not specified in the change
- **THEN** the agent MUST NOT implement drag-and-drop reparenting
- **AND** move operations MUST remain explicit (e.g. form fields calling `move_block`)

#### Scenario: Query parity

- **WHEN** a Desktop view runs a navigation query
- **THEN** it MUST use the same query expressions as the CLI (`parent:`, `children:`) or direct session APIs documented in the task
- **AND** it MUST NOT add implicit “current block” query context unless a spec defines that session state

### Requirement: Thin surface adapters

Agents implementing CLI, Desktop, or future App crates MUST keep domain logic in `core`, `graph`, and `storage`. UI crates MUST coordinate `Session` only and MUST NOT duplicate CRDT, trie, or mutation validation logic.

#### Scenario: Validation in UI crate

- **WHEN** an agent implements delete in Desktop
- **THEN** it MUST call `Session::delete_block` (or future session API)
- **AND** it MUST NOT reimplement root/leaf checks in the UI layer

### Requirement: Error presentation without friendly rewriting

When surfacing `CoreError` or related errors in UI, agents MUST show the error as returned (structured message / Debug-style detail acceptable). Agents MUST NOT replace errors with generic friendly copy unless a spec defines that copy.

#### Scenario: Mutation failure in Desktop

- **WHEN** `create_block` returns an error
- **THEN** the UI MUST display the error content without inventing a different user-facing message
- **AND** the UI MUST NOT silently retry or guess an alternate parent

### Requirement: Spec gap escalation

When implementation reaches unspecified product behavior, agents MUST prefer documenting the gap (comment referencing missing spec, or OpenSpec task addition) over shipping behavior. Agents MUST NOT close the gap by choosing industry-standard UX.

#### Scenario: Close window with unsaved dirty session

- **WHEN** dirty-session close behavior is not specified
- **THEN** the agent MUST NOT assume a native “Save changes?” dialog
- **AND** the agent MUST leave close handling stubbed or document the required new requirement in the change

### Requirement: Keyboard shortcuts and chrome

Agents MUST NOT add global keyboard shortcuts, menu bars, tray icons, or theme toggles unless an OpenSpec requirement or explicit task names them.

#### Scenario: Agent adds Cmd+S

- **WHEN** a Desktop task does not mention keyboard shortcuts
- **THEN** the agent MUST NOT bind Cmd+S / Ctrl+S to save
- **AND** save MUST only exist if a task defines a Save control or save policy

### Requirement: Project guidance discoverability

The repository MUST include `AGENTS.md` at the root that summarizes this policy and points to `openspec/specs/agent-anti-default/spec.md`. OpenSpec project context MUST mention that UI and Desktop work follows the anti-default policy.

#### Scenario: New contributor runs an agent on Desktop

- **WHEN** an agent reads repository guidance before implementing UI
- **THEN** it finds `AGENTS.md` with the anti-default checklist
- **AND** OpenSpec-generated artifacts include anti-default context from `openspec/config.yaml`

### Requirement: Tool wiring for Cursor and Claude Code

The repository MUST wire `AGENTS.md` so Cursor and Claude Code load the policy without manual paste. Cursor MUST use a project rule under `.cursor/rules/` with `alwaysApply: true` that instructs agents to follow `AGENTS.md`. Claude Code MUST use root `CLAUDE.md` that instructs agents to follow `AGENTS.md`.

#### Scenario: Cursor session on UI work

- **WHEN** an agent works in Cursor on any file in this repository
- **THEN** the anti-default Cursor rule is active
- **AND** the rule points to `AGENTS.md` as the canonical policy document

#### Scenario: Claude Code session

- **WHEN** an agent starts Claude Code in this repository
- **THEN** `CLAUDE.md` directs the agent to read and follow `AGENTS.md`
- **AND** the agent does not rely on duplicated conflicting policy in `CLAUDE.md`
