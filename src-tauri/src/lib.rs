mod game;

use game::{inc_count, dec_count, AppState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            inc_count,
            dec_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
