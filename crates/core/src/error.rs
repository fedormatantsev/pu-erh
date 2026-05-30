use graph::GraphError;
use storage::StorageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Graph(#[from] GraphError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("parent is required")]
    ParentRequired,
    #[error("invalid query syntax")]
    InvalidQuerySyntax,
    #[error("invalid UUID in query")]
    InvalidQueryUuid,
    #[error("cannot move block to root")]
    CannotMoveRoot,
    #[error("move would create a cycle")]
    CycleDetected,
    #[error("cannot delete root block")]
    DeleteRootForbidden,
    #[error("cannot delete block with children")]
    DeleteWithChildren,
}
