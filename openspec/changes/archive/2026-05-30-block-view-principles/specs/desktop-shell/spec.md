## MODIFIED Requirements

### Requirement: Anti-default UI shell

The desktop window's main content surface MUST be the Block View defined by the `block-view` capability, rendering the current selected block. The desktop window MUST NOT include navigation trees, sidebars, auto-save, keyboard shortcut menus, themes, or welcome/marketing copy as part of this change. Any surface beyond the Block View and its associated Properties View (per `block-view`) MUST be introduced only by an explicit future requirement.

#### Scenario: Main surface is the Block View

- **WHEN** the application renders its main content surface
- **THEN** it shows the Block View rendering the current selected block as defined by `block-view`
- **AND** it does not include hierarchical block navigation trees, sidebars, or file-open dialogs

#### Scenario: No invented chrome

- **WHEN** the application renders
- **THEN** it does not add auto-save, keyboard shortcut menus, themes, or welcome/marketing copy
