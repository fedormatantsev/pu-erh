use std::sync::Arc;

use crate::bitmask256::Bitmask256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrieDiffEntry<'a, V> {
    pub kind: DiffKind,
    pub key: &'a [u8],
    pub old: Option<&'a V>,
    pub new: Option<&'a V>,
}

impl<'a, V> Copy for TrieDiffEntry<'a, V> {}

impl<'a, V> Clone for TrieDiffEntry<'a, V> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Debug)]
enum Node<V> {
    Inner {
        children_mask: Bitmask256,
        children: Vec<Arc<Node<V>>>,
    },
    Leaf {
        key: Box<[u8]>,
        value: V,
    },
}

#[derive(Debug, Clone)]
pub struct RadixTrieMap<V> {
    root: Option<Arc<Node<V>>>,
    len: usize,
}

impl<V: Clone> Default for RadixTrieMap<V> {
    fn default() -> Self {
        Self {
            root: None,
            len: 0,
        }
    }
}

impl<V: Clone> RadixTrieMap<V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        let node = self.root.as_ref()?;
        get_node(node, key, 0)
    }

    pub fn insert(&self, key: &[u8], value: V) -> Self {
        if key.is_empty() {
            return self.clone();
        }

        match &self.root {
            None => Self {
                root: Some(Arc::new(Node::Leaf {
                    key: key.into(),
                    value,
                })),
                len: 1,
            },
            Some(root) => {
                let (new_root, added) = insert_node(root, key, value, 0);
                Self {
                    root: Some(new_root),
                    len: if added { self.len + 1 } else { self.len },
                }
            }
        }
    }

    pub fn remove(&self, key: &[u8]) -> Self {
        let Some(root) = &self.root else {
            return self.clone();
        };

        let (new_root, removed) = remove_node(root, key, 0);
        Self {
            root: new_root,
            len: if removed {
                self.len.saturating_sub(1)
            } else {
                self.len
            },
        }
    }

    pub fn iter(&self) -> PrefixIter<'_, V> {
        self.iter_prefix(&[])
    }

    pub fn iter_prefix<'a>(&'a self, prefix: &'a [u8]) -> PrefixIter<'a, V> {
        let mut stack = Vec::new();
        if let Some(node) = &self.root {
            push_descendants(node, prefix, &mut stack);
        }
        PrefixIter { stack }
    }

    pub fn winner_under_prefix<'a>(&'a self, prefix: &[u8]) -> Option<(&'a [u8], &'a V)> {
        let mut node = self.root.as_ref()?;
        let mut prefix_pos = 0;

        loop {
            match node.as_ref() {
                Node::Leaf { key, value } => {
                    if prefix_pos < prefix.len()
                        && (key.len() < prefix.len() || key[..prefix.len()] != prefix[..])
                    {
                        return None;
                    }
                    return Some((key.as_ref(), value));
                }
                Node::Inner {
                    children_mask,
                    children,
                } => {
                    if prefix_pos < prefix.len() {
                        let byte = prefix[prefix_pos];
                        prefix_pos += 1;
                        node = child_at(children_mask, children, byte)?;
                    } else {
                        node = last_child(children_mask, children)?;
                    }
                }
            }
        }
    }

    pub fn diff<'a>(&'a self, other: &'a Self) -> TrieDiff<'a, V> {
        TrieDiff {
            pending: vec![DiffStep {
                left: self.root.as_ref(),
                right: other.root.as_ref(),
            }],
            output: Vec::new(),
            output_pos: 0,
        }
    }
}

fn get_node<'a, V>(node: &'a Arc<Node<V>>, key: &[u8], depth: usize) -> Option<&'a V> {
    match node.as_ref() {
        Node::Leaf { key: existing, value } if existing.as_ref() == key => Some(value),
        Node::Leaf { .. } => None,
        Node::Inner {
            children_mask,
            children,
        } => {
            if depth >= key.len() {
                return None;
            }
            let child = child_at(children_mask, children, key[depth])?;
            get_node(child, key, depth + 1)
        }
    }
}

