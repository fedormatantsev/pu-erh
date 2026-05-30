use graph::{Block, Graph};
use uuid::Uuid;

use crate::error::CoreError;

pub fn execute(graph: &Graph, expression: &str) -> Result<Vec<Block>, CoreError> {
    let (kind, id) = parse(expression)?;
    match kind {
        QueryKind::Parent => match graph.parent(id)? {
            Some(block) => Ok(vec![block]),
            None => Ok(Vec::new()),
        },
        QueryKind::Children => graph.children(id).map_err(CoreError::from),
    }
}

enum QueryKind {
    Parent,
    Children,
}

fn parse(expression: &str) -> Result<(QueryKind, Uuid), CoreError> {
    if let Some(id) = expression.strip_prefix("parent:") {
        let id = parse_uuid(id)?;
        return Ok((QueryKind::Parent, id));
    }

    if let Some(id) = expression.strip_prefix("children:") {
        let id = parse_uuid(id)?;
        return Ok((QueryKind::Children, id));
    }

    Err(CoreError::InvalidQuerySyntax)
}

fn parse_uuid(value: &str) -> Result<Uuid, CoreError> {
    value.parse().map_err(|_| CoreError::InvalidQueryUuid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::{Edge, Graph, Properties, PARENT_EDGE_TYPE};

    #[test]
    fn parent_and_children_queries() {
        let mut graph = Graph::new();
        let root = graph.root_id();
        let child = uuid::Uuid::new_v4();
        graph.insert_block(graph::Block {
            id: child,
            properties: Properties::new(),
        });
        graph
            .insert_edge(Edge {
                source: child,
                target: root,
                edge_type: PARENT_EDGE_TYPE.into(),
                properties: Properties::new(),
            })
            .unwrap();

        let parent = execute(&graph, &format!("parent:{child}")).unwrap();
        assert_eq!(parent.len(), 1);
        assert_eq!(parent[0].id, root);

        let children = execute(&graph, &format!("children:{root}")).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child);
    }

    #[test]
    fn root_parent_query_is_empty() {
        let graph = Graph::new();
        let root = graph.root_id();
        let result = execute(&graph, &format!("parent:{root}")).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_syntax_and_uuid_are_errors() {
        let graph = Graph::new();
        assert!(matches!(
            execute(&graph, "ancestors:abc").unwrap_err(),
            CoreError::InvalidQuerySyntax
        ));
        assert!(matches!(
            execute(&graph, "parent:not-a-uuid").unwrap_err(),
            CoreError::InvalidQueryUuid
        ));
    }

    #[test]
    fn unknown_block_is_error() {
        let graph = Graph::new();
        let missing = uuid::Uuid::new_v4();
        assert!(execute(&graph, &format!("parent:{missing}")).is_err());
    }
}
