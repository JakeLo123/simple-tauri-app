use std::{sync::Mutex, thread, time};
use tauri::{self, AppHandle, Emitter, State};

use super::entities::{Board, Coordinates, GameOfLife};

#[tauri::command]
pub fn get_board(state: State<'_, Mutex<GameOfLife>>) -> Board {
    state.lock().unwrap().board.clone()
}

#[tauri::command]
pub fn toggle_cell(
    coordinates: Coordinates,
    state: State<'_, Mutex<GameOfLife>>,
    app_handle: AppHandle,
) {
    let mut state = state.lock().unwrap();

    state.board.toggle_cell(coordinates);
    let _ = app_handle.emit("tick", &state.board);
}

#[tauri::command]
pub fn tick(state: State<'_, Mutex<GameOfLife>>, app_handle: AppHandle) {
    let mut state = state.lock().unwrap();
    state.board.tick();
    let _ = app_handle.emit("tick", &state.board);
}

#[tauri::command]
pub fn reset_game(state: State<'_, Mutex<GameOfLife>>, app_handle: AppHandle) {
    let mut state = state.lock().unwrap();
    state.board = Board::new(20, 20);
    let _ = app_handle.emit("tick", &state.board);
}

#[tauri::command]
pub fn start_game(state: State<'_, Mutex<GameOfLife>>, app_handle: AppHandle) {
    let mut state = state.lock().unwrap();
    let interval = time::Duration::from_millis(500);

    loop {
        state.board.tick();
        let _ = app_handle.emit("tick", &state.board);
        thread::sleep(interval);
    }
}
