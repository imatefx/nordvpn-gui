// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod nordvpn;
pub mod utils;
use crate::nordvpn::account_info::get_nordvpn_account_info;
use crate::nordvpn::cities::get_nordvpn_cities;
use crate::nordvpn::cli::nordvpn_cli_path;
use crate::nordvpn::connect::nordvpn_connect_city;
use crate::nordvpn::connect::nordvpn_connect_country;
use crate::nordvpn::connect::nordvpn_connect_group;
use crate::nordvpn::connect::nordvpn_connect_random;
use crate::nordvpn::countries::get_nordvpn_countries;
use crate::nordvpn::disconnect::nordvpn_disconnect;
use crate::nordvpn::groups::get_nordvpn_groups;
use crate::nordvpn::login::nordvpn_login;
use crate::nordvpn::login::nordvpn_login_using_token;
use crate::nordvpn::logout::nordvpn_logout;
use crate::nordvpn::status::get_nordvpn_status;
use crate::nordvpn::version::nordvpn_version;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_nordvpn_account_info,
            get_nordvpn_countries,
            get_nordvpn_cities,
            get_nordvpn_groups,
            get_nordvpn_status,
            nordvpn_connect_random,
            nordvpn_connect_country,
            nordvpn_connect_city,
            nordvpn_connect_group,
            nordvpn_disconnect,
            nordvpn_cli_path,
            nordvpn_login,
            nordvpn_login_using_token,
            nordvpn_logout,
            nordvpn_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
