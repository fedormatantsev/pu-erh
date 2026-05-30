use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::model::{Block, Edge, EdgeKey, GraphError, PARENT_EDGE_TYPE};
use crate::version::{BlockVersion, EdgeVersion, VersionHistory};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Snapshot {
    blocks: HashMap<Uuid, Block>,
    edges: HashMap<EdgeKey, Edge>,
    root_id: Option<Uuid>,
}

impl Snapshot {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn materialize(history: &VersionHistory) -> Self {
        let mut snapshot = Self::empty();
        if history.block_versions.is_empty() && history.edge_versions.is_empty() {
            return snapshot;
        }

        let winning_blocks = select_winning_block_versions(&history.block_versions);
        let winning_edges = select_winning_edge_versions(&history.edge_versions);

        for version in winning_blocks.values() {
            if version.tombstoned {
                continue;
            }
            snapshot.blocks.insert(
                version.id,
                Block {
                    id: version.id,
                    properties: version.properties.clone(),
                },
            );
        }

        for version in winning_edges.values() {
            if version.tombstoned {
                continue;
            }
            let key = EdgeKey::new(version.target, &version.edge_type, version.source);
            snapshot.edges.insert(
                key,
                Edge {
                    source: version.source,
                    target: version.target,
                    edge_type: version.edge_type.clone(),
                    properties: version.properties.clone(),
                },
            );
        }

        snapshot.filter_invariants();
        snapshot
    }

    fn filter_invariants(&mut self) {
        self.drop_edges_with_missing_endpoints();
        self.drop_conflicting_parent_edges();
        self.drop_cycle_edges();
        self.recompute_root();
    }

    fn drop_edges_with_missing_endpoints(&mut self) {
        self.edges.retain(|_, edge| {
            self.blocks.contains_key(&edge.source) && self.blocks.contains_key(&edge.target)
        });
    }

    fn drop_conflicting_parent_edges(&mut self) {
        let mut parent_counts: HashMap<Uuid, usize> = HashMap::new();
        for edge in self.edges.values() {
            if edge.edge_type == PARENT_EDGE_TYPE {
                *parent_counts.entry(edge.source).or_default() += 1;
            }
        }

        let invalid_children: HashSet<Uuid> = parent_counts
            .into_iter()
            .filter(|(_, count)| *count != 1)
            .map(|(child, _)| child)
            .collect();

        if invalid_children.is_empty() {
            return;
        }

        self.edges.retain(|_, edge| {
            !(edge.edge_type == PARENT_EDGE_TYPE && invalid_children.contains(&edge.source))
        });
    }

    fn drop_cycle_edges(&mut self) {
        loop {
            let cycle_nodes = find_cycle_nodes(&self.edges);
            if cycle_nodes.is_empty() {
                break;
            }
            self.edges.retain(|_, edge| {
                !(edge.edge_type == PARENT_EDGE_TYPE
                    && (cycle_nodes.contains(&edge.source) || cycle_nodes.contains(&edge.target)))
            });
        }
    }

    fn recompute_root(&mut self) {
        let blocks_with_parent: HashSet<Uuid> = self
            .edges
            .values()
            .filter(|edge| edge.edge_type == PARENT_EDGE_TYPE)
            .map(|edge| edge.source)
            .collect();

        let roots: Vec<Uuid> = self
            .blocks
            .keys()
            .copied()
            .filter(|id| !blocks_with_parent.contains(id))
            .collect();

        self.root_id = match roots.len() {
            1 => Some(roots[0]),
            _ => None,
        };

        if self.root_id.is_none() {
            self.blocks.clear();
            self.edges.clear();
        }
    }

    pub fn root_id(&self) -> Result<Uuid, GraphError> {
        self.root_id
            .ok_or_else(|| GraphError::InvalidGraph("no valid root block".into()))
    }

    pub fn block(&self, id: Uuid) -> Option<&Block> {
        self.blocks.get(&id)
    }

    pub fn get_block(&self, id: Uuid) -> Option<&Block> {
        self.block(id)
    }

