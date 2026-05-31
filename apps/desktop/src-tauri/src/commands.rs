use desktop::{AppState, BlockDto, PropertyValue, PING_RESPONSE};
use tauri::State;

#[tauri::command]
pub fn ping() -> &'static str {
    PING_RESPONSE
}

#[tauri::command]
pub fn root_id(state: State<AppState>) -> Result<String, String> {
    state.root_id()
}

#[tauri::command]
pub fn block(state: State<AppState>, id: String) -> Result<BlockDto, String> {
    state.block(&id)
}

#[tauri::command]
pub fn parent(state: State<AppState>, id: String) -> Result<Option<BlockDto>, String> {
    state.parent(&id)
}

#[tauri::command]
pub fn children(state: State<AppState>, id: String) -> Result<Vec<BlockDto>, String> {
    state.children(&id)
}

#[tauri::command]
pub fn set_property(
    state: State<AppState>,
    id: String,
    key: String,
    value: PropertyValue,
) -> Result<(), String> {
    state.set_property(&id, key, value)
}

#[tauri::command]
pub fn save(state: State<AppState>) -> Result<(), String> {
    state.save()
}
