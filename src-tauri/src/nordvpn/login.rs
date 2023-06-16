use crate::utils::string_utils::clean_multiline_string_url;
use std::process::Command;
#[tauri::command]
pub fn nordvpn_login() -> String {
    let mut cmd = Command::new("nordvpn");

    cmd.args(["login"]);
    let output = cmd.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_str);
    let formatted_output = clean_multiline_string_url(output_str);
    format!("{:#?}", formatted_output)
}

#[tauri::command]
pub fn nordvpn_login_using_token(token: String) -> String {
    let mut cmd = Command::new("nordvpn");

    cmd.args(["login", "--token", &token]);
    let output = cmd.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_str);
    let formatted_output = clean_multiline_string_url(output_str);
    format!("{:#?}", formatted_output)
}
