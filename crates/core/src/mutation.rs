use graph::{
    append_block_version, append_edge_version, GraphError, Properties, Snapshot,
    VersionHistory, PARENT_EDGE_TYPE,
};
use uuid::Uuid;

use crate::error::CoreError;

pub fn create_block(
    history: &mut VersionHistory,
    snapshot: &Snapshot,
    parent: Option<Uuid>,
) -> Result<Uuid, CoreError> {
    let parent = parent.ok_or(CoreError::ParentRequired)?;
    if snapshot.block(parent).is_none() {
        return Err(GraphError::BlockNotFound(parent).into());
    }

    let id = Uuid::new_v4();
    append_block_version(history, id, false, Properties::new());
    append_edge_version(
        history,
        id,
        parent,
        PARENT_EDGE_TYPE,
        false,
        Properties::new(),
    );
    Ok(id)
}

pub fn move_block(
    history: &mut VersionHistory,
    snapshot: &Snapshot,
    id: Uuid,
    new_parent: Option<Uuid>,
) -> Result<(), CoreError> {
    let new_parent = new_parent.ok_or(CoreError::ParentRequired)?;

    if snapshot.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if snapshot.block(new_parent).is_none() {
        return Err(GraphError::BlockNotFound(new_parent).into());
    }
    if snapshot.is_root(id) {
        return Err(CoreError::CannotMoveRoot);
    }
    if new_parent == id || is_ancestor(snapshot, id, new_parent)? {
        return Err(CoreError::CycleDetected);
    }

    if let Some(old_parent) = snapshot.parent_edge_target(id) {
        append_edge_version(
            history,
            id,
            old_parent,
            PARENT_EDGE_TYPE,
            true,
            Properties::new(),
        );
    }

    append_edge_version(
        history,
        id,
        new_parent,
        PARENT_EDGE_TYPE,
        false,
        Properties::new(),
    );
    Ok(())
}

pub fn delete_block(
    history: &mut VersionHistory,
    snapshot: &Snapshot,
    id: Uuid,
) -> Result<(), CoreError> {
    if snapshot.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if snapshot.is_root(id) {
        return Err(CoreError::DeleteRootForbidden);
    }
    if snapshot.has_children(id) {
        return Err(CoreError::DeleteWithChildren);
    }

    if let Some(parent) = snapshot.parent_edge_target(id) {
        append_edge_version(
            history,
            id,
            parent,
            PARENT_EDGE_TYPE,
            true,
            Properties::new(),
        );
    }

    append_block_version(history, id, true, snapshot.block(id).unwrap().properties.clone());
    Ok(())
}

fn is_ancestor(snapshot: &Snapshot, ancestor: Uuid, mut current: Uuid) -> Result<bool, CoreError> {
    while let Some(parent) = snapshot.parent_edge_target(current) {
        if parent == ancestor {
            return Ok(true);
        }
        current = parent;
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{create_root_block_version, Snapshot};

    fn test_state() -> (VersionHistory, Snapshot, Uuid) {
        let mut history = VersionHistory::default();
        let root = Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let snapshot = Snapshot::materialize(&history);
        (history, snapshot, root)
    }

    fn rematerialize(history: &VersionHistory) -> Snapshot {
        Snapshot::materialize(history)
    }

    #[test]
    fn create_requires_parent() {
        let (mut history, snapshot, _) = test_state();
        assert!(matches!(
            create_block(&mut history, &snapshot, None).unwrap_err(),
            CoreError::ParentRequired
        ));
    }

    #[test]
    fn create_child_under_root() {
        let (mut history, snapshot, root) = test_state();
        let child = create_block(&mut history, &snapshot, Some(root)).unwrap();
        let snapshot = rematerialize(&history);
        assert!(snapshot.block(child).is_some());
        assert_eq!(snapshot.parent(child).unwrap().unwrap().id, root);
    }

    #[test]
    fn move_and_delete_leaf() {
        let (mut history, snapshot, root) = test_state();
        let child = create_block(&mut history, &snapshot, Some(root)).unwrap();
        let mut snapshot = rematerialize(&history);
        let sibling = create_block(&mut history, &snapshot, Some(root)).unwrap();
        snapshot = rematerialize(&history);

        move_block(&mut history, &snapshot, child, Some(sibling)).unwrap();
        snapshot = rematerialize(&history);
        assert_eq!(snapshot.parent(child).unwrap().unwrap().id, sibling);

        delete_block(&mut history, &snapshot, child).unwrap();
        snapshot = rematerialize(&history);
        assert!(snapshot.block(child).is_none());
    }

    #[test]
    fn move_to_root_and_cycle_are_rejected() {
        let (mut history, snapshot, root) = test_state();
        let child = create_block(&mut history, &snapshot, Some(root)).unwrap();
        let mut snapshot = rematerialize(&history);
        let grandchild = create_block(&mut history, &snapshot, Some(child)).unwrap();
        snapshot = rematerialize(&history);

        assert!(matches!(
            move_block(&mut history, &snapshot, child, None).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert!(matches!(
            move_block(&mut history, &snapshot, child, Some(grandchild)).unwrap_err(),
            CoreError::CycleDetected
        ));
    }

    #[test]
    fn delete_root_and_parent_with_children_are_rejected() {
        let (mut history, snapshot, root) = test_state();
        let child = create_block(&mut history, &snapshot, Some(root)).unwrap();
        let mut snapshot = rematerialize(&history);
        let _grandchild = create_block(&mut history, &snapshot, Some(child)).unwrap();
        snapshot = rematerialize(&history);

        assert!(matches!(
            delete_block(&mut history, &snapshot, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert!(matches!(
            delete_block(&mut history, &snapshot, child).unwrap_err(),
            CoreError::DeleteWithChildren
        ));
    }

    #[test]
    fn failed_mutations_append_nothing() {
        let (mut history, snapshot, root) = test_state();
        let before_blocks = history.block_versions.len();
        let before_edges = history.edge_versions.len();

        assert!(matches!(
            create_block(&mut history, &snapshot, None).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert_eq!(history.block_versions.len(), before_blocks);
        assert_eq!(history.edge_versions.len(), before_edges);

        let child = create_block(&mut history, &snapshot, Some(root)).unwrap();
        let snapshot = rematerialize(&history);
        let before_blocks = history.block_versions.len();

        assert!(matches!(
            delete_block(&mut history, &snapshot, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert_eq!(history.block_versions.len(), before_blocks);
        assert!(snapshot.block(child).is_some());
    }
}
