use std::collections::HashSet;

use uuid::Uuid;

use crate::model::{Block, Edge, EdgeType, GraphError};
use crate::radix_trie::{DiffKind, RadixTrieMap, TrieDiffEntry};
use crate::trie_key::{
    block_entity_prefix, block_version_key_from, edge_entity_prefix, edge_nav_prefix,
    edge_version_key_from, BLOCK_ENTITY_PREFIX_LEN, EDGE_ENTITY_PREFIX_LEN,
};
use crate::version::{BlockVersion, EdgeVersion, VersionHistory};

#[derive(Debug, Clone)]
pub struct Snapshot {
    block_versions: RadixTrieMap<BlockVersion>,
    edge_versions: RadixTrieMap<EdgeVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotDiffEntity {
    Block(Uuid),
    Edge {
        target: Uuid,
        edge_type: EdgeType,
        source: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotDiffEntry {
    pub entity: SnapshotDiffEntity,
    pub kind: DiffKind,
    pub old: Option<BlockOrEdge>,
    pub new: Option<BlockOrEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockOrEdge {
    Block(Block),
    Edge(Edge),
}

impl Default for Snapshot {
    fn default() -> Self {
        Self {
            block_versions: RadixTrieMap::new(),
            edge_versions: RadixTrieMap::new(),
        }
    }
}

impl Snapshot {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn materialize(history: &VersionHistory) -> Self {
        Self::materialize_from(None, history)
    }

    pub fn materialize_from(previous: Option<&Snapshot>, history: &VersionHistory) -> Self {
        let mut block_versions = previous
            .map(|snapshot| snapshot.block_versions.clone())
            .unwrap_or_default();
        let mut edge_versions = previous
            .map(|snapshot| snapshot.edge_versions.clone())
            .unwrap_or_default();

        for version in &history.block_versions {
            let key = block_version_key_from(version);
            block_versions = block_versions.insert(&key, version.clone());
        }

        for version in &history.edge_versions {
            let key = edge_version_key_from(version);
            edge_versions = edge_versions.insert(&key, version.clone());
        }

        Self {
            block_versions,
            edge_versions,
        }
    }

    fn active_block_version(&self, id: Uuid) -> Option<&BlockVersion> {
        let prefix = block_entity_prefix(id);
        let (_, version) = self.block_versions.winner_under_prefix(&prefix)?;
        if version.tombstoned {
            return None;
        }
        Some(version)
    }

    fn active_block(&self, id: Uuid) -> Option<Block> {
        self.active_block_version(id).map(|version| Block {
            id: version.id,
            properties: version.properties.clone(),
        })
    }

    fn active_edge(&self, target: Uuid, edge_type: EdgeType, source: Uuid) -> Option<Edge> {
        let entity = edge_entity_prefix(target, edge_type, source);
        let (_, version) = self.edge_versions.winner_under_prefix(&entity)?;
        if version.tombstoned {
            return None;
        }
        if self.active_block(source).is_none() || self.active_block(target).is_none() {
            return None;
        }
        Some(edge_from_version(version))
    }

    fn distinct_block_ids(&self) -> Vec<Uuid> {
        let mut seen = HashSet::new();
        let mut ids = Vec::new();
        for (key, _) in self.block_versions.iter() {
            if key.len() < BLOCK_ENTITY_PREFIX_LEN {
                continue;
            }
            let id = Uuid::from_bytes(key[..BLOCK_ENTITY_PREFIX_LEN].try_into().expect("uuid"));
            if seen.insert(id) {
                ids.push(id);
            }
        }
        ids
    }

    fn distinct_edge_entities(&self) -> Vec<[u8; EDGE_ENTITY_PREFIX_LEN]> {
        let mut seen = HashSet::new();
        let mut entities = Vec::new();
        for (key, _) in self.edge_versions.iter() {
            if key.len() < EDGE_ENTITY_PREFIX_LEN {
                continue;
            }
            let mut entity = [0u8; EDGE_ENTITY_PREFIX_LEN];
            entity.copy_from_slice(&key[..EDGE_ENTITY_PREFIX_LEN]);
            if seen.insert(entity) {
                entities.push(entity);
            }
        }
        entities
    }

    fn active_edge_from_entity(&self, entity: [u8; EDGE_ENTITY_PREFIX_LEN]) -> Option<Edge> {
        let edge_type = edge_type_from_entity_byte(entity[16])?;
        let target = Uuid::from_bytes(entity[..16].try_into().ok()?);
        let source = Uuid::from_bytes(entity[17..33].try_into().ok()?);
        self.active_edge(target, edge_type, source)
    }

    pub fn root_id(&self) -> Result<Uuid, GraphError> {
        let mut roots = Vec::new();
        for id in self.distinct_block_ids() {
            if self.active_block(id).is_none() {
                continue;
            }
            if self.parent_of(id).is_none() {
                roots.push(id);
            }
        }
        match roots.len() {
            1 => Ok(roots[0]),
            _ => Err(GraphError::InvalidGraph("no valid root block".into())),
        }
    }

    pub fn block(&self, id: Uuid) -> Option<Block> {
        self.active_block(id)
    }

    pub fn get_block(&self, id: Uuid) -> Option<Block> {
        self.block(id)
    }

    pub fn get_edge(&self, source: Uuid, target: Uuid, edge_type: EdgeType) -> Option<Edge> {
        self.active_edge(target, edge_type, source)
    }

    pub fn parent_of(&self, child: Uuid) -> Option<Uuid> {
        for entity in self.distinct_edge_entities() {
            if entity[16] != EdgeType::Parent as u8 {
                continue;
            }
            let source = Uuid::from_bytes(entity[17..33].try_into().ok()?);
            if source != child {
                continue;
            }
            let target = Uuid::from_bytes(entity[..16].try_into().ok()?);
            if self
                .active_edge(target, EdgeType::Parent, source)
                .is_some()
            {
                return Some(target);
            }
        }
        None
    }

    pub fn parent(&self, id: Uuid) -> Result<Option<Block>, GraphError> {
        if self.active_block(id).is_none() {
            return Err(GraphError::BlockNotFound(id));
        }
        Ok(self.parent_of(id).and_then(|parent_id| self.active_block(parent_id)))
    }

    pub fn children(&self, id: Uuid) -> Result<Vec<Block>, GraphError> {
        if self.active_block(id).is_none() {
            return Err(GraphError::BlockNotFound(id));
        }
        Ok(self
            .children_of(id)
            .into_iter()
            .filter_map(|child_id| self.active_block(child_id))
            .collect())
    }

    pub fn children_of(&self, parent: Uuid) -> Vec<Uuid> {
        let nav = edge_nav_prefix(parent, EdgeType::Parent);
        let mut seen = HashSet::new();
        let mut children = Vec::new();
        for (key, _) in self.edge_versions.iter_prefix(&nav) {
            if key.len() < EDGE_ENTITY_PREFIX_LEN {
                continue;
            }
            let source = Uuid::from_bytes(key[17..33].try_into().expect("source"));
            if !seen.insert(source) {
                continue;
            }
            if self
                .active_edge(parent, EdgeType::Parent, source)
                .is_some()
            {
                children.push(source);
            }
        }
        children
    }

    pub fn is_root(&self, id: Uuid) -> bool {
        self.root_id().ok() == Some(id)
    }

    pub fn has_children(&self, id: Uuid) -> bool {
        !self.children_of(id).is_empty()
    }

    pub fn parent_edge_target(&self, child: Uuid) -> Option<Uuid> {
        self.parent_of(child)
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.distinct_block_ids()
            .into_iter()
            .filter_map(|id| self.active_block(id))
            .collect()
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.distinct_edge_entities()
            .into_iter()
            .filter_map(|entity| self.active_edge_from_entity(entity))
            .collect()
    }

    pub fn edges_with_prefix(&self, prefix: &[u8]) -> Vec<Edge> {
        self.distinct_edge_entities()
            .into_iter()
            .filter(|entity| entity.starts_with(prefix))
            .filter_map(|entity| self.active_edge_from_entity(entity))
            .collect()
    }

    pub fn block_count(&self) -> usize {
        self.blocks().len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges().len()
    }

    pub fn diff<'a>(&'a self, other: &'a Self) -> SnapshotDiff<'a> {
        SnapshotDiff {
            block_diff: self.block_versions.diff(&other.block_versions),
            edge_diff: self.edge_versions.diff(&other.edge_versions),
            left: self,
            right: other,
            block_done: false,
            edge_done: false,
        }
    }
}

pub struct SnapshotDiff<'a> {
    block_diff: crate::radix_trie::TrieDiff<'a, BlockVersion>,
    edge_diff: crate::radix_trie::TrieDiff<'a, EdgeVersion>,
    left: &'a Snapshot,
    right: &'a Snapshot,
    block_done: bool,
    edge_done: bool,
}

impl<'a> Iterator for SnapshotDiff<'a> {
    type Item = SnapshotDiffEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.block_done {
            if let Some(entry) = self.block_diff.next() {
                return Some(decode_block_diff(entry, self.left, self.right));
            }
            self.block_done = true;
        }
        if !self.edge_done {
            if let Some(entry) = self.edge_diff.next() {
                return Some(decode_edge_diff(entry, self.left, self.right));
            }
            self.edge_done = true;
        }
        None
    }
}

fn decode_block_diff(
    entry: TrieDiffEntry<'_, BlockVersion>,
    left: &Snapshot,
    right: &Snapshot,
) -> SnapshotDiffEntry {
    let id = Uuid::from_bytes(entry.key[..16].try_into().expect("block id"));
    SnapshotDiffEntry {
        entity: SnapshotDiffEntity::Block(id),
        kind: entry.kind,
        old: entry.old.and_then(|_| left.block(id)).map(BlockOrEdge::Block),
        new: entry.new.and_then(|_| right.block(id)).map(BlockOrEdge::Block),
    }
}

fn decode_edge_diff(
    entry: TrieDiffEntry<'_, EdgeVersion>,
    left: &Snapshot,
    right: &Snapshot,
) -> SnapshotDiffEntry {
    let target = Uuid::from_bytes(entry.key[..16].try_into().expect("target"));
    let edge_type = edge_type_from_entity_byte(entry.key[16]).unwrap_or(EdgeType::Parent);
    let source = Uuid::from_bytes(entry.key[17..33].try_into().expect("source"));
    SnapshotDiffEntry {
        entity: SnapshotDiffEntity::Edge {
            target,
            edge_type,
            source,
        },
        kind: entry.kind,
        old: left
            .get_edge(source, target, edge_type)
            .map(BlockOrEdge::Edge),
        new: right
            .get_edge(source, target, edge_type)
            .map(BlockOrEdge::Edge),
    }
}

fn edge_type_from_entity_byte(byte: u8) -> Option<EdgeType> {
    if byte == EdgeType::Parent as u8 {
        Some(EdgeType::Parent)
    } else {
        None
    }
}

fn edge_from_version(version: &EdgeVersion) -> Edge {
    Edge {
        source: version.source,
        target: version.target,
        edge_type: version.edge_type,
        properties: version.properties.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{EdgeType, Properties};
    use crate::version::{append_block_version, append_edge_version, create_root_block_version};

    #[test]
    fn materialize_picks_highest_version() {
        let mut history = VersionHistory::default();
        let id = Uuid::new_v4();
        append_block_version(
            &mut history,
            id,
            false,
            Properties::from([("v".into(), serde_json::json!(1))]),
        );
        append_block_version(
            &mut history,
            id,
            false,
            Properties::from([("v".into(), serde_json::json!(2))]),
        );
        let snapshot = Snapshot::materialize(&history);
        assert_eq!(
            snapshot.get_block(id).unwrap().properties["v"],
            serde_json::json!(2)
        );
    }

    #[test]
    fn tombstoned_winner_excludes_block() {
        let mut history = VersionHistory::default();
        let id = Uuid::new_v4();
        append_block_version(&mut history, id, false, Properties::new());
        append_block_version(&mut history, id, true, Properties::new());
        let snapshot = Snapshot::materialize(&history);
        assert!(snapshot.get_block(id).is_none());
    }

    #[test]
    fn digest_tie_break_picks_lexicographically_larger() {
        let mut history = VersionHistory::default();
        let id = Uuid::new_v4();
        let mut first = BlockVersion::new(id, 1, None, false, Properties::new());
        let mut second = BlockVersion::new(id, 1, None, false, Properties::new());
        if first.digest > second.digest {
            std::mem::swap(&mut first.digest, &mut second.digest);
        }
        let expected = if first.digest > second.digest {
            first.digest
        } else {
            second.digest
        };
        history.append_block(first);
        history.append_block(second);
        let snapshot = Snapshot::materialize(&history);
        assert!(snapshot.block(id).is_some());
        let winner = history
            .block_versions
            .iter()
            .filter(|version| version.id == id)
            .max_by(|left, right| {
                left.version
                    .cmp(&right.version)
                    .then_with(|| left.digest.cmp(&right.digest))
            })
            .unwrap();
        assert_eq!(winner.digest, expected);
    }

    #[test]
    fn branching_histories_converge_on_merge() {
        let root = Uuid::new_v4();
        let child = Uuid::new_v4();

        let mut left = VersionHistory::default();
        left.append_block(create_root_block_version(root));
        append_block_version(&mut left, child, false, Properties::new());
        append_edge_version(
            &mut left,
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );

        let mut right = VersionHistory::default();
        right.append_block(create_root_block_version(root));
        append_block_version(&mut right, child, false, Properties::new());
        append_edge_version(
            &mut right,
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );

        let merged = crate::version::merge_histories(&left, &right);
        let snapshot = Snapshot::materialize(&merged);
        assert_eq!(snapshot.block_count(), 2);
        assert_eq!(snapshot.parent_of(child), Some(root));
    }

    #[test]
    fn identical_histories_share_trie_roots() {
        let mut history = VersionHistory::default();
        let root = Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let left = Snapshot::materialize(&history);
        let right = left.clone();
        assert_eq!(left.diff(&right).count(), 0);
    }

    #[test]
    fn incremental_materialize_matches_full_rebuild() {
        let mut history = VersionHistory::default();
        let root = Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let partial = Snapshot::materialize(&history);
        append_block_version(&mut history, Uuid::new_v4(), false, Properties::new());
        let incremental = Snapshot::materialize_from(Some(&partial), &history);
        let full = Snapshot::materialize(&history);
        assert_eq!(incremental.block_count(), full.block_count());
        assert_eq!(incremental.edge_count(), full.edge_count());
    }

    #[test]
    fn per_call_reads_recompute_without_cache() {
        let mut history = VersionHistory::default();
        let root = Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let snapshot = Snapshot::materialize(&history);
        assert_eq!(snapshot.root_id().unwrap(), root);
        assert_eq!(snapshot.block_count(), 1);
        assert_eq!(snapshot.block_count(), snapshot.blocks().len());
    }
}
