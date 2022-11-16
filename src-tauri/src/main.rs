#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod command;
pub mod errors;
pub mod modals;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::cmd::open_app,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
