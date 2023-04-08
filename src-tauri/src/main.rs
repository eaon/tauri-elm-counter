// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, Submenu};

#[tauri::command]
fn increment(current: i64) -> i64 {
    current + 1
}

#[tauri::command]
fn decrement(current: i64) -> i64 {
    current - 1
}

#[tauri::command]
fn reset() -> i64 {
    0
}

fn main() {
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "exit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![increment, decrement, reset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
