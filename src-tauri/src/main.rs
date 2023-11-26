// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use spacetraders::helpers::agent::*;
use spacetraders::helpers::contracts::*;
use spacetraders::helpers::mission::*;
use spacetraders::helpers::ships::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_user_agent,
            get_user_contracts,
            get_user_ships,
            get_waypoints,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
