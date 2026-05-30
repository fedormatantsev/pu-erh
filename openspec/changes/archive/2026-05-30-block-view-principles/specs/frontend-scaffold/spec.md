## MODIFIED Requirements

### Requirement: Neutral empty shell in app

The desktop app's main surface MUST follow the `block-view` capability: it renders the current selected block through the Block View. The app MUST NOT include tutorial text, sample data, or marketing welcome flows. Before user data is available through a specified workflow, the app MUST show only bare scaffold content or the Block View's specified empty/raw rendering, without calls to action beyond IPC wiring proof.

#### Scenario: Main surface follows block-view

- **WHEN** the app renders its main surface
- **THEN** it renders the current selected block through the Block View as defined by `block-view`
- **AND** it does not include tutorial text, sample data, or marketing welcome flows

#### Scenario: Initial render without user data

- **WHEN** no user data has been loaded through a specified workflow
- **THEN** the app shows bare scaffold content or the Block View's specified raw rendering without calls to action beyond IPC wiring proof
