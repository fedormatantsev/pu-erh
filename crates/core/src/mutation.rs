use graph::{EdgeType, GraphError, KnowledgeBase, Properties, PropertyValue};
use uuid::Uuid;

use crate::error::CoreError;

// Mutations validate graph invariants before appending versions; reads assume
// the active view is already consistent and only apply cheap local checks (tombstone,
// unknown edge type, missing endpoints on point lookups).

pub fn create_block(kb: &mut KnowledgeBase, parent: Option<Uuid>) -> Result<Uuid, CoreError> {
    let parent = parent.ok_or(CoreError::ParentRequired)?;
    if kb.block(parent).is_none() {
        return Err(GraphError::BlockNotFound(parent).into());
    }

    let id = Uuid::new_v4();
    kb.append_block_version(id, false, Properties::new());
    kb.append_edge_version(
        id,
        parent,
        EdgeType::Parent,
        false,
        Properties::new(),
    );
    Ok(id)
}

pub fn move_block(
    kb: &mut KnowledgeBase,
    id: Uuid,
    new_parent: Option<Uuid>,
) -> Result<(), CoreError> {
    let new_parent = new_parent.ok_or(CoreError::ParentRequired)?;

    if kb.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if kb.block(new_parent).is_none() {
        return Err(GraphError::BlockNotFound(new_parent).into());
    }
    if kb.is_root(id) {
        return Err(CoreError::CannotMoveRoot);
    }
    if new_parent == id || is_ancestor(kb, id, new_parent)? {
        return Err(CoreError::CycleDetected);
    }

    if let Some(old_parent) = kb.parent_edge_target(id) {
        kb.append_edge_version(
            id,
            old_parent,
            EdgeType::Parent,
            true,
            Properties::new(),
        );
    }

    kb.append_edge_version(
        id,
        new_parent,
        EdgeType::Parent,
        false,
        Properties::new(),
    );
    Ok(())
}

pub fn delete_block(kb: &mut KnowledgeBase, id: Uuid) -> Result<(), CoreError> {
    if kb.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if kb.is_root(id) {
        return Err(CoreError::DeleteRootForbidden);
    }
    if kb.has_children(id) {
        return Err(CoreError::DeleteWithChildren);
    }

    if let Some(parent) = kb.parent_edge_target(id) {
        kb.append_edge_version(
            id,
            parent,
            EdgeType::Parent,
            true,
            Properties::new(),
        );
    }

    kb.append_block_version(id, true, kb.block(id).unwrap().properties.clone());
    Ok(())
}

pub fn set_property(
    kb: &mut KnowledgeBase,
    id: Uuid,
    key: String,
    value: PropertyValue,
) -> Result<(), CoreError> {
    let block = kb.block(id).ok_or(GraphError::BlockNotFound(id))?;
    let mut properties = block.properties;
    properties.insert(key, value);
    kb.append_block_version(id, false, properties);
    Ok(())
}

fn is_ancestor(kb: &KnowledgeBase, ancestor: Uuid, mut current: Uuid) -> Result<bool, CoreError> {
    while let Some(parent) = kb.parent_edge_target(current) {
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
    use graph::KnowledgeBase;

    fn test_state() -> (KnowledgeBase, Uuid) {
        let mut kb = KnowledgeBase::empty();
        let root = Uuid::new_v4();
        kb.append_root_block(root);
        (kb, root)
    }

    #[test]
    fn create_requires_parent() {
        let (mut kb, _) = test_state();
        assert!(matches!(
            create_block(&mut kb, None).unwrap_err(),
            CoreError::ParentRequired
        ));
    }

    #[test]
    fn create_child_under_root() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root)).unwrap();
        assert!(kb.block(child).is_some());
        assert_eq!(kb.parent(child).unwrap().unwrap().id, root);
    }

    #[test]
    fn move_and_delete_leaf() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root)).unwrap();
        let sibling = create_block(&mut kb, Some(root)).unwrap();

        move_block(&mut kb, child, Some(sibling)).unwrap();
        assert_eq!(kb.parent(child).unwrap().unwrap().id, sibling);

        delete_block(&mut kb, child).unwrap();
        assert!(kb.block(child).is_none());
    }

    #[test]
    fn move_to_root_and_cycle_are_rejected() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root)).unwrap();
        let grandchild = create_block(&mut kb, Some(child)).unwrap();

        assert!(matches!(
            move_block(&mut kb, child, None).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert!(matches!(
            move_block(&mut kb, child, Some(grandchild)).unwrap_err(),
            CoreError::CycleDetected
        ));
    }

    #[test]
    fn delete_root_and_parent_with_children_are_rejected() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root)).unwrap();
        let _grandchild = create_block(&mut kb, Some(child)).unwrap();

        assert!(matches!(
            delete_block(&mut kb, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert!(matches!(
            delete_block(&mut kb, child).unwrap_err(),
            CoreError::DeleteWithChildren
        ));
    }

    #[test]
    fn set_property_sets_and_overwrites() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root)).unwrap();

        set_property(
            &mut kb,
            child,
            "display".to_string(),
            PropertyValue::String("tree".to_string()),
        )
        .unwrap();
        set_property(
            &mut kb,
            child,
            "title".to_string(),
            PropertyValue::String("first".to_string()),
        )
        .unwrap();
        let props = kb.block(child).unwrap().properties;
        assert_eq!(
            props.get("display"),
            Some(&PropertyValue::String("tree".to_string()))
        );
        assert_eq!(
            props.get("title"),
            Some(&PropertyValue::String("first".to_string()))
        );

        set_property(
            &mut kb,
            child,
            "title".to_string(),
            PropertyValue::String("second".to_string()),
        )
        .unwrap();
        let props = kb.block(child).unwrap().properties;
        assert_eq!(
            props.get("title"),
            Some(&PropertyValue::String("second".to_string()))
        );
        // other keys untouched
        assert_eq!(
            props.get("display"),
            Some(&PropertyValue::String("tree".to_string()))
        );
    }

    #[test]
    fn set_property_on_missing_block_errors_and_appends_nothing() {
        let (mut kb, _root) = test_state();
        let missing = Uuid::new_v4();
        let before = kb.block_version_records().len();
        assert!(matches!(
            set_property(&mut kb, missing, "k".to_string(), PropertyValue::Null).unwrap_err(),
            CoreError::Graph(GraphError::BlockNotFound(_))
        ));
        assert_eq!(kb.block_version_records().len(), before);
    }

    #[test]
    fn failed_mutations_append_nothing() {
        let (mut kb, root) = test_state();
        let before_blocks = kb.block_version_records().len();
        let before_edges = kb.edge_version_records().len();

        assert!(matches!(
            create_block(&mut kb, None).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert_eq!(kb.block_version_records().len(), before_blocks);
        assert_eq!(kb.edge_version_records().len(), before_edges);

        let child = create_block(&mut kb, Some(root)).unwrap();
        let before_blocks = kb.block_version_records().len();

        assert!(matches!(
            delete_block(&mut kb, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert_eq!(kb.block_version_records().len(), before_blocks);
        assert!(kb.block(child).is_some());
    }
}
