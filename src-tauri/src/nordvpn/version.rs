use crate::utils::string_utils::clean_multiline_string;
use std::process::Command;

#[tauri::command]
pub fn nordvpn_version() -> String {
    let mut cmd = Command::new("nordvpn");

    cmd.args(["version"]);
    let output = cmd.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_str);
    let formatted_output = clean_multiline_string(output_str);

    format!("{:#?}", formatted_output)
}
