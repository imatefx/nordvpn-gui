use regex::Regex;
use std::process::Command;

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::account_info;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: account_info = serde_json::from_str(&json).unwrap();
// }

extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub has_update: Option<bool>,

    pub email: Option<String>,

    pub vpn_service_status: Option<String>,

    pub expires_on: Option<String>,
}

pub fn extract_email(line: String) -> String {
    println!("{}", line);

    let mut output = "".to_string();

    let re = Regex::new(r"Email Address: (?P<mailid>[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+(?:\.[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+)*@(?:[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?\.)+[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?)").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps["mailid"]);
        output = String::from(&caps["mailid"]);
    }

    return output;
}

#[tauri::command]
pub fn get_nordvpn_account_info() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["account"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let json = r#"{
    "hasUpdate": true,
    "email": "hello@hello.com",
    "vpnServiceStatus" : "Active",
    "expiresOn": "May 30th, 2024"
}"#;
    let mut model: AccountInfo = serde_json::from_str(&json).unwrap();

    // let return_str: String = "Hello".to_string();

    let parts = output_str.split("\n");

    for part in parts {
        model.email = Some(extract_email(part.to_string()));

        // for caps in re.captures_iter(part) {
        //     println!("{:?}", &caps["mailid"]);
        //     model.email = Some(String::from(&caps["mailid"]));
        // }
    }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    let j = serde_json::to_string(&model);

    format!("mailid: {:#?}", j.unwrap())
}
