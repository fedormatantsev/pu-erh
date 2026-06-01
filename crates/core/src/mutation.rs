use graph::{EdgeType, GraphError, KnowledgeBase, OrderError, PositionHint, Properties, PropertyValue};
use uuid::Uuid;

use crate::error::CoreError;

// Mutations validate graph invariants before appending versions; reads assume
// the active view is already consistent and only apply cheap local checks (tombstone,
// unknown edge type, missing endpoints on point lookups).

pub fn create_block(
    kb: &mut KnowledgeBase,
    parent: Option<Uuid>,
    position: PositionHint,
) -> Result<Uuid, CoreError> {
    let parent = parent.ok_or(CoreError::ParentRequired)?;
    if kb.block(parent).is_none() {
        return Err(GraphError::BlockNotFound(parent).into());
    }

    let (left, right) = kb.resolve_position(parent, position)?;
    let order = graph::generate_key_between(left.as_deref(), right.as_deref())
        .map_err(|e: OrderError| GraphError::InvalidGraph(e.to_string()))?;

    let id = Uuid::new_v4();
    kb.append_block_version(id, false, Properties::new());
    let mut edge_props = Properties::new();
    edge_props.insert("order".into(), PropertyValue::String(order));
    kb.append_edge_version(id, parent, EdgeType::Parent, false, edge_props);
    Ok(id)
}

pub fn move_block(
    kb: &mut KnowledgeBase,
    id: Uuid,
    new_parent: Option<Uuid>,
    position: PositionHint,
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

    let (left, right) = kb.resolve_position(new_parent, position)?;
    let order = graph::generate_key_between(left.as_deref(), right.as_deref())
        .map_err(|e: OrderError| GraphError::InvalidGraph(e.to_string()))?;

    if let Some(old_parent) = kb.parent_edge_target(id) {
        kb.append_edge_version(id, old_parent, EdgeType::Parent, true, Properties::new());
    }

    let mut edge_props = Properties::new();
    edge_props.insert("order".into(), PropertyValue::String(order));
    kb.append_edge_version(id, new_parent, EdgeType::Parent, false, edge_props);
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

pub fn remove_property(
    kb: &mut KnowledgeBase,
    id: Uuid,
    key: String,
) -> Result<(), CoreError> {
    let block = kb.block(id).ok_or(GraphError::BlockNotFound(id))?;
    if !block.properties.contains_key(&key) {
        return Err(CoreError::PropertyNotFound(key));
    }
    let mut properties = block.properties;
    properties.remove(&key);
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
    use graph::{KnowledgeBase, PositionHint};

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
            create_block(&mut kb, None, PositionHint::Last).unwrap_err(),
            CoreError::ParentRequired
        ));
    }

    #[test]
    fn create_child_under_root() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        assert!(kb.block(child).is_some());
        assert_eq!(kb.parent(child).unwrap().unwrap().id, root);
    }

    #[test]
    fn move_and_delete_leaf() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let sibling = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();

        move_block(&mut kb, child, Some(sibling), PositionHint::Last).unwrap();
        assert_eq!(kb.parent(child).unwrap().unwrap().id, sibling);

        delete_block(&mut kb, child).unwrap();
        assert!(kb.block(child).is_none());
    }

    #[test]
    fn move_to_root_and_cycle_are_rejected() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let grandchild = create_block(&mut kb, Some(child), PositionHint::Last).unwrap();

        assert!(matches!(
            move_block(&mut kb, child, None, PositionHint::Last).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert!(matches!(
            move_block(&mut kb, child, Some(grandchild), PositionHint::Last).unwrap_err(),
            CoreError::CycleDetected
        ));
    }

    #[test]
    fn delete_root_and_parent_with_children_are_rejected() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let _grandchild = create_block(&mut kb, Some(child), PositionHint::Last).unwrap();

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
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();

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
    fn remove_property_removes_key() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        set_property(&mut kb, child, "foo".to_string(), PropertyValue::String("bar".to_string())).unwrap();
        set_property(&mut kb, child, "baz".to_string(), PropertyValue::Boolean(true)).unwrap();

        remove_property(&mut kb, child, "foo".to_string()).unwrap();

        let props = kb.block(child).unwrap().properties;
        assert!(!props.contains_key("foo"), "key should be removed");
        assert_eq!(props.get("baz"), Some(&PropertyValue::Boolean(true)), "other keys untouched");
    }

    #[test]
    fn remove_property_on_missing_block_errors_and_appends_nothing() {
        let (mut kb, _root) = test_state();
        let missing = Uuid::new_v4();
        let before = kb.block_version_records().len();
        assert!(matches!(
            remove_property(&mut kb, missing, "k".to_string()).unwrap_err(),
            CoreError::Graph(GraphError::BlockNotFound(_))
        ));
        assert_eq!(kb.block_version_records().len(), before);
    }

    #[test]
    fn remove_property_on_missing_key_errors_and_appends_nothing() {
        let (mut kb, root) = test_state();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let before = kb.block_version_records().len();
        assert!(matches!(
            remove_property(&mut kb, child, "nonexistent".to_string()).unwrap_err(),
            CoreError::PropertyNotFound(_)
        ));
        assert_eq!(kb.block_version_records().len(), before);
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
            create_block(&mut kb, None, PositionHint::Last).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert_eq!(kb.block_version_records().len(), before_blocks);
        assert_eq!(kb.edge_version_records().len(), before_edges);

        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let before_blocks = kb.block_version_records().len();

        assert!(matches!(
            delete_block(&mut kb, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert_eq!(kb.block_version_records().len(), before_blocks);
        assert!(kb.block(child).is_some());
    }

    #[test]
    fn create_with_first_position_sorts_before_existing() {
        let (mut kb, root) = test_state();
        let existing = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let first = create_block(&mut kb, Some(root), PositionHint::First).unwrap();
        let ordered = kb.children_ordered(root);
        assert_eq!(ordered[0], first);
        assert_eq!(ordered[1], existing);
    }

    #[test]
    fn create_with_after_position() {
        let (mut kb, root) = test_state();
        let a = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let b = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let mid = create_block(&mut kb, Some(root), PositionHint::After(a)).unwrap();
        let ordered = kb.children_ordered(root);
        assert_eq!(ordered[0], a);
        assert_eq!(ordered[1], mid);
        assert_eq!(ordered[2], b);
    }

    #[test]
    fn create_with_before_position() {
        let (mut kb, root) = test_state();
        let a = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let b = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let mid = create_block(&mut kb, Some(root), PositionHint::Before(b)).unwrap();
        let ordered = kb.children_ordered(root);
        assert_eq!(ordered[0], a);
        assert_eq!(ordered[1], mid);
        assert_eq!(ordered[2], b);
    }

    #[test]
    fn create_before_nonchild_errors() {
        let (mut kb, root) = test_state();
        let stranger = Uuid::new_v4();
        assert!(matches!(
            create_block(&mut kb, Some(root), PositionHint::Before(stranger)).unwrap_err(),
            CoreError::Graph(GraphError::PositionSiblingNotFound(_))
        ));
    }

    #[test]
    fn move_with_position_hint() {
        let (mut kb, root) = test_state();
        let a = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let b = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        let child = create_block(&mut kb, Some(root), PositionHint::Last).unwrap();
        // Move child to be first
        move_block(&mut kb, child, Some(root), PositionHint::First).unwrap();
        let ordered = kb.children_ordered(root);
        assert_eq!(ordered[0], child);
        assert!(ordered.contains(&a));
        assert!(ordered.contains(&b));
    }
}