fn insert_node<V: Clone>(
    node: &Arc<Node<V>>,
    key: &[u8],
    value: V,
    depth: usize,
) -> (Arc<Node<V>>, bool) {
    match node.as_ref() {
        Node::Leaf {
            key: existing,
            value: old_value,
        } => {
            if existing.as_ref() == key {
                return (
                    Arc::new(Node::Leaf {
                        key: existing.clone(),
                        value,
                    }),
                    false,
                );
            }

            let diff = first_diff(existing.as_ref(), key);
            let split = split_at(existing.as_ref(), old_value.clone(), key, value, diff);
            let wrapped = wrap_from_depth(split, existing.as_ref(), depth, diff);
            (wrapped, true)
        }
        Node::Inner {
            children_mask,
            children,
        } => {
            if depth >= key.len() {
                return (node.clone(), false);
            }
            let byte = key[depth];
            if let Some(child) = child_at(children_mask, children, byte) {
                let (new_child, added) = insert_node(child, key, value, depth + 1);
                return (
                    Arc::new(Node::Inner {
                        children_mask: *children_mask,
                        children: replace_child(children_mask, children, byte, new_child),
                    }),
                    added,
                );
            }

            let mut new_mask = *children_mask;
            new_mask.set(byte);
            let mut new_children = children.to_vec();
            new_children.insert(
                new_mask.rank(byte),
                Arc::new(Node::Leaf {
                    key: key.into(),
                    value,
                }),
            );
            (
                Arc::new(Node::Inner {
                    children_mask: new_mask,
                    children: new_children,
                }),
                true,
            )
        }
    }
}

fn remove_node<V: Clone>(
    node: &Arc<Node<V>>,
    key: &[u8],
    depth: usize,
) -> (Option<Arc<Node<V>>>, bool) {
    match node.as_ref() {
        Node::Leaf { key: existing, .. } => {
            if existing.as_ref() == key {
                (None, true)
            } else {
                (Some(node.clone()), false)
            }
        }
        Node::Inner {
            children_mask,
            children,
        } => {
            if depth >= key.len() {
                return (Some(node.clone()), false);
            }
            let byte = key[depth];
            let Some(child) = child_at(children_mask, children, byte) else {
                return (Some(node.clone()), false);
            };

            let (new_child, removed) = remove_node(child, key, depth + 1);
            if !removed {
                return (Some(node.clone()), false);
            }

            match new_child {
                None => {
                    let mut new_mask = *children_mask;
                    new_mask.clear(byte);
                    if new_mask.is_empty() {
                        return (None, true);
                    }
                    let mut new_children = children.to_vec();
                    new_children.remove(children_mask.rank(byte));
                    collapse_inner(new_mask, new_children)
                }
                Some(replacement) => collapse_inner(
                    *children_mask,
                    replace_child(children_mask, children, byte, replacement),
                ),
            }
        }
    }
}

fn collapse_inner<V: Clone>(
    mask: Bitmask256,
    children: Vec<Arc<Node<V>>>,
) -> (Option<Arc<Node<V>>>, bool) {
    if children.len() == 1 {
        if let Node::Leaf { .. } = children[0].as_ref() {
            return (Some(children[0].clone()), true);
        }
    }
    (
        Some(Arc::new(Node::Inner {
            children_mask: mask,
            children,
        })),
        true,
    )
}

fn replace_child<V: Clone>(
    mask: &Bitmask256,
    children: &[Arc<Node<V>>],
    index: u8,
    new_child: Arc<Node<V>>,
) -> Vec<Arc<Node<V>>> {
    let rank = mask.rank(index);
    let mut new_children = children.to_vec();
    new_children[rank] = new_child;
    new_children
}

fn first_diff(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| a.len().min(b.len()))
}

fn split_at<V: Clone>(
    existing: &[u8],
    old_value: V,
    new_key: &[u8],
    new_value: V,
    diff: usize,
) -> Arc<Node<V>> {
    let mut mask = Bitmask256::EMPTY;
    mask.set(existing[diff]);
    mask.set(new_key[diff]);

    let mut entries = [
        (existing[diff], existing.to_vec(), old_value),
        (new_key[diff], new_key.to_vec(), new_value),
    ];
    entries.sort_by_key(|(byte, _, _)| *byte);

    let children = entries
        .into_iter()
        .map(|(_, leaf_key, leaf_value)| {
            Arc::new(Node::Leaf {
                key: leaf_key.into_boxed_slice(),
                value: leaf_value,
            })
        })
        .collect();

    Arc::new(Node::Inner {
        children_mask: mask,
        children,
    })
}

