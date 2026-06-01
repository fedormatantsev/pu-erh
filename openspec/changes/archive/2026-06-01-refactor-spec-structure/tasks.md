## 1. Archive preparation

- [x] 1.1 Run `openspec validate refactor-spec-structure` and fix any delta format errors
- [x] 1.2 Review change deltas for requirement coverage vs `openspec/specs/block-view`, `well-known-properties`, `knowledge-base-bootstrap`

## 2. Apply spec deltas (archive change)

- [x] 2.1 Archive `refactor-spec-structure` so merged specs land in `openspec/specs/`
- [x] 2.2 Verify new capabilities exist: `property-registry`, `desktop-shell-ui`, `tree-view`, `properties-view`
- [x] 2.3 Verify retired capability directories are removed: `block-view`, `well-known-properties`, `knowledge-base-bootstrap`

## 3. Spec hygiene (post-archive)

- [x] 3.1 Fix `openspec/specs/ui-direction/spec.md`: add `# ui-direction Specification`, `## Purpose`, and `## Requirements` wrapper (content currently starts with `## ADDED Requirements`)
- [x] 3.2 Write Purpose sections for specs that still have TBD after archive (`document-view`, others if any remain)
- [x] 3.3 Trim `openspec/specs/block-model/spec.md` Purpose to remove active-read wording duplicated by new **Active read semantics reference** requirement

## 4. Informative index and agent guidance

- [x] 4.1 Add `openspec/CAPABILITIES.md` with layer diagram, reading order, normative cross-ref convention summary, and deprecated capability table
- [x] 4.2 Update `openspec/config.yaml` context to list new desktop capabilities and point to `CAPABILITIES.md`
- [x] 4.3 Update `AGENTS.md` OpenSpec section: new capability names, reading order, cross-reference convention

## 5. Cleanup

- [x] 5.1 Delete stale `openspec/changes/structured-document-view/` if still present
- [x] 5.2 Grep repo for references to retired capabilities (`block-view`, `well-known-properties`, `knowledge-base-bootstrap`) in active docs/rules and update to new names

## 6. Verification

- [x] 6.1 Confirm `openspec list` shows no blocking issues for active changes
- [x] 6.2 Spot-check normative refs: `document-view` primary-only inline fallback, `desktop-shell` interim open policy, `property-registry` title inline vs editor scenarios
