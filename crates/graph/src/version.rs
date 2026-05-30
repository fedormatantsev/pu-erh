use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::digest::{hash_block_content, hash_edge_content, Digest, DigestError};
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
    ) -> Result<Self, DigestError> {
        let digest = hash_block_content(id, version, tombstoned, &properties)?;
        Ok(Self {
            id,
            version,
            digest,
            previous_digest,
            tombstoned,
            properties,
        })
    }

    pub fn verify_digest(&self) -> Result<(), DigestError> {
        let expected = hash_block_content(self.id, self.version, self.tombstoned, &self.properties)?;
        if expected != self.digest {
            return Err(DigestError::Mismatch);
        }
        Ok(())
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
    ) -> Result<Self, DigestError> {
        let digest = hash_edge_content(
            source,
            target,
            edge_type,
            version,
            tombstoned,
            &properties,
        )?;
        Ok(Self {
            source,
            target,
            edge_type,
            version,
            digest,
            previous_digest,
            tombstoned,
            properties,
        })
    }

    pub fn verify_digest(&self) -> Result<(), DigestError> {
        let expected = hash_edge_content(
            self.source,
            self.target,
            self.edge_type,
            self.version,
            self.tombstoned,
            &self.properties,
        )?;
        if expected != self.digest {
            return Err(DigestError::Mismatch);
        }
        Ok(())
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
/// Logical edge identity. Trie/CRDT indexing uses `(target, edge_type, source)` byte order.
pub struct EdgeIdentity {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
}