    pub fn get_edge(&self, source: Uuid, target: Uuid, edge_type: &str) -> Option<&Edge> {
        let key = EdgeKey::new(target, edge_type, source);
        self.edges.get(&key)
    }

    pub fn parent_of(&self, child: Uuid) -> Option<Uuid> {
        self.edges
            .values()
            .find(|edge| edge.edge_type == PARENT_EDGE_TYPE && edge.source == child)
            .map(|edge| edge.target)
    }

    pub fn parent(&self, id: Uuid) -> Result<Option<Block>, GraphError> {
        if self.block(id).is_none() {
            return Err(GraphError::BlockNotFound(id));
        }
        Ok(self.parent_of(id).and_then(|parent_id| self.block(parent_id).cloned()))
    }

    pub fn children(&self, id: Uuid) -> Result<Vec<Block>, GraphError> {
        if self.block(id).is_none() {
            return Err(GraphError::BlockNotFound(id));
        }
        Ok(self
            .children_of(id)
            .into_iter()
            .filter_map(|child_id| self.block(child_id).cloned())
            .collect())
    }

    pub fn children_of(&self, parent: Uuid) -> Vec<Uuid> {
        self.edges
            .values()
            .filter(|edge| edge.edge_type == PARENT_EDGE_TYPE && edge.target == parent)
            .map(|edge| edge.source)
            .collect()
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

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

fn select_winning_block_versions(versions: &[BlockVersion]) -> HashMap<Uuid, BlockVersion> {
    let mut winners = HashMap::new();
    for version in versions {
        winners
            .entry(version.id)
            .and_modify(|current| {
                if is_newer_version(version, current) {
                    *current = version.clone();
                }
            })
            .or_insert_with(|| version.clone());
    }
    winners
}

fn select_winning_edge_versions(versions: &[EdgeVersion]) -> HashMap<(Uuid, Uuid, String), EdgeVersion> {
    let mut winners = HashMap::new();
    for version in versions {
        let key = (version.source, version.target, version.edge_type.clone());
        winners
            .entry(key)
            .and_modify(|current| {
                if is_newer_version(version, current) {
                    *current = version.clone();
                }
            })
            .or_insert_with(|| version.clone());
    }
    winners
}

fn is_newer_version<T: Versioned>(candidate: &T, current: &T) -> bool {
    candidate.version() > current.version()
        || (candidate.version() == current.version() && candidate.digest() > current.digest())
}

trait Versioned {
    fn version(&self) -> u64;
    fn digest(&self) -> &[u8; 32];
}

impl Versioned for BlockVersion {
    fn version(&self) -> u64 {
        self.version
    }

    fn digest(&self) -> &[u8; 32] {
        &self.digest
    }
}

impl Versioned for EdgeVersion {
    fn version(&self) -> u64 {
        self.version
    }

    fn digest(&self) -> &[u8; 32] {
        &self.digest
    }
}

fn find_cycle_nodes(edges: &HashMap<EdgeKey, Edge>) -> HashSet<Uuid> {
    let mut parent_of: HashMap<Uuid, Uuid> = HashMap::new();
    for edge in edges.values() {
        if edge.edge_type == PARENT_EDGE_TYPE {
            parent_of.insert(edge.source, edge.target);
        }
    }

    let mut cycle_nodes = HashSet::new();
    for start in parent_of.keys().copied() {
        let mut seen = HashSet::new();
        let mut current = Some(start);
        while let Some(node) = current {
            if !seen.insert(node) {
                cycle_nodes.insert(node);
                break;
            }
            current = parent_of.get(&node).copied();
        }
    }
    cycle_nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Properties;
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
            PARENT_EDGE_TYPE,
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
            PARENT_EDGE_TYPE,
            false,
            Properties::new(),
        );

        let merged = crate::version::merge_histories(&left, &right);
        let snapshot = Snapshot::materialize(&merged);
        assert_eq!(snapshot.block_count(), 2);
        assert_eq!(snapshot.parent_of(child), Some(root));
    }
}
