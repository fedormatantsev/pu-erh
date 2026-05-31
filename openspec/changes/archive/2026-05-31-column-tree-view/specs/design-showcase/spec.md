## MODIFIED Requirements

### Requirement: Showcase displays all base components

The system SHALL render live `@pu-erh/ui` React component examples covering: Button (primary, secondary, disabled), Badge (neutral, primary), Card with sample content, Divider, Text (heading levels and body), Stack, and the TreeView column component(s) shown with static sample data (a parent, a current block with siblings, and children) and a distinguished current block.

#### Scenario: Components section is present

- **WHEN** the showcase page is loaded
- **THEN** a section labelled "Components" or equivalent contains a live rendered example of each exported component

#### Scenario: TreeView column example is present

- **WHEN** the showcase page is loaded
- **THEN** the Components section contains a live rendered example of the TreeView column component(s) using static sample data
- **AND** the example renders the three columns (parent, current block with siblings, children) with the current block distinguished
