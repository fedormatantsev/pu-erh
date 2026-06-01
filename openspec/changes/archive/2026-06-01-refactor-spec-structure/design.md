## Context

pu-erh has 22 flat OpenSpec capabilities (~2,565 lines) accumulated from walking-skeleton through document-view work. Core specs (`block-model` through `storage`) are well-layered. UI specs grew into `block-view` (279 lines, 14 requirements) mixing navigation framework, routers, tree renderer, and properties panel. Property semantics for `title`, `display`, and `body` appear in three specs with conflicting absent-`title` rules.

The product core is stable enough to reorganize specs by mental model before adding more Block View renderers. This change is **spec-only** — no application code.

## Goals / Non-Goals

**Goals:**

- Restructure capabilities by layer: core → adapters → desktop app → design system.
- Split `block-view` into `desktop-shell-ui`, `tree-view`, and `properties-view`.
- Introduce `property-registry` as canonical domain semantics for reserved keys.
- Unify renderer dispatch: one registry keyed by `display` for primary and inline paths.
- Consolidate cross-cutting UI rules in `agent-anti-default` with normative cross-references elsewhere.
- Add informative `openspec/CAPABILITIES.md` reading order.
- Fix hygiene: TBD Purpose sections, malformed `ui-direction` header, duplicate action-bar requirement.

**Non-Goals:**

- Application code refactors (routers, components) — separate future changes.
- Future storage-engine autosave — only document extension points in `desktop-shell` and `agent-anti-default`.
- Rewriting archived change folders.
- CLI REPL mode or autosave behavior.

## Decisions

### 1. Capability split over monolith edit

**Choice:** Retire `block-view`, `well-known-properties`, `knowledge-base-bootstrap`; create four new capabilities.

**Alternatives:** Edit `block-view` in place and add sections — rejected because filename would still mislead agents and archive history would stay confusing.

### 2. Unified renderer registry keyed by `display`

**Choice:** Single registry in `desktop-shell-ui`. Both primary and inline dispatch resolve `block.display` → registered component. Registry entries declare supported modes (`primary`, `inline`, or both).

**Alternatives:** Separate Block View router and Inline View router with independent defaults — rejected; user chose unified lookup.

### 3. Default inline renderer in `desktop-shell-ui`

**Choice:** Default inline renderer (title → else block id) lives in `desktop-shell-ui`, not `tree-view`.

**Rationale:** Any host view showing inline blocks (tree, future outline/kanban) shares the same fallback without re-specifying labels.

### 4. Document renderer is primary-only

**Choice:** `document-view` registers `display=document` as primary-only. Inline dispatch for such blocks falls back to default inline renderer.

**Rationale:** Avoids half-baked document preview in tree columns; keeps unified registry honest via mode metadata rather than a second namespace.

### 5. `knowledge-base-bootstrap` stays adapter-specific

**Choice:** Move requirement to `desktop-shell` as **Desktop open policy (interim)**. Do not merge into `session`.

**Rationale:** Session stays storage-engine-agnostic (`root_id()` fails until first save). Desktop auto-save-on-open is interim until future autosave storage engine. CLI batch mode does not need it.

### 6. Normative cross-reference convention

**Choice:** References use `**capability-name**`, Requirement: **Exact heading** (optional Scenario).

**Alternatives:** Separate architecture doc — rejected; user wants normative refs agents can follow.

### 7. `CAPABILITIES.md` is informative only

**Choice:** Reading order and layer diagram live in `openspec/CAPABILITIES.md`, not as a normative spec.

## Risks / Trade-offs

- **[Risk] Large spec delta review burden** → Phased tasks (foundation → split → hygiene); requirement migration map in tasks.md.
- **[Risk] Agents reference retired capability names** → Update `AGENTS.md`, `openspec/config.yaml`, and CAPABILITIES.md deprecation table.
- **[Risk] MODIFIED deltas lose detail at archive** → Full requirement blocks copied for any MODIFIED entry; new capabilities get complete ADDED content.
- **[Risk] Primary-only pattern doesn't scale** → Registry mode set is extensible; future renderers declare modes explicitly.

## Migration Plan

**Phase 1 — Foundation**

1. Create new capability spec files under `openspec/specs/` via archive merge (or apply copies from change deltas).
2. Add consolidated requirements to `agent-anti-default`.
3. Move bootstrap to `desktop-shell`; add session adapter note.
4. Fix `ui-direction` header.

**Phase 2 — Retire old capabilities**

1. Verify all requirements from `block-view`, `well-known-properties`, `knowledge-base-bootstrap` appear in new/modified specs.
2. Delete `openspec/specs/block-view/`, `well-known-properties/`, `knowledge-base-bootstrap/`.

**Phase 3 — Hygiene**

1. Add `openspec/CAPABILITIES.md`.
2. Update `AGENTS.md` and `openspec/config.yaml` capability names.
3. Delete stale `openspec/changes/structured-document-view/`.

**Rollback:** Git revert; no runtime impact (specs only).

## Open Questions

None — decisions locked in exploration:

- Default inline renderer → `desktop-shell-ui`
- Unified registry by `display`
- Document → primary-only
- Bootstrap → `desktop-shell` interim policy
