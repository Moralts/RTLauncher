// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod api;
mod module;
mod utils;

use api::login::get_code;
use module::download::dwl_main::{get_version_manifest, dwl_version_manifest};
use module::start_game::stg_main::stg;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_code,
            get_version_manifest,
            dwl_version_manifest,
            stg
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
