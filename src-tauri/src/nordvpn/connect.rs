use std::process::Command;

use crate::utils::string_utils::clean_multiline_string;

#[tauri::command]
pub fn nordvpn_connect_random() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["connect"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let formatted_output = clean_multiline_string(output_str);

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", formatted_output)
}

#[tauri::command]
pub fn nordvpn_connect_country(country: String) -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["connect", &country]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let json_stringify = serde_json::to_string(&model).unwrap();
    let formatted_output = clean_multiline_string(output_str);
    format!("{:#?}", formatted_output)
}

#[tauri::command]
pub fn nordvpn_connect_city(country: String, city: String) -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["connect", &country, &city]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let json_stringify = serde_json::to_string(&model).unwrap();
    let formatted_output = clean_multiline_string(output_str);
    format!("{:#?}", formatted_output)
}

#[tauri::command]
pub fn nordvpn_connect_group(group: String) -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["connect", &group]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let json_stringify = serde_json::to_string(&model).unwrap();
    let formatted_output = clean_multiline_string(output_str);
    format!("{:#?}", formatted_output)
}
