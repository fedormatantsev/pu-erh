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
    #[error("invalid knowledge base: {0}")]
    InvalidGraph(String),
}
