use desktop::{AppState, PING_RESPONSE};
use tauri::State;

#[tauri::command]
pub fn ping() -> &'static str {
    PING_RESPONSE
}

#[tauri::command]
pub fn root_id(state: State<AppState>) -> Result<String, String> {
    state.root_id()
}
