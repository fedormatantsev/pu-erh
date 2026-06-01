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
pub fn children_ordered(state: State<AppState>, id: String) -> Result<Vec<BlockDto>, String> {
    state.children_ordered(&id)
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
pub fn create_block(
    state: State<AppState>,
    parent: String,
    after: Option<String>,
) -> Result<String, String> {
    state.create_block(&parent, after.as_deref())
}

#[tauri::command]
pub fn delete_block(state: State<AppState>, id: String) -> Result<(), String> {
    state.delete_block(&id)
}

#[tauri::command]
pub fn move_block(
    state: State<AppState>,
    id: String,
    after: Option<String>,
) -> Result<(), String> {
    state.move_block(&id, after.as_deref())
}

#[tauri::command]
pub fn remove_property(
    state: State<AppState>,
    id: String,
    key: String,
) -> Result<(), String> {
    state.remove_property(&id, key)
}

#[tauri::command]
pub fn save(state: State<AppState>) -> Result<(), String> {
    state.save()
}
