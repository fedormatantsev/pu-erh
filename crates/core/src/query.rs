use graph::{Block, Snapshot};
use uuid::Uuid;

use crate::error::CoreError;

pub fn execute(snapshot: &Snapshot, expression: &str) -> Result<Vec<Block>, CoreError> {
    let (kind, id) = parse(expression)?;
    match kind {
        QueryKind::Parent => match snapshot.parent(id)? {
            Some(block) => Ok(vec![block]),
            None => Ok(Vec::new()),
        },
        QueryKind::Children => snapshot.children(id).map_err(CoreError::from),
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
    use graph::{
        append_block_version, append_edge_version, create_root_block_version, EdgeType, Properties,
        Snapshot, VersionHistory,
    };

    fn test_snapshot() -> (Snapshot, Uuid, Uuid) {
        let mut history = VersionHistory::default();
        let root = uuid::Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let child = uuid::Uuid::new_v4();
        append_block_version(&mut history, child, false, Properties::new());
        append_edge_version(
            &mut history,
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );
        (Snapshot::materialize(&history), root, child)
    }

    #[test]
    fn parent_and_children_queries() {
        let (snapshot, root, child) = test_snapshot();

        let parent = execute(&snapshot, &format!("parent:{child}")).unwrap();
        assert_eq!(parent.len(), 1);
        assert_eq!(parent[0].id, root);

        let children = execute(&snapshot, &format!("children:{root}")).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child);
    }

    #[test]
    fn root_parent_query_is_empty() {
        let mut history = VersionHistory::default();
        let root = uuid::Uuid::new_v4();
        history.append_block(create_root_block_version(root));
        let snapshot = Snapshot::materialize(&history);
        let result = execute(&snapshot, &format!("parent:{root}")).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_syntax_and_uuid_are_errors() {
        let snapshot = Snapshot::empty();
        assert!(matches!(
            execute(&snapshot, "ancestors:abc").unwrap_err(),
            CoreError::InvalidQuerySyntax
        ));
        assert!(matches!(
            execute(&snapshot, "parent:not-a-uuid").unwrap_err(),
            CoreError::InvalidQueryUuid
        ));
    }

    #[test]
    fn unknown_block_is_error() {
        let snapshot = Snapshot::empty();
        let missing = uuid::Uuid::new_v4();
        assert!(execute(&snapshot, &format!("parent:{missing}")).is_err());
    }
}
