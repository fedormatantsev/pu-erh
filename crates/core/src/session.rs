use std::path::{Path, PathBuf};

use graph::{Block, KnowledgeBase};
use storage::{load, save};
use uuid::Uuid;

use crate::error::CoreError;
use crate::mutation;
use crate::query;

pub struct Session {
    kb: KnowledgeBase,
    path: PathBuf,
    dirty: bool,
}

impl Session {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CoreError> {
        let path = path.as_ref().to_path_buf();
        let existed = path.exists();
        let kb = load(&path)?;
        Ok(Self {
            kb,
            path,
            dirty: !existed,
        })
    }

    pub fn knowledge_base(&self) -> &KnowledgeBase {
        &self.kb
    }

    pub fn root_id(&self) -> Result<Uuid, CoreError> {
        Ok(self.kb.root_id()?)
    }

    pub fn save(&mut self) -> Result<(), CoreError> {
        if self.dirty {
            self.ensure_root()?;
            save(&self.path, &self.kb)?;
            self.dirty = false;
        }
        Ok(())
    }

    pub fn query(&self, expression: &str) -> Result<Vec<Block>, CoreError> {
        query::execute(&self.kb, expression)
    }

    pub fn create_block(&mut self, parent: Option<Uuid>) -> Result<Uuid, CoreError> {
        let id = mutation::create_block(&mut self.kb, parent)?;
        self.dirty = true;
        Ok(id)
    }

    pub fn move_block(&mut self, id: Uuid, new_parent: Option<Uuid>) -> Result<(), CoreError> {
        mutation::move_block(&mut self.kb, id, new_parent)?;
        self.dirty = true;
        Ok(())
    }

    pub fn delete_block(&mut self, id: Uuid) -> Result<(), CoreError> {
        mutation::delete_block(&mut self.kb, id)?;
        self.dirty = true;
        Ok(())
    }

    fn ensure_root(&mut self) -> Result<Uuid, CoreError> {
        if self.kb.is_empty() {
            let root_id = Uuid::new_v4();
            self.kb.append_root_block(root_id);
        }
        Ok(self.kb.root_id()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn open_new_knowledge_base_initializes_root_on_save() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut session = Session::open(&path).unwrap();
        session.save().unwrap();
        let root = session.root_id().unwrap();
        let child = session.create_block(Some(root)).unwrap();
        session.save().unwrap();

        let session = Session::open(&path).unwrap();
        assert_eq!(session.root_id().unwrap(), root);
        assert!(session.knowledge_base().block(child).is_some());
    }

    #[test]
    fn read_only_session_does_not_mark_existing_file_dirty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("kb.json");

        let mut session = Session::open(&path).unwrap();
        session.save().unwrap();
        let root = session.root_id().unwrap();

        let session = Session::open(&path).unwrap();
        let _ = session.query(&format!("children:{root}")).unwrap();
        drop(session);

        let metadata = std::fs::metadata(&path).unwrap();
        let first_modified = metadata.modified().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let session = Session::open(&path).unwrap();
        let _ = session.query(&format!("children:{root}")).unwrap();
        drop(session);
        let metadata = std::fs::metadata(&path).unwrap();
        assert_eq!(metadata.modified().unwrap(), first_modified);
    }
}
