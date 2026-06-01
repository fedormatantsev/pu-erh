# pu-erh capabilities

Informative index for humans and agents. **Normative requirements live in `openspec/specs/<capability>/spec.md`.**

## Reading order

1. `block-model` → `version-history` → `radix-trie-map` → `immutable-snapshot`
2. `child-ordering` → `property-registry` → `mutations`
3. `session` → `storage` → `query-language`
4. `cli` | `desktop-shell` (pick your adapter)
5. `desktop-shell-ui` → `tree-view` | `document-view` | `properties-view`
6. `action-bar` → `design-tokens`, `ui-direction`, `base-components`
7. `agent-anti-default` — read before any UI or Desktop work

## Layer map

```
Meta:           agent-anti-default

Core:           block-model, version-history, radix-trie-map,
                immutable-snapshot, child-ordering, property-registry,
                mutations, query-language, session, storage

Adapters:       cli, desktop-shell

Desktop app:    desktop-shell-ui, tree-view, properties-view,
                document-view, action-bar

Design system:  frontend-scaffold, design-tokens, ui-direction,
                base-components, design-showcase
```

## Normative cross-reference convention

When one capability depends on another, reference:

`**capability-name**`, Requirement: **Exact requirement heading**

Optional: `Scenario: **Exact scenario heading**`

Do not duplicate normative text from another capability unless adding surface-specific behavior.

## Deprecated capabilities (2026-06-01 restructure)

| Retired | Replacement |
|---------|-------------|
| `block-view` | `desktop-shell-ui` + `tree-view` + `properties-view` |
| `well-known-properties` | `property-registry` (domain) + `properties-view` (UI slots) |
| `knowledge-base-bootstrap` | `desktop-shell`, Requirement: **Desktop open policy (interim)** |

Archived change: `openspec/changes/archive/2026-06-01-refactor-spec-structure/`
