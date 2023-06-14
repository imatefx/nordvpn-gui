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
    pub has_update: bool,

    pub email: Option<String>,

    pub vpn_service_status: Option<String>,

    pub expires_on: Option<String>,
}

pub fn extract_email(line: String) -> Option<String> {
    println!("{}", line);

    let mut output: Option<String> = None;

    let re = Regex::new(r"Email Address: (?P<email>[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+(?:\.[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+)*@(?:[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?\.)+[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?)").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps["email"]);
        output = Some(String::from(&caps["email"]));
    }

    return output;
}

pub fn extract_vpn_service_status(line: String) -> Option<String> {
    println!("{}", line);

    let mut output: Option<String> = None;

    let re = Regex::new(r"VPN Service: (?P<status>[A-Za-z0-9]+) \((?P<expiry>[^)]*\))").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps["status"]);
        output = Some(String::from(&caps["status"]));
    }

    return output;
}

pub fn extract_expires_on(line: String) -> Option<String> {
    println!("{}", line);

    let mut output: Option<String> = None;

    let re = Regex::new(r"VPN Service: (?P<status>[A-Za-z0-9]+) \((?P<expiry>[^)]*)\)").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps["expiry"]);
        output = Some(String::from(&caps["expiry"]));
    }

    return output;
}

pub fn extract_update_status(line: String) -> bool {
    println!("{}", line);

    let mut output: bool = false;

    let re = Regex::new(r"^.*new.*version.*available.*$").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps[0]);
        output = true;
    }

    return output;
}

#[tauri::command]
pub fn get_nordvpn_account_info() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["account"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    //     let json = r#"{
    //     "hasUpdate": true,
    //     "email": "hello@hello.com",
    //     "vpnServiceStatus" : "Active",
    //     "expiresOn": "May 30th, 2024"
    // }"#;
    // let mut model: AccountInfo = serde_json::from_str(&json).unwrap();
    let mut model: AccountInfo = AccountInfo {
        has_update: false,
        email: None,
        vpn_service_status: None,
        expires_on: None,
    };

    // let return_str: String = "Hello".to_string();

    let parts = output_str.split("\n");

    for part in parts {
        println!("{}", part);

        if model.has_update == false {
            model.has_update = extract_update_status(part.to_string());
        }

        if model.email.is_none() {
            model.email = extract_email(part.to_string());
        }

        if model.vpn_service_status.is_none() {
            model.vpn_service_status = extract_vpn_service_status(part.to_string());
        }

        if model.expires_on.is_none() {
            model.expires_on = extract_expires_on(part.to_string());
        }
    }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
