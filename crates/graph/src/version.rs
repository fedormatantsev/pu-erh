use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::digest::{hash_block_content, hash_edge_content, Digest};
use crate::model::{EdgeType, Properties};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockVersion {
    pub id: Uuid,
    pub version: u64,
    #[serde(with = "crate::digest::serde_hex")]
    pub digest: Digest,
    #[serde(
        with = "crate::digest::serde_hex::option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_digest: Option<Digest>,
    pub tombstoned: bool,
    pub properties: Properties,
}

impl BlockVersion {
    pub fn new(
        id: Uuid,
        version: u64,
        previous_digest: Option<Digest>,
        tombstoned: bool,
        properties: Properties,
    ) -> Self {
        let digest = hash_block_content(id, version, tombstoned, &properties);
        Self {
            id,
            version,
            digest,
            previous_digest,
            tombstoned,
            properties,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeVersion {
    pub source: Uuid,
    pub target: Uuid,
    #[serde(rename = "type")]
    pub edge_type: EdgeType,
    pub version: u64,
    #[serde(with = "crate::digest::serde_hex")]
    pub digest: Digest,
    #[serde(
        with = "crate::digest::serde_hex::option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_digest: Option<Digest>,
    pub tombstoned: bool,
    pub properties: Properties,
}

impl EdgeVersion {
    pub fn new(
        source: Uuid,
        target: Uuid,
        edge_type: EdgeType,
        version: u64,
        previous_digest: Option<Digest>,
        tombstoned: bool,
        properties: Properties,
    ) -> Self {
        let digest = hash_edge_content(
            source,
            target,
            edge_type,
            version,
            tombstoned,
            &properties,
        );
        Self {
            source,
            target,
            edge_type,
            version,
            digest,
            previous_digest,
            tombstoned,
            properties,
        }
    }

    pub fn identity(&self) -> EdgeIdentity {
        EdgeIdentity {
            source: self.source,
            target: self.target,
            edge_type: self.edge_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeIdentity {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
}
