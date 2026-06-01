## MODIFIED Requirements

### Requirement: Action bar overlay surface

The application MUST present an **action bar**: a compact floating panel pinned to the upper-left corner that overlays the active content surface (Block View or Properties View). The action bar MUST be subordinate chrome per **`ui-direction`**. It MUST NOT displace or resize content beneath it. The action bar MUST render the shell-level navigation actions (backward and forward per **`navigation-history`**) followed by the actions provided by the active view's descriptor factory per **`desktop-shell-ui`**, Requirement: **View Router** — it MUST NOT introduce navigation trees, sidebars, menus, or other unspecified surfaces.

#### Scenario: Action bar overlays the content in the upper-left

- **WHEN** the application renders its content surface
- **THEN** the action bar is shown as a floating panel in the upper-left corner over the content
- **AND** the content surface fills the available width beneath it without being displaced

#### Scenario: Action bar renders navigation actions then view actions

- **WHEN** the action bar is rendered
- **THEN** it shows backward and forward navigation actions followed by the actions from the active view's descriptor factory
- **AND** it adds no navigation tree, sidebar, menu, or other surface

## ADDED Requirements

### Requirement: Backward and forward navigation actions

The action bar MUST always render a backward action and a forward action as the first two entries in the action list, regardless of the active view mode. The backward action MUST be disabled when the back stack is empty per **`navigation-history`**, Requirement: **Navigate back**. The forward action MUST be disabled when the forward stack is empty per **`navigation-history`**, Requirement: **Navigate forward**. Activating the backward action MUST invoke `navigateBack`. Activating the forward action MUST invoke `navigateForward`.

#### Scenario: Backward action disabled with no history

- **WHEN** the back stack is empty
- **THEN** the backward action is rendered in the action bar with `isDisabled` set to true

#### Scenario: Backward action enabled with history

- **WHEN** the back stack is non-empty
- **THEN** the backward action is rendered in the action bar with `isDisabled` set to false

#### Scenario: Forward action disabled with no forward history

- **WHEN** the forward stack is empty
- **THEN** the forward action is rendered in the action bar with `isDisabled` set to true

#### Scenario: Forward action enabled with forward history

- **WHEN** the forward stack is non-empty
- **THEN** the forward action is rendered in the action bar with `isDisabled` set to false

#### Scenario: Activating backward navigates back

- **WHEN** the user activates the backward action
- **THEN** `navigateBack` is invoked per **`navigation-history`**, Requirement: **Navigate back**

#### Scenario: Activating forward navigates forward

- **WHEN** the user activates the forward action
- **THEN** `navigateForward` is invoked per **`navigation-history`**, Requirement: **Navigate forward**
