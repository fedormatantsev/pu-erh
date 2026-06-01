use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use graph::{
    block_version_key_from, edge_version_key_from, BlockVersion, EdgeVersion, KnowledgeBase,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const FORMAT_VERSION: u32 = 3;

const MANIFEST_FILE: &str = "format_version.toml";
const BLOCKS_DIR: &str = "blocks";
const EDGES_DIR: &str = "edges";

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("failed to read storage: {0}")]
    Read(String),
    #[error("failed to write storage: {0}")]
    Write(String),
    #[error("invalid storage: {0}")]
    Invalid(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct FormatManifest {
    format_version: u32,
}

pub fn load(path: &Path) -> Result<KnowledgeBase, StorageError> {
    if !path.exists() {
        return Ok(KnowledgeBase::empty());
    }

    let format_version = read_format_version(path)?;
    if format_version != FORMAT_VERSION {
        return Err(StorageError::Invalid(format!(
            "unsupported format_version {format_version}"
        )));
    }

    let mut block_versions = Vec::new();
    for file_path in list_record_files(&path.join(BLOCKS_DIR))? {
        let contents =
            fs::read_to_string(&file_path).map_err(|err| StorageError::Read(err.to_string()))?;
        let record: BlockVersion = toml::from_str(&contents)
            .map_err(|err| StorageError::Invalid(err.to_string()))?;
        record.verify_digest().map_err(|err| {
            StorageError::Invalid(format!("block version digest mismatch: {err}"))
        })?;
        block_versions.push(record);
    }

    let mut edge_versions = Vec::new();
    for file_path in list_record_files(&path.join(EDGES_DIR))? {
        let contents =
            fs::read_to_string(&file_path).map_err(|err| StorageError::Read(err.to_string()))?;
        let record: EdgeVersion =
            toml::from_str(&contents).map_err(|err| StorageError::Invalid(err.to_string()))?;
        record.verify_digest().map_err(|err| {
            StorageError::Invalid(format!("edge version digest mismatch: {err}"))
        })?;
        edge_versions.push(record);
    }

    Ok(KnowledgeBase::from_records(block_versions, edge_versions))
}

pub fn save(path: &Path, kb: &KnowledgeBase) -> Result<(), StorageError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|err| StorageError::Write(err.to_string()))?;
        }
    }
    fs::create_dir_all(path).map_err(|err| StorageError::Write(err.to_string()))?;

    let blocks_dir = path.join(BLOCKS_DIR);
    let edges_dir = path.join(EDGES_DIR);
    fs::create_dir_all(&blocks_dir).map_err(|err| StorageError::Write(err.to_string()))?;
    fs::create_dir_all(&edges_dir).map_err(|err| StorageError::Write(err.to_string()))?;

    write_format_version(path)?;

    let block_records = kb.block_version_records();
    let edge_records = kb.edge_version_records();

    let mut block_names = HashSet::with_capacity(block_records.len());
    for record in &block_records {
        let name = write_block_record(&blocks_dir, record)?;
        block_names.insert(name);
    }
    prune_stale_files(&blocks_dir, &block_names)?;

    let mut edge_names = HashSet::with_capacity(edge_records.len());
    for record in &edge_records {
        let name = write_edge_record(&edges_dir, record)?;
        edge_names.insert(name);
    }
    prune_stale_files(&edges_dir, &edge_names)?;

    Ok(())
}

pub fn merge_knowledge_bases_from_paths(
    left: &Path,
    right: &Path,
) -> Result<KnowledgeBase, StorageError> {
    let left_kb = load(left)?;
    let right_kb = load(right)?;
    Ok(left_kb.merge(&right_kb))
}

fn read_format_version(root: &Path) -> Result<u32, StorageError> {
    let manifest_path = root.join(MANIFEST_FILE);
    let contents = fs::read_to_string(&manifest_path)
        .map_err(|err| StorageError::Read(err.to_string()))?;
    let manifest: FormatManifest =
        toml::from_str(&contents).map_err(|err| StorageError::Invalid(err.to_string()))?;
    Ok(manifest.format_version)
}

fn write_format_version(root: &Path) -> Result<(), StorageError> {
    let manifest = FormatManifest {
        format_version: FORMAT_VERSION,
    };
    let contents =
        toml::to_string_pretty(&manifest).map_err(|err| StorageError::Write(err.to_string()))?;
    fs::write(root.join(MANIFEST_FILE), contents)
        .map_err(|err| StorageError::Write(err.to_string()))
}

