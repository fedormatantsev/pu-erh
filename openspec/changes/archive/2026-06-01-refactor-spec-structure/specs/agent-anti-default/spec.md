## ADDED Requirements

### Requirement: UI adapters coordinate Session only

UI and Desktop view and renderer code MUST read and mutate graph state exclusively through `core::Session` (or adapter IPC commands that delegate to Session without adding domain logic). UI MUST NOT duplicate CRDT, trie, mutation validation, or child-ordering logic defined in core capabilities.

#### Scenario: View coordinates session only

- **WHEN** a view or renderer resolves block state or applies a mutation
- **THEN** it uses `core::Session` or thin IPC wrappers over Session
- **AND** it does not reimplement domain logic in the UI layer

### Requirement: In-memory mutation with adapter-defined persistence

Unless a capability explicitly defines different persistence timing, UI mutations MUST apply in memory immediately through Session APIs. Persisting to disk MUST occur only through mechanisms explicitly defined by an adapter capability. Currently, **`desktop-shell`** defines explicit Save only; a future storage-engine capability MAY define autosave and supersede interim desktop policies.

#### Scenario: Property edit held in memory until Save

- **WHEN** a user edits a block property in the Desktop UI and no autosave capability applies
- **THEN** the change is applied in memory through Session immediately
- **AND** no save to disk occurs until the user invokes explicit Save per **`desktop-shell`**

#### Scenario: Agent does not add autosave without spec

- **WHEN** a change implements Desktop UI but no adapter or storage capability defines autosave
- **THEN** the agent MUST NOT implement save-on-mutation, save-on-blur, or save-on-close
- **AND** the agent MUST follow **`desktop-shell`** or an explicit task line for save timing

### Requirement: Normative cross-reference convention

When one capability depends on another, requirements MUST reference the dependency using: the capability name in bold, Requirement: **Exact requirement heading**, and optionally Scenario: **Exact scenario heading**. Requirements MUST NOT duplicate normative text that already exists in another capability unless this capability adds surface-specific behavior.

#### Scenario: View references save policy

- **WHEN** a view spec defines edit behavior
- **THEN** persistence timing references **`desktop-shell`** or **`agent-anti-default`**, Requirement: **In-memory mutation with adapter-defined persistence**
- **AND** does not restate the full save policy verbatim
