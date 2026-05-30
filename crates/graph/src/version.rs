use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::digest::{hash_block_content, hash_edge_content, Digest};
use crate::model::{EdgeType, Properties};
use crate::snapshot::Snapshot;

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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionHistory {
    pub block_versions: Vec<BlockVersion>,
    pub edge_versions: Vec<EdgeVersion>,
}

impl VersionHistory {
    pub fn append_block(&mut self, version: BlockVersion) {
        self.block_versions.push(version);
    }

    pub fn append_edge(&mut self, version: EdgeVersion) {
        self.edge_versions.push(version);
    }

    pub fn materialize(&self) -> Snapshot {
        Snapshot::materialize(self)
    }

    pub fn winning_block_digest(&self, id: Uuid) -> Option<Digest> {
        self.block_versions
            .iter()
            .filter(|version| version.id == id)
            .max_by(|left, right| {
                left.version
                    .cmp(&right.version)
                    .then_with(|| left.digest.cmp(&right.digest))
            })
            .map(|version| version.digest)
    }

    pub fn winning_edge_digest(&self, identity: &EdgeIdentity) -> Option<Digest> {
        self.edge_versions
            .iter()
            .filter(|version| {
                version.source == identity.source
                    && version.target == identity.target
                    && version.edge_type == identity.edge_type
            })
            .max_by(|left, right| {
                left.version
                    .cmp(&right.version)
                    .then_with(|| left.digest.cmp(&right.digest))
            })
            .map(|version| version.digest)
    }

    pub fn next_block_version(&self, id: Uuid) -> u64 {
        self.block_versions
            .iter()
            .filter(|version| version.id == id)
            .map(|version| version.version)
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn next_edge_version(&self, identity: &EdgeIdentity) -> u64 {
        self.edge_versions
            .iter()
            .filter(|version| {
                version.source == identity.source
                    && version.target == identity.target
                    && version.edge_type == identity.edge_type
            })
            .map(|version| version.version)
            .max()
            .unwrap_or(0)
            + 1
    }
}

pub fn create_root_block_version(root_id: Uuid) -> BlockVersion {
    BlockVersion::new(root_id, 1, None, false, Properties::new())
}

pub fn append_block_version(
    history: &mut VersionHistory,
    id: Uuid,
    tombstoned: bool,
    properties: Properties,
) -> BlockVersion {
    let version = history.next_block_version(id);
    let previous_digest = history.winning_block_digest(id);
    let record = BlockVersion::new(id, version, previous_digest, tombstoned, properties);
    history.append_block(record.clone());
    record
}

pub fn append_edge_version(
    history: &mut VersionHistory,
    source: Uuid,
    target: Uuid,
    edge_type: EdgeType,
    tombstoned: bool,
    properties: Properties,
) -> EdgeVersion {
    let identity = EdgeIdentity {
        source,
        target,
        edge_type,
    };
    let version = history.next_edge_version(&identity);
    let previous_digest = history.winning_edge_digest(&identity);
    let record = EdgeVersion::new(
        source,
        target,
        edge_type,
        version,
        previous_digest,
        tombstoned,
        properties,
    );
    history.append_edge(record.clone());
    record
}

pub fn merge_histories(left: &VersionHistory, right: &VersionHistory) -> VersionHistory {
    let mut seen_blocks = HashSet::new();
    let mut seen_edges = HashSet::new();
    let mut block_versions = Vec::new();
    let mut edge_versions = Vec::new();

    for version in left
        .block_versions
        .iter()
        .chain(right.block_versions.iter())
    {
        if seen_blocks.insert(version.digest) {
            block_versions.push(version.clone());
        }
    }

    for version in left
        .edge_versions
        .iter()
        .chain(right.edge_versions.iter())
    {
        if seen_edges.insert(version.digest) {
            edge_versions.push(version.clone());
        }
    }

    VersionHistory {
        block_versions,
        edge_versions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EdgeType;

    #[test]
    fn next_version_increments() {
        let mut history = VersionHistory::default();
        let id = Uuid::new_v4();
        append_block_version(&mut history, id, false, Properties::new());
        append_block_version(&mut history, id, false, Properties::new());
        assert_eq!(history.next_block_version(id), 3);
    }

    #[test]
    fn merge_dedupes_by_digest() {
        let mut left = VersionHistory::default();
        let id = Uuid::new_v4();
        let version = append_block_version(&mut left, id, false, Properties::new());
        let right = VersionHistory {
            block_versions: vec![version],
            edge_versions: vec![],
        };
        let merged = merge_histories(&left, &right);
        assert_eq!(merged.block_versions.len(), 1);
    }

    #[test]
    fn append_edge_links_previous_digest() {
        let mut history = VersionHistory::default();
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        let first = append_edge_version(
            &mut history,
            source,
            target,
            EdgeType::Parent,
            false,
            Properties::new(),
        );
        let second = append_edge_version(
            &mut history,
            source,
            target,
            EdgeType::Parent,
            true,
            Properties::new(),
        );
        assert_eq!(second.previous_digest, Some(first.digest));
    }
}