fn wrap_from_depth<V>(
    node: Arc<Node<V>>,
    existing: &[u8],
    depth: usize,
    diff: usize,
) -> Arc<Node<V>> {
    if depth >= diff {
        return node;
    }

    let mut current = node;
    for index in (depth..diff).rev() {
        let mut mask = Bitmask256::EMPTY;
        mask.set(existing[index]);
        current = Arc::new(Node::Inner {
            children_mask: mask,
            children: vec![current],
        });
    }
    current
}

fn child_at<'a, V>(
    mask: &Bitmask256,
    children: &'a [Arc<Node<V>>],
    index: u8,
) -> Option<&'a Arc<Node<V>>> {
    if !mask.test(index) {
        return None;
    }
    children.get(mask.rank(index))
}

fn last_child<'a, V>(
    mask: &Bitmask256,
    children: &'a [Arc<Node<V>>],
) -> Option<&'a Arc<Node<V>>> {
    let index = mask.last_set()?;
    child_at(mask, children, index)
}

fn push_descendants<'a, V>(node: &'a Arc<Node<V>>, prefix: &[u8], stack: &mut Vec<LeafRef<'a, V>>) {
    let Some(current) = descend_prefix(node, prefix) else {
        return;
    };
    collect_leaves(current, prefix, stack);
}

fn descend_prefix<'a, V>(
    mut node: &'a Arc<Node<V>>,
    prefix: &[u8],
) -> Option<&'a Arc<Node<V>>> {
    for &byte in prefix {
        match node.as_ref() {
            Node::Leaf { key, .. } if key.starts_with(prefix) => return Some(node),
            Node::Leaf { .. } => return None,
            Node::Inner {
                children_mask,
                children,
            } => {
                node = child_at(children_mask, children, byte)?;
            }
        }
    }
    Some(node)
}

fn collect_leaves<'a, V>(
    node: &'a Arc<Node<V>>,
    prefix: &[u8],
    stack: &mut Vec<LeafRef<'a, V>>,
) {
    match node.as_ref() {
        Node::Leaf { key, value } => {
            if key.starts_with(prefix) {
                stack.push(LeafRef {
                    key: key.as_ref(),
                    value,
                });
            }
        }
        Node::Inner {
            children_mask,
            children,
        } => {
            for index in children_mask.iter_set_bits_rev() {
                if let Some(child) = child_at(children_mask, children, index) {
                    collect_leaves(child, prefix, stack);
                }
            }
        }
    }
}

struct LeafRef<'a, V> {
    key: &'a [u8],
    value: &'a V,
}

pub struct PrefixIter<'a, V> {
    stack: Vec<LeafRef<'a, V>>,
}

impl<'a, V> Iterator for PrefixIter<'a, V> {
    type Item = (&'a [u8], &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let leaf = self.stack.pop()?;
        Some((leaf.key, leaf.value))
    }
}

struct DiffStep<'a, V> {
    left: Option<&'a Arc<Node<V>>>,
    right: Option<&'a Arc<Node<V>>>,
}

pub struct TrieDiff<'a, V> {
    pending: Vec<DiffStep<'a, V>>,
    output: Vec<TrieDiffEntry<'a, V>>,
    output_pos: usize,
}

impl<'a, V: PartialEq> Iterator for TrieDiff<'a, V> {
    type Item = TrieDiffEntry<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.output_pos < self.output.len() {
                let entry = self.output[self.output_pos];
                self.output_pos += 1;
                return Some(entry);
            }

            let step = self.pending.pop()?;
            let left = step.left;
            let right = step.right;

            if left.is_none() && right.is_none() {
                continue;
            }

