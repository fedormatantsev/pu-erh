use std::path::{Path, PathBuf};
use std::sync::Mutex;

use pu_erh_core::{Block, CoreError, GraphError, Properties, PropertyValue, Session};
use serde::Serialize;
use uuid::Uuid;

pub const PING_RESPONSE: &str = "pong";

/// Serializable view of a block for the frontend: id as string, the raw
/// property map, and whether the block has children (so the tree can show an
/// expand affordance before expanding).
#[derive(Debug, Serialize)]
pub struct BlockDto {
    pub id: String,
    pub properties: Properties,
    pub has_children: bool,
}

impl BlockDto {
    fn new(block: Block, has_children: bool) -> Self {
        Self {
            id: block.id.to_string(),
            properties: block.properties,
            has_children,
        }
    }
}

pub struct AppState {
    session: Mutex<Session>,
}

impl AppState {
    pub fn kb_path(base: &Path) -> PathBuf {
        base.join("pu-erh").join("kb.json")
    }

    pub fn open_at(path: PathBuf) -> Result<Self, CoreError> {
        let is_new = !path.exists();
        let mut session = Session::open(path)?;
        if is_new {
            session.save()?;
        }
        Ok(Self {
            session: Mutex::new(session),
        })
    }

    pub fn root_id(&self) -> Result<String, String> {
        let session = self.session.lock().map_err(|err| err.to_string())?;
        session
            .root_id()
            .map(|id| id.to_string())
            .map_err(|err| err.to_string())
    }

    pub fn block(&self, id: &str) -> Result<BlockDto, String> {
        let id = parse_uuid(id)?;
        let session = self.session.lock().map_err(|err| err.to_string())?;
        let kb = session.knowledge_base();
        let block = kb
            .block(id)
            .ok_or_else(|| CoreError::from(GraphError::BlockNotFound(id)).to_string())?;
        let has_children = kb.has_children(id);
        Ok(BlockDto::new(block, has_children))
    }

    pub fn parent(&self, id: &str) -> Result<Option<BlockDto>, String> {
        let id = parse_uuid(id)?;
        let session = self.session.lock().map_err(|err| err.to_string())?;
        let parents = session
            .query(&format!("parent:{id}"))
            .map_err(|err| err.to_string())?;
        let kb = session.knowledge_base();
        Ok(parents.into_iter().next().map(|block| {
            let has_children = kb.has_children(block.id);
            BlockDto::new(block, has_children)
        }))
    }

    pub fn children(&self, id: &str) -> Result<Vec<BlockDto>, String> {
        let parent = parse_uuid(id)?;
        let session = self.session.lock().map_err(|err| err.to_string())?;
        let children = session
            .query(&format!("children:{parent}"))
            .map_err(|err| err.to_string())?;
        let kb = session.knowledge_base();
        Ok(children
            .into_iter()
            .map(|block| {
                let has_children = kb.has_children(block.id);
                BlockDto::new(block, has_children)
            })
            .collect())
    }

    pub fn set_property(&self, id: &str, key: String, value: PropertyValue) -> Result<(), String> {
        let id = parse_uuid(id)?;
        let mut session = self.session.lock().map_err(|err| err.to_string())?;
        session.set_property(id, key, value).map_err(|err| err.to_string())
    }

    /// Creates a new child of `parent` through the session and returns the new
    /// block id. The append is held in memory; no save occurs here.
    pub fn create_block(&self, parent: &str) -> Result<String, String> {
        let parent = parse_uuid(parent)?;
        let mut session = self.session.lock().map_err(|err| err.to_string())?;
        session
            .create_block(Some(parent))
            .map(|id| id.to_string())
            .map_err(|err| err.to_string())
    }

    pub fn save(&self) -> Result<(), String> {
        let mut session = self.session.lock().map_err(|err| err.to_string())?;
        session.save().map_err(|err| err.to_string())
    }
}

