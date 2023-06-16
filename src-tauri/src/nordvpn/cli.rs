use std::process::Command;

#[tauri::command]
pub fn nordvpn_cli_path() -> String {
    let mut cmd = Command::new("which");

    cmd.args(["nordvpn"]);
    let output = cmd.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_str);
    format!("{:#?}", output_str.replace("\n", ""))
}
