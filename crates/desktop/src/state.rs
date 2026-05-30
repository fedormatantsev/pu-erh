use std::path::{Path, PathBuf};
use std::sync::Mutex;

use pu_erh_core::{CoreError, Session};

pub const PING_RESPONSE: &str = "pong";

pub struct AppState {
    session: Mutex<Session>,
}

impl AppState {
    pub fn kb_path(base: &Path) -> PathBuf {
        base.join("pu-erh").join("kb.json")
    }

    pub fn open_at(path: PathBuf) -> Result<Self, CoreError> {
        Ok(Self {
            session: Mutex::new(Session::open(path)?),
        })
    }

    pub fn root_id(&self) -> Result<String, String> {
        let session = self.session.lock().map_err(|err| err.to_string())?;
        session
            .root_id()
            .map(|id| id.to_string())
            .map_err(|err| err.to_string())
    }
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
    fn open_at_missing_file_creates_empty_session() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("pu-erh/kb.json");
        let state = AppState::open_at(path.clone()).expect("open");
        assert!(state.root_id().is_err());
    }
}