fn parse_uuid(value: &str) -> Result<Uuid, String> {
    Uuid::parse_str(value).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn kb_path_under_app_data_dir() {
        let base = Path::new("/tmp/data");
        assert_eq!(
            AppState::kb_path(base),
            PathBuf::from("/tmp/data/pu-erh/kb.json")
        );
    }

    #[test]
    fn open_at_missing_file_bootstraps_root() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("pu-erh/kb.json");
        let state = AppState::open_at(path).expect("open");
        assert!(state.root_id().is_ok(), "root_id must succeed after bootstrap");
    }

    fn state_with_root() -> (AppState, String, tempfile::TempDir) {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("pu-erh/kb.json");
        let state = AppState::open_at(path).expect("open");
        let root = state.root_id().expect("root id");
        (state, root, dir)
    }

    #[test]
    fn open_at_existing_file_does_not_overwrite() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("pu-erh/kb.json");
        AppState::open_at(path.clone()).expect("first open bootstraps");
        let mtime_before = std::fs::metadata(&path)
            .expect("file exists")
            .modified()
            .expect("mtime");
        AppState::open_at(path.clone()).expect("second open");
        let mtime_after = std::fs::metadata(&path)
            .expect("file still exists")
            .modified()
            .expect("mtime");
        assert_eq!(mtime_before, mtime_after, "re-opening existing KB must not modify the file");
    }

    #[test]
    fn children_round_trip_reports_has_children() {
        let (state, root, _dir) = state_with_root();
        assert!(state.children(&root).expect("children").is_empty());

        // Create a child and a grandchild through the session.
        let child = {
            let mut session = state.session.lock().unwrap();
            let root_id = session.root_id().unwrap();
            let child = session.create_block(Some(root_id)).unwrap();
            session.create_block(Some(child)).unwrap();
            child.to_string()
        };

        let children = state.children(&root).expect("children");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child);
        assert!(children[0].has_children, "child has a grandchild");
    }

    #[test]
    fn parent_of_child_is_root_with_has_children() {
        let (state, root, _dir) = state_with_root();
        let child = {
            let mut session = state.session.lock().unwrap();
            let root_id = session.root_id().unwrap();
            session.create_block(Some(root_id)).unwrap().to_string()
        };

        let parent = state.parent(&child).expect("parent").expect("has a parent");
        assert_eq!(parent.id, root);
        assert!(parent.has_children, "root has the child");
    }

    #[test]
    fn parent_of_root_is_none() {
        let (state, root, _dir) = state_with_root();
        assert!(state.parent(&root).expect("parent").is_none());
    }

    #[test]
    fn parent_missing_surfaces_error() {
        let (state, _root, _dir) = state_with_root();
        let missing = uuid::Uuid::new_v4().to_string();
        let err = state.parent(&missing).unwrap_err();
        assert!(err.contains("block not found"), "got: {err}");
    }

    #[test]
    fn set_property_round_trip_visible_in_block() {
        let (state, _root, _dir) = state_with_root();
        let child = {
            let mut session = state.session.lock().unwrap();
            let root_id = session.root_id().unwrap();
            session.create_block(Some(root_id)).unwrap().to_string()
        };

        state
            .set_property(&child, "display".to_string(), PropertyValue::String("tree".to_string()))
            .expect("set_property");

        let dto = state.block(&child).expect("block");
        assert_eq!(
            dto.properties.get("display"),
            Some(&PropertyValue::String("tree".to_string()))
        );
    }

    #[test]
    fn create_block_appends_child_visible_in_children() {
        let (state, root, _dir) = state_with_root();
        let child = state.create_block(&root).expect("create child");

        let children = state.children(&root).expect("children");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, child);
    }

    #[test]
    fn create_block_nonexistent_parent_surfaces_error() {
        let (state, _root, _dir) = state_with_root();
        let missing = uuid::Uuid::new_v4().to_string();
        let err = state.create_block(&missing).unwrap_err();
        assert!(!err.is_empty(), "expected a CoreError-derived message");
    }

    #[test]
    fn block_missing_surfaces_error() {
        let (state, _root, _dir) = state_with_root();
        let missing = uuid::Uuid::new_v4().to_string();
        let err = state.block(&missing).unwrap_err();
        assert!(err.contains("block not found"), "got: {err}");
    }
}
