use std::fs;
use std::path::Path;

use graph::{Block, Edge, EdgeKey, Graph, GraphError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Graph(#[from] GraphError),
    #[error("failed to read storage file: {0}")]
    Read(String),
    #[error("failed to write storage file: {0}")]
    Write(String),
    #[error("invalid storage file: {0}")]
    Invalid(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeBaseFile {
    blocks: Vec<Block>,
    edges: Vec<Edge>,
}

pub fn load(path: &Path) -> Result<Graph, StorageError> {
    if !path.exists() {
        return Ok(Graph::new());
    }

    let contents = fs::read_to_string(path).map_err(|err| StorageError::Read(err.to_string()))?;
    let file: KnowledgeBaseFile =
        serde_json::from_str(&contents).map_err(|err| StorageError::Invalid(err.to_string()))?;

    let mut blocks = std::collections::HashMap::new();
    for block in file.blocks {
        blocks.insert(block.id, block);
    }

    let mut edges = std::collections::HashMap::new();
    for edge in file.edges {
        let key = EdgeKey::new(edge.target, &edge.edge_type, edge.source);
        if edges.contains_key(&key) {
            return Err(StorageError::Invalid("duplicate edge".into()));
        }
        edges.insert(key, edge);
    }

    Graph::from_parts(blocks, edges).map_err(StorageError::Graph)
}

pub fn save(path: &Path, graph: &Graph) -> Result<(), StorageError> {
    let file = KnowledgeBaseFile {
        blocks: graph.blocks().cloned().collect(),
        edges: graph.edges().cloned().collect(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{Properties, PARENT_EDGE_TYPE};
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn missing_file_returns_new_graph_with_root() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("missing.json");
        let graph = load(&path).unwrap();
        assert_eq!(graph.blocks().count(), 1);
        assert!(graph.is_root(graph.root_id()));
    }

    #[test]
    fn round_trip_preserves_blocks_and_edges() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = Uuid::new_v4();
        graph.insert_block(graph::Block {
            id: child,
            properties: Properties::new(),
        });
        graph
            .insert_edge(graph::Edge {
                source: child,
                target: root,
                edge_type: PARENT_EDGE_TYPE.into(),
                properties: Properties::new(),
            })
            .unwrap();

        save(&path, &graph).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.root_id(), root);
        assert_eq!(loaded.blocks().count(), 2);
        assert_eq!(loaded.edges().count(), 1);
        assert_eq!(loaded.children(root).unwrap().len(), 1);
    }

    #[test]
    fn invalid_json_is_rejected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.json");
        fs::write(&path, "{ not json").unwrap();
        let err = load(&path).unwrap_err();
        assert!(matches!(err, StorageError::Invalid(_)));
    }
}
