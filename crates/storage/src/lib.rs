use std::fs;
use std::path::Path;

use graph::{merge_histories, BlockVersion, EdgeVersion, VersionHistory};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("failed to read storage file: {0}")]
    Read(String),
    #[error("failed to write storage file: {0}")]
    Write(String),
    #[error("invalid storage file: {0}")]
    Invalid(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeBaseFile {
    pub format_version: u32,
    pub block_versions: Vec<BlockVersion>,
    pub edge_versions: Vec<EdgeVersion>,
}

pub fn load(path: &Path) -> Result<VersionHistory, StorageError> {
    if !path.exists() {
        return Ok(VersionHistory::default());
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

    Ok(VersionHistory {
        block_versions: file.block_versions,
        edge_versions: file.edge_versions,
    })
}

pub fn save(path: &Path, history: &VersionHistory) -> Result<(), StorageError> {
    let file = KnowledgeBaseFile {
        format_version: FORMAT_VERSION,
        block_versions: history.block_versions.clone(),
        edge_versions: history.edge_versions.clone(),
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

pub fn merge_histories_from_paths(
    left: &Path,
    right: &Path,
) -> Result<VersionHistory, StorageError> {
    let left_history = load(left)?;
    let right_history = load(right)?;
    Ok(merge_histories(&left_history, &right_history))
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{
        append_block_version, append_edge_version, create_root_block_version, Properties,
        Snapshot, PARENT_EDGE_TYPE,
    };
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn missing_file_returns_empty_history() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.json");
        let history = load(&path).unwrap();
        assert!(history.block_versions.is_empty());
        assert!(history.edge_versions.is_empty());
    }

    #[test]
    fn round_trip_preserves_versions_and_materializes() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut history = VersionHistory::default();
        let root = Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let child = Uuid::new_v4();
        append_block_version(&mut history, child, false, Properties::new());
        append_edge_version(
            &mut history,
            child,
            root,
            PARENT_EDGE_TYPE,
            false,
            Properties::new(),
        );

        save(&path, &history).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.block_versions.len(), 2);
        assert_eq!(loaded.edge_versions.len(), 1);

        let snapshot = Snapshot::materialize(&loaded);
        assert_eq!(snapshot.root_id().unwrap(), root);
        assert_eq!(snapshot.block_count(), 2);
        assert_eq!(snapshot.children(root).unwrap().len(), 1);
    }

    #[test]
    fn merge_histories_unions_by_digest() {
        let dir = tempdir().unwrap();
        let left_path = dir.path().join("left.json");
        let right_path = dir.path().join("right.json");

        let mut left = VersionHistory::default();
        let root = Uuid::new_v4();
        left.append_block(create_root_block_version(root));
        save(&left_path, &left).unwrap();

        let mut right = VersionHistory::default();
        append_block_version(&mut right, Uuid::new_v4(), false, Properties::new());
        save(&right_path, &right).unwrap();

        let merged = merge_histories_from_paths(&left_path, &right_path).unwrap();
        assert_eq!(merged.block_versions.len(), 2);
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
}
