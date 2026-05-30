use graph::{Block, KnowledgeBase};
use uuid::Uuid;

use crate::error::CoreError;

pub fn execute(kb: &KnowledgeBase, expression: &str) -> Result<Vec<Block>, CoreError> {
    let (kind, id) = parse(expression)?;
    match kind {
        QueryKind::Parent => match kb.parent(id)? {
            Some(block) => Ok(vec![block]),
            None => Ok(Vec::new()),
        },
        QueryKind::Children => kb.children(id).map_err(CoreError::from),
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
    use graph::{EdgeType, KnowledgeBase, Properties};

    fn test_knowledge_base() -> (KnowledgeBase, Uuid, Uuid) {
        let mut kb = KnowledgeBase::empty();
        let root = uuid::Uuid::new_v4();
        kb.append_root_block(root);
        let child = uuid::Uuid::new_v4();
        kb.append_block_version(child, false, Properties::new());
        kb.append_edge_version(
            child,
            root,
            EdgeType::Parent,
            false,
            Properties::new(),
        );
        (kb, root, child)
    }

    #[test]
    fn parent_and_children_queries() {
        let (kb, root, child) = test_knowledge_base();

        let parent = execute(&kb, &format!("parent:{child}")).unwrap();
        assert_eq!(parent.len(), 1);
        assert_eq!(parent[0].id, root);

        let children = execute(&kb, &format!("children:{root}")).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child);
    }

    #[test]
    fn root_parent_query_is_empty() {
        let mut kb = KnowledgeBase::empty();
        let root = uuid::Uuid::new_v4();
        kb.append_root_block(root);
        let result = execute(&kb, &format!("parent:{root}")).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_syntax_and_uuid_are_errors() {
        let kb = KnowledgeBase::empty();
        assert!(matches!(
            execute(&kb, "ancestors:abc").unwrap_err(),
            CoreError::InvalidQuerySyntax
        ));
        assert!(matches!(
            execute(&kb, "parent:not-a-uuid").unwrap_err(),
            CoreError::InvalidQueryUuid
        ));
    }

    #[test]
    fn unknown_block_is_error() {
        let kb = KnowledgeBase::empty();
        let missing = uuid::Uuid::new_v4();
        assert!(execute(&kb, &format!("parent:{missing}")).is_err());
    }
}
