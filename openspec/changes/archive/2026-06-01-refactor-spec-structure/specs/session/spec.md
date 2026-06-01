## ADDED Requirements

### Requirement: Adapter-defined open-time persistence

The session capability defines generic load, save, and dirty semantics. Adapters (for example **`desktop-shell`**) MAY define additional open-time persistence policies that cause an earlier save than the generic "first save creates root" flow. Such policies MUST live in adapter capabilities and MUST NOT alter **`session`** requirements for CLI or batch use.

#### Scenario: Desktop interim open policy does not change session contract

- **WHEN** the desktop adapter auto-saves on first open per **`desktop-shell`**, Requirement: **Desktop open policy (interim)**
- **THEN** **`session`** still defines that a new session opened without an adapter policy has no version records until first save
- **AND** **`cli`** behavior is unchanged
