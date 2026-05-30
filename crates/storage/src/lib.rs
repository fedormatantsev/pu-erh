use std::fs;
use std::path::Path;

use graph::{BlockVersion, EdgeVersion, KnowledgeBase};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const FORMAT_VERSION: u32 = 2;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("failed to read storage file: {0}")]
    Read(String),
    #[error("failed to write storage file: {0}")]
    Write(String),
    #[error("invalid storage file: {0}")]
    Invalid(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeBaseFile {
    pub format_version: u32,
    pub block_versions: Vec<BlockVersion>,
    pub edge_versions: Vec<EdgeVersion>,
}

pub fn load(path: &Path) -> Result<KnowledgeBase, StorageError> {
    if !path.exists() {
        return Ok(KnowledgeBase::empty());
    }

    let contents = fs::read_to_string(path).map_err(|err| StorageError::Read(err.to_string()))?;
    let file: KnowledgeBaseFile =
        serde_json::from_str(&contents).map_err(|err| StorageError::Invalid(err.to_string()))?;

    if file.format_version != FORMAT_VERSION {
        return Err(StorageError::Invalid(format!(
            "unsupported format_version {}",
            file.format_version
        )));
    }

    for record in &file.block_versions {
        record
            .verify_digest()
            .map_err(|err| StorageError::Invalid(format!("block version digest mismatch: {err}")))?;
    }
    for record in &file.edge_versions {
        record
            .verify_digest()
            .map_err(|err| StorageError::Invalid(format!("edge version digest mismatch: {err}")))?;
    }

    Ok(KnowledgeBase::from_records(
        file.block_versions,
        file.edge_versions,
    ))
}

pub fn save(path: &Path, kb: &KnowledgeBase) -> Result<(), StorageError> {
    let file = KnowledgeBaseFile {
        format_version: FORMAT_VERSION,
        block_versions: kb.block_version_records(),
        edge_versions: kb.edge_version_records(),
    };

    let contents =
        serde_json::to_string_pretty(&file).map_err(|err| StorageError::Write(err.to_string()))?;

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|err| StorageError::Write(err.to_string()))?;
        }
    }

    fs::write(path, contents).map_err(|err| StorageError::Write(err.to_string()))
}

pub fn merge_knowledge_bases_from_paths(
    left: &Path,
    right: &Path,
) -> Result<KnowledgeBase, StorageError> {
    let left_kb = load(left)?;
    let right_kb = load(right)?;
    Ok(left_kb.merge(&right_kb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{EdgeType, Properties};
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn missing_file_returns_empty_knowledge_base() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.json");
        let kb = load(&path).unwrap();
        assert!(kb.is_empty());
    }

    #[test]
    fn round_trip_preserves_versions_and_reads() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let child = Uuid::new_v4();
        kb.append_block_version(child, false, Properties::new());
        kb.append_edge_version(
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );

        save(&path, &kb).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.block_version_records().len(), 2);
        assert_eq!(loaded.edge_version_records().len(), 1);
        assert_eq!(loaded.root_id().unwrap(), root);
        assert_eq!(loaded.block_count(), 2);
        assert_eq!(loaded.children(root).unwrap().len(), 1);
    }

    #[test]
    fn merge_knowledge_bases_unions_by_key() {
        let dir = tempdir().unwrap();
        let left_path = dir.path().join("left.json");
        let right_path = dir.path().join("right.json");

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
    fn invalid_json_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.json");
        fs::write(&path, "{ not json").unwrap();
        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn legacy_snapshot_format_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("legacy.json");
        fs::write(&path, r#"{"blocks":[],"edges":[]}"#).unwrap();
        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn format_version_one_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("v1.json");
        fs::write(
            &path,
            r#"{"format_version":1,"block_versions":[],"edge_versions":[]}"#,
        )
        .unwrap();
        let err = load(&path).unwrap_err();
        match err {
            StorageError::Invalid(msg) => assert!(msg.contains("unsupported format_version 1")),
            other => panic!("expected invalid storage error, got {other:?}"),
        }
    }

    #[test]
    fn null_property_round_trips() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut props = Properties::new();
        props.insert("note".into(), graph::PropertyValue::Null);

        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(id, false, props);
        save(&path, &kb).unwrap();

        let loaded = load(&path).unwrap();
        let block = loaded.get_block(id).unwrap();
        assert_eq!(block.properties.get("note"), Some(&graph::PropertyValue::Null));
    }

    #[test]
    fn array_property_value_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");
        let id = Uuid::new_v4();
        fs::write(
            &path,
            format!(
                r#"{{
  "format_version": 2,
  "block_versions": [{{
    "id": "{id}",
    "version": 1,
    "digest": "0000000000000000000000000000000000000000000000000000000000000000",
    "tombstoned": false,
    "properties": {{ "tags": [] }}
  }}],
  "edge_versions": []
}}"#
            ),
        )
        .unwrap();

        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn object_property_value_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");
        let id = Uuid::new_v4();
        fs::write(
            &path,
            format!(
                r#"{{
  "format_version": 2,
  "block_versions": [{{
    "id": "{id}",
    "version": 1,
    "digest": "0000000000000000000000000000000000000000000000000000000000000000",
    "tombstoned": false,
    "properties": {{ "meta": {{}} }}
  }}],
  "edge_versions": []
}}"#
            ),
        )
        .unwrap();

        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }

    #[test]
    fn tampered_digest_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let root = Uuid::new_v4();
        let mut record = BlockVersion::new(root, 1, None, false, Properties::new()).unwrap();
        record.digest = [0u8; 32];

        let file = KnowledgeBaseFile {
            format_version: FORMAT_VERSION,
            block_versions: vec![record],
            edge_versions: vec![],
        };
        let contents = serde_json::to_string_pretty(&file).unwrap();
        fs::write(&path, contents).unwrap();

        let err = load(&path).unwrap_err();
        match err {
            StorageError::Invalid(msg) => assert!(msg.contains("digest mismatch")),
            other => panic!("expected invalid storage error, got {other:?}"),
        }
    }
}
