use std::collections::HashSet;

use uuid::Uuid;

use crate::model::{Block, Edge, EdgeType, GraphError, PositionHint, Properties};
use crate::property_value::PropertyValue;
use crate::radix_trie::{DiffKind, RadixTrieMap, TrieDiffEntry};
use crate::trie_key::{
    block_entity_prefix, block_version_key_from, edge_entity_prefix, edge_nav_prefix,
    edge_version_key_from, BLOCK_ENTITY_PREFIX_LEN, EDGE_ENTITY_PREFIX_LEN,
};
use crate::version::{BlockVersion, EdgeVersion};

#[derive(Debug, Clone)]
pub struct KnowledgeBase {
    block_versions: RadixTrieMap<BlockVersion>,
    edge_versions: RadixTrieMap<EdgeVersion>,
}

/// Deprecated alias; use [`KnowledgeBase`].
pub type Snapshot = KnowledgeBase;

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

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self {
            block_versions: RadixTrieMap::new(),
            edge_versions: RadixTrieMap::new(),
        }
    }
}

impl KnowledgeBase {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.block_versions.is_empty() && self.edge_versions.is_empty()
    }

    pub fn from_records(
        block_versions: impl IntoIterator<Item = BlockVersion>,
        edge_versions: impl IntoIterator<Item = EdgeVersion>,
    ) -> Self {
        let mut kb = Self::empty();
        for record in block_versions {
            kb.insert_block_record(record);
        }
        for record in edge_versions {
            kb.insert_edge_record(record);
        }
        kb
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut block_versions = self.block_versions.clone();
        for (key, value) in other.block_versions.iter() {
            block_versions = block_versions.insert(key, value.clone());
        }
        let mut edge_versions = self.edge_versions.clone();
        for (key, value) in other.edge_versions.iter() {
            edge_versions = edge_versions.insert(key, value.clone());
        }
        Self {
            block_versions,
            edge_versions,
        }
    }

    pub fn block_version_records(&self) -> Vec<BlockVersion> {
        let mut records: Vec<_> = self
            .block_versions
            .iter()
            .map(|(_, version)| version.clone())
            .collect();
        records.sort_by_key(block_version_key_from);
        records
    }

    pub fn edge_version_records(&self) -> Vec<EdgeVersion> {
        let mut records: Vec<_> = self
            .edge_versions
            .iter()
            .map(|(_, version)| version.clone())
            .collect();
        records.sort_by_key(edge_version_key_from);
        records
    }

    pub fn crdt_winner_block(&self, id: Uuid) -> Option<&BlockVersion> {
        let prefix = block_entity_prefix(id);
        self.block_versions
            .winner_under_prefix(&prefix)
            .map(|(_, version)| version)
    }

    pub fn crdt_winner_edge(
        &self,
        source: Uuid,
        target: Uuid,
        edge_type: EdgeType,
    ) -> Option<&EdgeVersion> {
        let entity = edge_entity_prefix(target, edge_type, source);
        self.edge_versions
            .winner_under_prefix(&entity)
            .map(|(_, version)| version)
    }

    pub fn append_block_version(
        &mut self,
        id: Uuid,
        tombstoned: bool,
        properties: Properties,
    ) -> BlockVersion {
        let version = self
            .crdt_winner_block(id)
            .map(|winner| winner.version + 1)
            .unwrap_or(1);
        let previous_digest = self.crdt_winner_block(id).map(|winner| winner.digest);
        let record = BlockVersion::new(id, version, previous_digest, tombstoned, properties)
            .expect("block version digest must match");
        self.insert_block_record(record.clone());
        record
    }

    pub fn append_edge_version(
        &mut self,
        source: Uuid,
        target: Uuid,
        edge_type: EdgeType,
        tombstoned: bool,
        properties: Properties,
    ) -> EdgeVersion {
        let version = self
            .crdt_winner_edge(source, target, edge_type)
            .map(|winner| winner.version + 1)
            .unwrap_or(1);
        let previous_digest = self
            .crdt_winner_edge(source, target, edge_type)
            .map(|winner| winner.digest);
        let record = EdgeVersion::new(
            source,
            target,
            edge_type,
            version,
            previous_digest,
            tombstoned,
            properties,
        )
        .expect("edge version digest must match");
        self.insert_edge_record(record.clone());
        record
    }

    pub fn append_root_block(&mut self, root_id: Uuid) -> BlockVersion {
        self.append_block_version(root_id, false, Properties::new())
    }

    fn insert_block_record(&mut self, record: BlockVersion) {
        let key = block_version_key_from(&record);
        self.block_versions = self.block_versions.insert(&key, record);
    }

    fn insert_edge_record(&mut self, record: EdgeVersion) {
        let key = edge_version_key_from(&record);
        self.edge_versions = self.edge_versions.insert(&key, record);
    }

    fn active_block_version(&self, id: Uuid) -> Option<&BlockVersion> {
        let version = self.crdt_winner_block(id)?;
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
        let version = self.crdt_winner_edge(source, target, edge_type)?;
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
        Ok(self
            .parent_of(id)
            .and_then(|parent_id| self.active_block(parent_id)))
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

    pub fn parent_edge(&self, child: Uuid) -> Option<Edge> {
        let parent = self.parent_of(child)?;
        self.active_edge(parent, EdgeType::Parent, child)
    }

    fn child_order(&self, child: Uuid) -> String {
        self.parent_edge(child)
            .and_then(|e| match e.properties.get("order") {
                Some(PropertyValue::String(s)) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn children_ordered(&self, parent: Uuid) -> Vec<Uuid> {
        let mut children = self.children_of(parent);
        children.sort_by(|a, b| {
            let oa = self.child_order(*a);
            let ob = self.child_order(*b);
            oa.cmp(&ob).then_with(|| a.as_bytes().cmp(b.as_bytes()))
        });
        children
    }

    pub fn resolve_position(
        &self,
        parent: Uuid,
        hint: PositionHint,
    ) -> Result<(Option<String>, Option<String>), GraphError> {
        let ordered = self.children_ordered(parent);
        // Convert empty-sentinel orders to None so generate_key_between gets valid inputs.
        let valid_order = |s: String| if s.is_empty() { None } else { Some(s) };
        match hint {
            PositionHint::Last => {
                let left = ordered.last().and_then(|id| valid_order(self.child_order(*id)));
                Ok((left, None))
            }
            PositionHint::First => {
                let right = ordered.first().and_then(|id| valid_order(self.child_order(*id)));
                Ok((None, right))
            }
            PositionHint::After(sibling) => {
                let pos = ordered
                    .iter()
                    .position(|&id| id == sibling)
                    .ok_or(GraphError::PositionSiblingNotFound(sibling))?;
                let left = valid_order(self.child_order(ordered[pos]));
                let right = ordered.get(pos + 1).and_then(|id| valid_order(self.child_order(*id)));
                Ok((left, right))
            }
            PositionHint::Before(sibling) => {
                let pos = ordered
                    .iter()
                    .position(|&id| id == sibling)
                    .ok_or(GraphError::PositionSiblingNotFound(sibling))?;
                let right = valid_order(self.child_order(ordered[pos]));
                let left = pos.checked_sub(1).and_then(|i| valid_order(self.child_order(ordered[i])));
                Ok((left, right))
            }
        }
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
    left: &'a KnowledgeBase,
    right: &'a KnowledgeBase,
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
    left: &KnowledgeBase,
    right: &KnowledgeBase,
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
    left: &KnowledgeBase,
    right: &KnowledgeBase,
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
fn explicit_crdt_winner_block(kb: &KnowledgeBase, id: Uuid) -> Option<BlockVersion> {
    kb.block_version_records()
        .into_iter()
        .filter(|version| version.id == id)
        .max_by(|left, right| {
            left.version
                .cmp(&right.version)
                .then_with(|| left.digest.cmp(&right.digest))
        })
}

#[cfg(test)]
use crate::version::EdgeIdentity;

#[cfg(test)]
fn explicit_crdt_winner_edge(
    kb: &KnowledgeBase,
    identity: &EdgeIdentity,
) -> Option<EdgeVersion> {
    kb.edge_version_records()
        .into_iter()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EdgeType;
    use crate::property_value::PropertyValue;
    use crate::version::EdgeIdentity;

    #[test]
    fn append_picks_highest_version() {
        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(
            id,
            false,
            Properties::from([("v".into(), PropertyValue::Number(1.0))]),
        );
        kb.append_block_version(
            id,
            false,
            Properties::from([("v".into(), PropertyValue::Number(2.0))]),
        );
        assert_eq!(
            kb.get_block(id).unwrap().properties["v"],
            PropertyValue::Number(2.0)
        );
    }

    #[test]
    fn tombstoned_winner_excludes_block() {
        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(id, false, Properties::new());
        kb.append_block_version(id, true, Properties::new());
        assert!(kb.get_block(id).is_none());
    }

    #[test]
    fn digest_tie_break_picks_lexicographically_larger() {
        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        let mut first = BlockVersion::new(id, 1, None, false, Properties::new()).unwrap();
        let mut second = BlockVersion::new(id, 1, None, false, Properties::new()).unwrap();
        if first.digest > second.digest {
            std::mem::swap(&mut first.digest, &mut second.digest);
        }
        let expected = if first.digest > second.digest {
            first.digest
        } else {
            second.digest
        };
        kb.insert_block_record(first);
        kb.insert_block_record(second);
        assert!(kb.block(id).is_some());
        assert_eq!(kb.crdt_winner_block(id).unwrap().digest, expected);
    }

    #[test]
    fn branching_knowledge_bases_converge_on_merge() {
        let root = Uuid::new_v4();
        let child = Uuid::new_v4();

        let mut left = KnowledgeBase::empty();
        left.append_root_block(root);
        left.append_block_version(child, false, Properties::new());
        left.append_edge_version(
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );

        let mut right = KnowledgeBase::empty();
        right.append_root_block(root);
        right.append_block_version(child, false, Properties::new());
        right.append_edge_version(
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );

        let merged = left.merge(&right);
        assert_eq!(merged.block_count(), 2);
        assert_eq!(merged.parent_of(child), Some(root));
    }

    #[test]
    fn identical_knowledge_bases_share_trie_roots() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let left = kb.clone();
        let right = left.clone();
        assert_eq!(left.diff(&right).count(), 0);
    }

    #[test]
    fn per_call_reads_recompute_without_cache() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        assert_eq!(kb.root_id().unwrap(), root);
        assert_eq!(kb.block_count(), 1);
        assert_eq!(kb.block_count(), kb.blocks().len());
    }

    #[test]
    fn crdt_winner_matches_explicit_scan_for_blocks() {
        let mut kb = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        kb.append_block_version(id, false, Properties::new());
        kb.append_block_version(id, false, Properties::new());
        kb.append_block_version(id, true, Properties::new());

        let trie_winner = kb.crdt_winner_block(id).unwrap().clone();
        let scan_winner = explicit_crdt_winner_block(&kb, id).unwrap();
        assert_eq!(trie_winner.digest, scan_winner.digest);
        assert_eq!(trie_winner.version, scan_winner.version);
    }

    #[test]
    fn crdt_winner_matches_explicit_scan_for_edges() {
        let mut kb = KnowledgeBase::empty();
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        kb.append_edge_version(
            source,
            target,
            EdgeType::Parent,
            false,
            Properties::new(),
        );
        kb.append_edge_version(
            source,
            target,
            EdgeType::Parent,
            true,
            Properties::new(),
        );

        let identity = EdgeIdentity {
            source,
            target,
            edge_type: EdgeType::Parent,
        };
        let trie_winner = kb
            .crdt_winner_edge(source, target, EdgeType::Parent)
            .unwrap()
            .clone();
        let scan_winner = explicit_crdt_winner_edge(&kb, &identity).unwrap();
        assert_eq!(trie_winner.digest, scan_winner.digest);
    }

    #[test]
    fn append_edge_links_previous_digest() {
        let mut kb = KnowledgeBase::empty();
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        let first = kb.append_edge_version(
            source,
            target,
            EdgeType::Parent,
            false,
            Properties::new(),
        );
        let second = kb.append_edge_version(
            source,
            target,
            EdgeType::Parent,
            true,
            Properties::new(),
        );
        assert_eq!(second.previous_digest, Some(first.digest));
    }

    #[test]
    fn merge_dedupes_identical_records() {
        let mut left = KnowledgeBase::empty();
        let id = Uuid::new_v4();
        left.append_block_version(id, false, Properties::new());
        let merged = left.merge(&left);
        assert_eq!(merged.block_version_records().len(), 1);
    }

    fn make_child_with_order(kb: &mut KnowledgeBase, parent: Uuid, order: Option<&str>) -> Uuid {
        let child = Uuid::new_v4();
        kb.append_block_version(child, false, Properties::new());
        let mut props = Properties::new();
        if let Some(o) = order {
            props.insert("order".into(), PropertyValue::String(o.to_owned()));
        }
        kb.append_edge_version(child, parent, EdgeType::Parent, false, props);
        child
    }

    #[test]
    fn children_ordered_by_order_property() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let c1 = make_child_with_order(&mut kb, root, Some("b0"));
        let c2 = make_child_with_order(&mut kb, root, Some("a0"));
        let c3 = make_child_with_order(&mut kb, root, Some("c0"));
        let ordered = kb.children_ordered(root);
        assert_eq!(ordered, vec![c2, c1, c3]);
    }

    #[test]
    fn legacy_blocks_without_order_sort_first() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let legacy = make_child_with_order(&mut kb, root, None);
        let ordered_child = make_child_with_order(&mut kb, root, Some("a0"));
        let result = kb.children_ordered(root);
        assert_eq!(result[0], legacy);
        assert_eq!(result[1], ordered_child);
    }

    #[test]
    fn tie_breaking_by_uuid() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let c1 = make_child_with_order(&mut kb, root, Some("a0"));
        let c2 = make_child_with_order(&mut kb, root, Some("a0"));
        let result = kb.children_ordered(root);
        let mut expected = vec![c1, c2];
        expected.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        assert_eq!(result, expected);
    }

    #[test]
    fn resolve_position_last_gives_correct_bounds() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        make_child_with_order(&mut kb, root, Some("a0"));
        make_child_with_order(&mut kb, root, Some("b0"));
        let (left, right) = kb.resolve_position(root, PositionHint::Last).unwrap();
        assert_eq!(left, Some("b0".to_owned()));
        assert_eq!(right, None);
    }

    #[test]
    fn resolve_position_before_sibling() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        make_child_with_order(&mut kb, root, Some("a0"));
        let sibling = make_child_with_order(&mut kb, root, Some("b0"));
        let (left, right) = kb.resolve_position(root, PositionHint::Before(sibling)).unwrap();
        assert_eq!(left, Some("a0".to_owned()));
        assert_eq!(right, Some("b0".to_owned()));
    }

    #[test]
    fn resolve_position_invalid_sibling_errors() {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        let stranger = Uuid::new_v4();
        assert!(matches!(
            kb.resolve_position(root, PositionHint::Before(stranger)),
            Err(GraphError::PositionSiblingNotFound(_))
        ));
    }
}
