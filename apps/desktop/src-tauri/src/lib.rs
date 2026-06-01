mod commands;

use commands::{
    block, children, children_ordered, create_block, delete_block, move_block, parent, ping,
    root_id, save, set_property,
};
use desktop::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");
            let kb_path = AppState::kb_path(&app_data_dir);
            if let Some(parent) = kb_path.parent() {
                std::fs::create_dir_all(parent).expect("failed to create app data directory");
            }
            let state = AppState::open_at(kb_path).expect("failed to open session");
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ping,
            root_id,
            block,
            parent,
            children,
            children_ordered,
            create_block,
            delete_block,
            move_block,
            set_property,
            save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
