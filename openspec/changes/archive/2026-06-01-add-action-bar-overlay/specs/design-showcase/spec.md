## MODIFIED Requirements

### Requirement: Showcase displays all base components

The system SHALL render live `@pu-erh/ui` React component examples covering: Button (primary, secondary, disabled), Badge (neutral, primary), Card with sample content, Divider, Text (heading levels and body), Stack, the TreeView column component(s) shown with static sample data (a parent, a current block with siblings, and children) and a distinguished current block, and the ActionBar shown with static sample state (a selected view mode and an enabled create-child action). The ActionBar example SHALL be session-agnostic: its toggle and create-child handlers operate on local example state and SHALL NOT call IPC/Tauri APIs.

#### Scenario: Components section is present

- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Components" or equivalent contains a live rendered example of each exported component

#### Scenario: TreeView column example is present

- **WHEN** the showcase page is loaded
- **THEN** the Components section contains a live rendered example of the TreeView column component(s) using static sample data
- **AND** the example renders the three columns (parent, current block with siblings, children) with the current block distinguished

#### Scenario: ActionBar example is present

- **WHEN** the showcase page is loaded
- **THEN** the Components section contains a live rendered example of the `ActionBar` component using static sample state
- **AND** the example renders the view-mode toggle and the create-child action without calling IPC/Tauri APIs
