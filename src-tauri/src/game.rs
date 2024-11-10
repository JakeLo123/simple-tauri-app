use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use tauri::{self, AppHandle, State};
use tauri::Emitter;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct AppState {
    count: usize,
}

#[tauri::command]
pub fn inc_count(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) {
    let mut state = state.lock().unwrap();
    state.count += 1;
    _ = app_handle.emit("tick", AppState { count: state.count });
}

#[tauri::command]
pub fn dec_count(state: State<'_, Mutex<AppState>>, app_handle: AppHandle) {
    let mut state = state.lock().unwrap();
    state.count -= 1;
    _ = app_handle.emit("tick", AppState { count: state.count });
}
