use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const PARENT_EDGE_TYPE: &str = "parent";

pub type Properties = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub source: Uuid,
    pub target: Uuid,
    #[serde(rename = "type")]
    pub edge_type: String,
    pub properties: Properties,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeKey(String);

impl EdgeKey {
    pub fn new(target: Uuid, edge_type: &str, source: Uuid) -> Self {
        Self(format!("{target}{edge_type}{source}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn prefix_for(target: Uuid, edge_type: &str) -> String {
        format!("{target}{edge_type}")
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("block not found: {0}")]
    BlockNotFound(Uuid),
    #[error("edge already exists")]
    DuplicateEdge,
    #[error("invalid knowledge base: {0}")]
    InvalidGraph(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    blocks: HashMap<Uuid, Block>,
    edges: HashMap<EdgeKey, Edge>,
    root_id: Uuid,
}

impl Graph {
    pub fn new() -> Self {
        let root_id = Uuid::new_v4();
        let mut blocks = HashMap::new();
        blocks.insert(
            root_id,
            Block {
                id: root_id,
                properties: Properties::new(),
            },
        );
        Self {
            blocks,
            edges: HashMap::new(),
            root_id,
        }
    }

    pub fn from_parts(blocks: HashMap<Uuid, Block>, edges: HashMap<EdgeKey, Edge>) -> Result<Self, GraphError> {
        let root_id = find_root_id(&blocks, &edges)?;
        Ok(Self {
            blocks,
            edges,
            root_id,
        })
    }

    pub fn root_id(&self) -> Uuid {
        self.root_id
    }

    pub fn is_root(&self, id: Uuid) -> bool {
        id == self.root_id
    }

    pub fn block(&self, id: Uuid) -> Option<&Block> {
        self.blocks.get(&id)
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    pub fn insert_block(&mut self, block: Block) {
        self.blocks.insert(block.id, block);
    }

    pub fn insert_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        let key = EdgeKey::new(edge.target, &edge.edge_type, edge.source);
        if self.edges.contains_key(&key) {
            return Err(GraphError::DuplicateEdge);
        }
        self.edges.insert(key, edge);
        Ok(())
    }

    pub fn remove_edge(&mut self, target: Uuid, edge_type: &str, source: Uuid) -> Option<Edge> {
        self.edges
            .remove(&EdgeKey::new(target, edge_type, source))
    }

    pub fn remove_block(&mut self, id: Uuid) -> Option<Block> {
        self.blocks.remove(&id)
    }

    pub fn remove_incident_edges(&mut self, id: Uuid) {
        self.edges.retain(|_, edge| edge.source != id && edge.target != id);
    }

    pub fn children(&self, parent_id: Uuid) -> Result<Vec<Block>, GraphError> {
        if !self.blocks.contains_key(&parent_id) {
            return Err(GraphError::BlockNotFound(parent_id));
        }

        let prefix = EdgeKey::prefix_for(parent_id, PARENT_EDGE_TYPE);
        let mut children = Vec::new();
        for edge in self.edges.values() {
            if edge.edge_type == PARENT_EDGE_TYPE {
                let key = EdgeKey::new(edge.target, &edge.edge_type, edge.source);
                if key.as_str().starts_with(&prefix) {
                    if let Some(block) = self.blocks.get(&edge.source) {
                        children.push(block.clone());
                    }
                }
            }
        }
        Ok(children)
    }

    pub fn parent(&self, child_id: Uuid) -> Result<Option<Block>, GraphError> {
        if !self.blocks.contains_key(&child_id) {
            return Err(GraphError::BlockNotFound(child_id));
        }

        for edge in self.edges.values() {
            if edge.source == child_id && edge.edge_type == PARENT_EDGE_TYPE {
                return Ok(self.blocks.get(&edge.target).cloned());
            }
        }
        Ok(None)
    }

    pub fn parent_edge_target(&self, child_id: Uuid) -> Option<Uuid> {
        self.edges.values().find_map(|edge| {
            if edge.source == child_id && edge.edge_type == PARENT_EDGE_TYPE {
                Some(edge.target)
            } else {
                None
            }
        })
    }

    pub fn has_children(&self, id: Uuid) -> bool {
        self.children(id)
            .map(|children| !children.is_empty())
            .unwrap_or(false)
    }

    pub fn into_parts(self) -> (HashMap<Uuid, Block>, HashMap<EdgeKey, Edge>) {
        (self.blocks, self.edges)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

fn find_root_id(blocks: &HashMap<Uuid, Block>, edges: &HashMap<EdgeKey, Edge>) -> Result<Uuid, GraphError> {
    let roots: Vec<Uuid> = blocks
        .keys()
        .copied()
        .filter(|id| {
            !edges.values().any(|edge| {
                edge.source == *id && edge.edge_type == PARENT_EDGE_TYPE
            })
        })
        .collect();

    match roots.len() {
        1 => Ok(roots[0]),
        0 => Err(GraphError::InvalidGraph("no root block".into())),
        _ => Err(GraphError::InvalidGraph("multiple root blocks".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_has_single_root() {
        let graph = Graph::new();
        assert_eq!(graph.blocks().count(), 1);
        assert!(graph.is_root(graph.root_id()));
        assert!(graph.parent(graph.root_id()).unwrap().is_none());
    }

    #[test]
    fn insert_edge_and_lookup_parent_children() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child_id = Uuid::new_v4();
        graph.insert_block(Block {
            id: child_id,
            properties: Properties::new(),
        });
        graph
            .insert_edge(Edge {
                source: child_id,
                target: root,
                edge_type: PARENT_EDGE_TYPE.into(),
                properties: Properties::new(),
            })
            .unwrap();

        let parent = graph.parent(child_id).unwrap().unwrap();
        assert_eq!(parent.id, root);

        let children = graph.children(root).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child_id);
    }

    #[test]
    fn children_prefix_scan_finds_direct_children_only() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = Uuid::new_v4();
        let grandchild = Uuid::new_v4();

        for id in [child, grandchild] {
            graph.insert_block(Block {
                id,
                properties: Properties::new(),
            });
        }

        graph
            .insert_edge(Edge {
                source: child,
                target: root,
                edge_type: PARENT_EDGE_TYPE.into(),
                properties: Properties::new(),
            })
            .unwrap();
        graph
            .insert_edge(Edge {
                source: grandchild,
                target: child,
                edge_type: PARENT_EDGE_TYPE.into(),
                properties: Properties::new(),
            })
            .unwrap();

        assert_eq!(graph.children(root).unwrap().len(), 1);
        assert_eq!(graph.children(child).unwrap().len(), 1);
    }

    #[test]
    fn duplicate_edge_is_rejected() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = Uuid::new_v4();
        graph.insert_block(Block {
            id: child,
            properties: Properties::new(),
        });
        let edge = Edge {
            source: child,
            target: root,
            edge_type: PARENT_EDGE_TYPE.into(),
            properties: Properties::new(),
        };
        graph.insert_edge(edge.clone()).unwrap();
        assert_eq!(graph.insert_edge(edge), Err(GraphError::DuplicateEdge));
    }

    #[test]
    fn from_parts_validates_single_root() {
        let graph = Graph::new();
        let (mut blocks, edges) = graph.into_parts();
        let orphan = Uuid::new_v4();
        blocks.insert(
            orphan,
            Block {
                id: orphan,
                properties: Properties::new(),
            },
        );

        let err = Graph::from_parts(blocks, edges).unwrap_err();
        assert!(matches!(err, GraphError::InvalidGraph(_)));

        let graph = Graph::new();
        let root = graph.root_id();
        let (blocks, edges) = graph.into_parts();
        let restored = Graph::from_parts(blocks, edges).unwrap();
        assert_eq!(restored.root_id(), root);
    }
}
