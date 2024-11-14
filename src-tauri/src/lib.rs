mod game;

use std::sync::Mutex;
use tauri::Manager;

use game::entities::GameOfLife;
use game::functions::{get_board, reset_game, start_game, tick, toggle_cell};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(GameOfLife::new(20, 20)));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_board,
            toggle_cell,
            start_game,
            tick,
            reset_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
