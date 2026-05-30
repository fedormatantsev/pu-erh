use std::path::{Path, PathBuf};

use graph::{Block, Graph};
use storage::{load, save};
use uuid::Uuid;

use crate::error::CoreError;
use crate::mutation;
use crate::query;

pub struct Session {
    graph: Graph,
    path: PathBuf,
    dirty: bool,
}

impl Session {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref().to_path_buf();
        let existed = path.exists();
        let graph = load(&path)?;
        Ok(Self {
            graph,
            path,
            dirty: !existed,
        })
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn root_id(&self) -> Uuid {
        self.graph.root_id()
    }

    pub fn save(&mut self) -> Result<(), CoreError> {
        if self.dirty {
            save(&self.path, &self.graph)?;
            self.dirty = false;
        }
        Ok(())
    }

    pub fn query(&self, expression: &str) -> Result<Vec<Block>, CoreError> {
        query::execute(&self.graph, expression)
    }

    pub fn create_block(&mut self, parent: Option<Uuid>) -> Result<Uuid, CoreError> {
        let id = mutation::create_block(&mut self.graph, parent)?;
        self.dirty = true;
        Ok(id)
    }

    pub fn move_block(&mut self, id: Uuid, new_parent: Option<Uuid>) -> Result<(), CoreError> {
        mutation::move_block(&mut self.graph, id, new_parent)?;
        self.dirty = true;
        Ok(())
    }

    pub fn delete_block(&mut self, id: Uuid) -> Result<(), CoreError> {
        mutation::delete_block(&mut self.graph, id)?;
        self.dirty = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_new_knowledge_base_initializes_root_and_persists_on_save() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut session = Session::open(&path).unwrap();
        let root = session.root_id();
        let child = session.create_block(Some(root)).unwrap();
        session.save().unwrap();

        let session = Session::open(&path).unwrap();
        assert_eq!(session.root_id(), root);
        assert!(session.graph().block(child).is_some());
    }

    #[test]
    fn read_only_session_does_not_mark_existing_file_dirty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut session = Session::open(&path).unwrap();
        session.save().unwrap();

        let session = Session::open(&path).unwrap();
        let _ = session.query(&format!("children:{}", session.root_id())).unwrap();
        drop(session);

        let metadata = std::fs::metadata(&path).unwrap();
        let first_modified = metadata.modified().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let session = Session::open(&path).unwrap();
        let _ = session.query(&format!("children:{}", session.root_id())).unwrap();
        drop(session);
        let metadata = std::fs::metadata(&path).unwrap();
        assert_eq!(metadata.modified().unwrap(), first_modified);
    }
}