            if let (Some(left), Some(right)) = (left, right) {
                if Arc::ptr_eq(left, right) {
                    continue;
                }

                match (left.as_ref(), right.as_ref()) {
                    (Node::Leaf { key, value }, Node::Leaf { key: rkey, value: rvalue }) => {
                        if key == rkey {
                            if value != rvalue {
                                return Some(TrieDiffEntry {
                                    kind: DiffKind::Changed,
                                    key: key.as_ref(),
                                    old: Some(value),
                                    new: Some(rvalue),
                                });
                            }
                        } else {
                            self.output.push(TrieDiffEntry {
                                kind: DiffKind::Removed,
                                key: key.as_ref(),
                                old: Some(value),
                                new: None,
                            });
                            self.output.push(TrieDiffEntry {
                                kind: DiffKind::Added,
                                key: rkey.as_ref(),
                                old: None,
                                new: Some(rvalue),
                            });
                            continue;
                        }
                    }
                    (Node::Inner { .. }, Node::Inner { .. }) => {
                        diff_inner(left, right, &mut self.pending);
                    }
                    (left_node, right_node) => {
                        collect_subtree(left_node, true, &mut self.output);
                        collect_subtree(right_node, false, &mut self.output);
                    }
                }
                continue;
            }

            if let Some(left) = left {
                collect_subtree(left, true, &mut self.output);
            }
            if let Some(right) = right {
                collect_subtree(right, false, &mut self.output);
            }
        }
    }
}

fn diff_inner<'a, V>(
    left: &'a Arc<Node<V>>,
    right: &'a Arc<Node<V>>,
    pending: &mut Vec<DiffStep<'a, V>>,
) {
    let (Node::Inner {
        children_mask: left_mask,
        children: left_children,
    }, Node::Inner {
        children_mask: right_mask,
        children: right_children,
    }) = (left.as_ref(), right.as_ref()) else {
        return;
    };

    for index in left_mask.iter_set_bits() {
        pending.push(DiffStep {
            left: child_at(left_mask, left_children, index),
            right: child_at(right_mask, right_children, index),
        });
    }

    for index in right_mask.iter_set_bits() {
        if !left_mask.test(index) {
            if let Some(right_child) = child_at(right_mask, right_children, index) {
                pending.push(DiffStep {
                    left: None,
                    right: Some(right_child),
                });
            }
        }
    }
}

fn collect_subtree<'a, V>(node: &'a Node<V>, is_left: bool, output: &mut Vec<TrieDiffEntry<'a, V>>) {
    match node {
        Node::Leaf { key, value } => {
            output.push(TrieDiffEntry {
                kind: if is_left {
                    DiffKind::Removed
                } else {
                    DiffKind::Added
                },
                key: key.as_ref(),
                old: if is_left { Some(value) } else { None },
                new: if is_left { None } else { Some(value) },
            });
        }
        Node::Inner { children, .. } => {
            for child in children {
                collect_subtree(child.as_ref(), is_left, output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RadixTrieMap;

    #[test]
    fn empty_single_and_replace() {
        let map = RadixTrieMap::<i32>::new();
        assert!(map.is_empty());

        let map = map.insert(b"a", 1);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(b"a"), Some(&1));

        let map = map.insert(b"a", 2);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(b"a"), Some(&2));
    }

    #[test]
    fn shared_prefix_split() {
        let map = RadixTrieMap::new().insert(b"ab", 1).insert(b"ac", 2);
        assert_eq!(map.get(b"ab"), Some(&1));
        assert_eq!(map.get(b"ac"), Some(&2));
    }

    #[test]
    fn remove_and_rank_select() {
        let map = RadixTrieMap::new()
            .insert(b"ab", 1)
            .insert(b"ac", 2)
            .remove(b"ab");
        assert_eq!(map.get(b"ab"), None);
        assert_eq!(map.get(b"ac"), Some(&2));
    }

    #[test]
    fn prefix_and_winner() {
        let map = RadixTrieMap::new()
            .insert(b"id:\x01", 1)
            .insert(b"id:\x02", 2)
            .insert(b"id:\x03", 3);

        let collected: Vec<_> = map.iter_prefix(b"id:").map(|(k, _)| k.to_vec()).collect();
        assert_eq!(collected.len(), 3);

        let (_, winner) = map.winner_under_prefix(b"id:").unwrap();
        assert_eq!(*winner, 3);
    }

    #[test]
    fn diff_iterator() {
        let left = RadixTrieMap::new().insert(b"a", 1);
        let right = left.clone();
        assert_eq!(left.diff(&right).count(), 0);

        let changed = left.insert(b"a", 2);
        let mut diff = left.diff(&changed);
        let entry = diff.next().unwrap();
        assert_eq!(entry.kind, super::DiffKind::Changed);
        assert!(diff.next().is_none());
    }
}
