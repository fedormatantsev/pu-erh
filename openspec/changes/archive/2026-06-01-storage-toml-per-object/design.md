## Context

Today `crates/storage` writes one pretty-printed JSON file (`format_version: 2`, `block_versions[]`, `edge_versions[]`). `Session::open` and `save` take a `Path` that callers treat as that file (`kb.json` in CLI tests and `{app_data}/pu-erh/kb.json` on desktop). Version records already serialize via serde on `BlockVersion` and `EdgeVersion`; digests and `PropertyValue` encoding are unchanged from format 2.

## Goals / Non-Goals

**Goals:**

- Persist each version record as its own TOML file under `blocks/` or `edges/`.
- Keep trie round-trip, digest verification on load, and missing-path → empty knowledge base semantics.
- Bump on-disk format to version 3 with an explicit root manifest.
- Update default desktop path and CLI/docs examples to a directory.

**Non-Goals:**

- Changing CRDT, trie keys, or digest algorithms.
- File watcher, incremental save, or concurrent multi-writer access.
- User-facing “open folder” UX (path remains infrastructure).

## Decisions

### 1. Storage path is a directory

`load(path)` and `save(path)` use `path` as the **root of a knowledge base**, not a single file. Missing root directory → empty `KnowledgeBase`.

**Alternative:** Keep `--file` as a file path and add `--dir` — rejected; one path type keeps `Session` API unchanged.

### 2. On-disk layout (format version 3)

```
<storage-dir>/
  format_version.toml    # format_version = 3
  blocks/
    <key-hex>.toml       # one BlockVersion per file
  edges/
    <key-hex>.toml       # one EdgeVersion per file
```

`format_version.toml` contains only `format_version` (integer). Subdirectories are created on save.

**Alternative:** Embed `format_version` in every record file — rejected; redundant and easy to drift.

### 3. Per-file naming via trie key hex

Filename = lowercase hex of the record’s trie key (`block_version_key_from` / `edge_version_key_from` from `graph::trie_key`). Keys are unique per version record; filenames are stable across save/load.

**Alternative:** `{uuid}-v{n}.toml` — readable but edge filenames need encoded `(target, type, source)` and must stay collision-free.

### 4. TOML body = serde struct only

Each file serializes one `BlockVersion` or `EdgeVersion` (same fields as today). `digest` and `previous_digest` remain hex strings via existing `serde_hex` adapters. `EdgeVersion` keeps `type` as the TOML key for `edge_type` (`serde(rename = "type")`).

Dependency: replace `serde_json` with `toml` in `crates/storage`.

### 5. Save replaces the directory contents

On save:

1. Ensure `blocks/` and `edges/` exist.
2. Write `format_version.toml` with version 3.
3. For each record from trie export, write `blocks/<key-hex>.toml` or `edges/<key-hex>.toml`.
4. Delete any `*.toml` in those subdirs whose names are not in the current export set (handles tombstones / superseded versions removed from trie export).

**Alternative:** Append-only files without delete — rejected; stale files would reload as duplicate trie keys.

### 6. Load walks subdirectories

1. If root missing → empty KB.
2. Read `format_version.toml`; reject if missing, malformed, or not `3`.
3. Read every `*.toml` in `blocks/` and `edges/` (ignore non-`.toml` entries).
4. Deserialize, verify digest per record, insert into tries (same as today).
5. Extra files in root (other than manifest) are ignored; unknown subdirs are ignored.

### 7. Caller path updates

| Caller | Old | New |
|--------|-----|-----|
| Desktop `AppState::kb_path` | `.../pu-erh/kb.json` | `.../pu-erh/kb/` |
| CLI/tests/examples | `kb.json` | `kb/` (directory in tempdir) |
| `merge_knowledge_bases_from_paths` | two file paths | two directory paths |

`--file` flag name unchanged (means storage location).

## Risks / Trade-offs

- **[Many small files]** → Slower save on large graphs; acceptable for current scale; mitigated by full rewrite only on explicit save.
- **[Partial writes on crash]** → Directory may be inconsistent mid-save; mitigated by writing records then deleting orphans last, or write-to-temp-then-rename in a follow-up if needed (initial implementation: simple write + delete).
- **[TOML property limits]** → Nested objects/arrays in properties remain rejected at deserialize.

## Migration Plan

1. Ship format 3 implementation.
2. Desktop first launch after upgrade sees missing `kb/` directory → bootstrap creates new empty store (per **`desktop-shell`** interim open policy).
3. Document directory layout in README.

## Open Questions

_None for implementation start._ Filename hex encoding and root manifest are decided above.
