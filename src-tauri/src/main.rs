// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod nordvpn;
use crate::nordvpn::account_info::get_nordvpn_account_info;
use crate::nordvpn::cities::get_nordvpn_cities;
use crate::nordvpn::connect::nordvpn_connect_random;
use crate::nordvpn::countries::get_nordvpn_countries;
use crate::nordvpn::disconnect::nordvpn_disconnect;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_nordvpn_account_info,
            get_nordvpn_countries,
            get_nordvpn_cities,
            nordvpn_connect_random,
            nordvpn_disconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