fn trie_key_filename(key: &[u8]) -> String {
    key.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn write_block_record(blocks_dir: &Path, record: &BlockVersion) -> Result<String, StorageError> {
    let filename = format!("{}.toml", trie_key_filename(&block_version_key_from(record)));
    let contents =
        toml::to_string_pretty(record).map_err(|err| StorageError::Write(err.to_string()))?;
    fs::write(blocks_dir.join(&filename), contents)
        .map_err(|err| StorageError::Write(err.to_string()))?;
    Ok(filename)
}

fn write_edge_record(edges_dir: &Path, record: &EdgeVersion) -> Result<String, StorageError> {
    let filename = format!("{}.toml", trie_key_filename(&edge_version_key_from(record)));
    let contents =
        toml::to_string_pretty(record).map_err(|err| StorageError::Write(err.to_string()))?;
    fs::write(edges_dir.join(&filename), contents)
        .map_err(|err| StorageError::Write(err.to_string()))?;
    Ok(filename)
}

fn list_record_files(dir: &Path) -> Result<Vec<PathBuf>, StorageError> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).map_err(|err| StorageError::Read(err.to_string()))? {
        let entry = entry.map_err(|err| StorageError::Read(err.to_string()))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn prune_stale_files(dir: &Path, keep: &HashSet<String>) -> Result<(), StorageError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|err| StorageError::Read(err.to_string()))? {
        let entry = entry.map_err(|err| StorageError::Read(err.to_string()))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !keep.contains(name) {
                fs::remove_file(&path).map_err(|err| StorageError::Write(err.to_string()))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{EdgeType, Properties};
    use tempfile::tempdir;
    use uuid::Uuid;

    fn kb_dir(parent: &Path) -> PathBuf {
        parent.join("kb")
    }

    fn write_manifest(dir: &Path, version: u32) {
        let manifest = FormatManifest {
            format_version: version,
        };
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join(MANIFEST_FILE),
            toml::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn missing_directory_returns_empty_knowledge_base() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());
        let kb = load(&path).unwrap();
        assert!(kb.is_empty());
    }

    #[test]
    fn round_trip_preserves_versions_and_reads() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());

        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let child = Uuid::new_v4();
        kb.append_block_version(child, false, Properties::new());
        kb.append_edge_version(child, root, EdgeType::Parent, false, Properties::new());

        save(&path, &kb).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.block_version_records().len(), 2);
        assert_eq!(loaded.edge_version_records().len(), 1);
        assert_eq!(loaded.root_id().unwrap(), root);
        assert_eq!(loaded.block_count(), 2);
        assert_eq!(loaded.children(root).unwrap().len(), 1);
        assert!(path.join(BLOCKS_DIR).read_dir().unwrap().count() >= 2);
        assert_eq!(path.join(EDGES_DIR).read_dir().unwrap().count(), 1);
    }

    #[test]
    fn merge_knowledge_bases_unions_by_key() {
        let dir = tempdir().unwrap();
        let left_path = kb_dir(dir.path()).join("left");
        let right_path = kb_dir(dir.path()).join("right");

        let mut left = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        left.append_root_block(root);
        save(&left_path, &left).unwrap();

        let mut right = KnowledgeBase::empty();
        right.append_block_version(Uuid::new_v4(), false, Properties::new());
        save(&right_path, &right).unwrap();

        let merged = merge_knowledge_bases_from_paths(&left_path, &right_path).unwrap();
        assert_eq!(merged.block_version_records().len(), 2);
    }

    #[test]
    fn corrupt_toml_is_rejected() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());
        write_manifest(&path, FORMAT_VERSION);
        let blocks = path.join(BLOCKS_DIR);
        fs::create_dir_all(&blocks).unwrap();
        fs::create_dir_all(path.join(EDGES_DIR)).unwrap();
        fs::write(blocks.join("bad.toml"), "not = [valid").unwrap();
        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn null_property_round_trips() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());

        let mut props = Properties::new();
        props.insert("note".into(), graph::PropertyValue::Null);

        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(id, false, props);
        save(&path, &kb).unwrap();

        let loaded = load(&path).unwrap();
        let block = loaded.get_block(id).unwrap();
        assert_eq!(
            block.properties.get("note"),
            Some(&graph::PropertyValue::Null)
        );
    }

    #[test]
    fn array_property_value_is_rejected() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());
        let id = Uuid::new_v4();
        write_manifest(&path, FORMAT_VERSION);
        let blocks = path.join(BLOCKS_DIR);
        fs::create_dir_all(&blocks).unwrap();
        fs::create_dir_all(path.join(EDGES_DIR)).unwrap();
        fs::write(
            blocks.join("bad.toml"),
            format!(
                r#"
id = "{id}"
version = 1
digest = "0000000000000000000000000000000000000000000000000000000000000000"
tombstoned = false

[properties]
tags = []
"#
            ),
        )
        .unwrap();

        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn object_property_value_is_rejected() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());
        let id = Uuid::new_v4();
        write_manifest(&path, FORMAT_VERSION);
        let blocks = path.join(BLOCKS_DIR);
        fs::create_dir_all(&blocks).unwrap();
        fs::create_dir_all(path.join(EDGES_DIR)).unwrap();
        fs::write(
            blocks.join("bad.toml"),
            format!(
                r#"
id = "{id}"
version = 1
digest = "0000000000000000000000000000000000000000000000000000000000000000"
tombstoned = false

[properties.meta]
"#
            ),
        )
        .unwrap();

        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn tampered_digest_is_rejected() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());

        let root = Uuid::new_v4();
        let mut record = BlockVersion::new(root, 1, None, false, Properties::new()).unwrap();
        record.digest = [0u8; 32];

        write_manifest(&path, FORMAT_VERSION);
        let blocks = path.join(BLOCKS_DIR);
        fs::create_dir_all(&blocks).unwrap();
        fs::create_dir_all(path.join(EDGES_DIR)).unwrap();
        let contents = toml::to_string_pretty(&record).unwrap();
        let filename = format!("{}.toml", trie_key_filename(&block_version_key_from(&record)));
        fs::write(blocks.join(filename), contents).unwrap();

        let err = load(&path).unwrap_err();
        match err {
            StorageError::Invalid(msg) => assert!(msg.contains("digest mismatch")),
            other => panic!("expected invalid storage error, got {other:?}"),
        }
    }

    #[test]
    fn save_removes_stale_record_files() {
        let dir = tempdir().unwrap();
        let path = kb_dir(dir.path());

        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(id, false, Properties::new());
        save(&path, &kb).unwrap();
        let stale = path.join(BLOCKS_DIR).join("deadbeef.toml");
        fs::write(&stale, "stale = true").unwrap();

        save(&path, &kb).unwrap();
        assert!(!stale.exists());
    }
}
