use crate::utils::string_utils::clean_multiline_string;
use std::process::Command;

#[tauri::command]
pub fn nordvpn_disconnect() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["disconnect"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let formatted_output = clean_multiline_string(output_str);
    format!("{:#?}", formatted_output)
}
