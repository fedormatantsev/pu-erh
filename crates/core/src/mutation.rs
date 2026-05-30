use graph::{Block, Edge, Graph, GraphError, Properties, PARENT_EDGE_TYPE};
use uuid::Uuid;

use crate::error::CoreError;

pub fn create_block(graph: &mut Graph, parent: Option<Uuid>) -> Result<Uuid, CoreError> {
    let parent = parent.ok_or(CoreError::ParentRequired)?;
    if graph.block(parent).is_none() {
        return Err(GraphError::BlockNotFound(parent).into());
    }

    let id = Uuid::new_v4();
    graph.insert_block(Block {
        id,
        properties: Properties::new(),
    });
    graph.insert_edge(Edge {
        source: id,
        target: parent,
        edge_type: PARENT_EDGE_TYPE.into(),
        properties: Properties::new(),
    })?;
    Ok(id)
}

pub fn move_block(graph: &mut Graph, id: Uuid, new_parent: Option<Uuid>) -> Result<(), CoreError> {
    let new_parent = new_parent.ok_or(CoreError::ParentRequired)?;

    if graph.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if graph.block(new_parent).is_none() {
        return Err(GraphError::BlockNotFound(new_parent).into());
    }
    if graph.is_root(id) {
        return Err(CoreError::CannotMoveRoot);
    }
    if new_parent == id || is_ancestor(graph, id, new_parent)? {
        return Err(CoreError::CycleDetected);
    }

    if let Some(old_parent) = graph.parent_edge_target(id) {
        graph.remove_edge(old_parent, PARENT_EDGE_TYPE, id);
    }

    graph.insert_edge(Edge {
        source: id,
        target: new_parent,
        edge_type: PARENT_EDGE_TYPE.into(),
        properties: Properties::new(),
    })?;
    Ok(())
}

pub fn delete_block(graph: &mut Graph, id: Uuid) -> Result<(), CoreError> {
    if graph.block(id).is_none() {
        return Err(GraphError::BlockNotFound(id).into());
    }
    if graph.is_root(id) {
        return Err(CoreError::DeleteRootForbidden);
    }
    if graph.has_children(id) {
        return Err(CoreError::DeleteWithChildren);
    }

    if let Some(parent) = graph.parent_edge_target(id) {
        graph.remove_edge(parent, PARENT_EDGE_TYPE, id);
    }
    graph.remove_incident_edges(id);
    graph.remove_block(id);
    Ok(())
}

fn is_ancestor(graph: &Graph, ancestor: Uuid, mut current: Uuid) -> Result<bool, CoreError> {
    while let Some(parent) = graph.parent_edge_target(current) {
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
    use graph::Graph;

    #[test]
    fn create_requires_parent() {
        let mut graph = Graph::new();
        assert!(matches!(
            create_block(&mut graph, None).unwrap_err(),
            CoreError::ParentRequired
        ));
    }

    #[test]
    fn create_child_under_root() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = create_block(&mut graph, Some(root)).unwrap();
        assert!(graph.block(child).is_some());
        assert_eq!(graph.parent(child).unwrap().unwrap().id, root);
    }

    #[test]
    fn move_and_delete_leaf() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = create_block(&mut graph, Some(root)).unwrap();
        let sibling = create_block(&mut graph, Some(root)).unwrap();

        move_block(&mut graph, child, Some(sibling)).unwrap();
        assert_eq!(graph.parent(child).unwrap().unwrap().id, sibling);

        delete_block(&mut graph, child).unwrap();
        assert!(graph.block(child).is_none());
    }

    #[test]
    fn move_to_root_and_cycle_are_rejected() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = create_block(&mut graph, Some(root)).unwrap();
        let grandchild = create_block(&mut graph, Some(child)).unwrap();

        assert!(matches!(
            move_block(&mut graph, child, None).unwrap_err(),
            CoreError::ParentRequired
        ));
        assert!(matches!(
            move_block(&mut graph, child, Some(grandchild)).unwrap_err(),
            CoreError::CycleDetected
        ));
    }

    #[test]
    fn delete_root_and_parent_with_children_are_rejected() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = create_block(&mut graph, Some(root)).unwrap();
        let _grandchild = create_block(&mut graph, Some(child)).unwrap();

        assert!(matches!(
            delete_block(&mut graph, root).unwrap_err(),
            CoreError::DeleteRootForbidden
        ));
        assert!(matches!(
            delete_block(&mut graph, child).unwrap_err(),
            CoreError::DeleteWithChildren
        ));
    }
}
