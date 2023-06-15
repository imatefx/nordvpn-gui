use regex::Regex;
use std::process::Command;

extern crate serde;

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
    let mut output: Option<String> = None;

    let re = Regex::new(r"Email Address: (?P<email>[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+(?:\.[-A-Za-z0-9!#$%&'*+/=?^_`{|}~]+)*@(?:[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?\.)+[A-Za-z0-9](?:[-A-Za-z0-9]*[A-Za-z0-9])?)").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["email"]));
    }

    return output;
}

pub fn extract_vpn_service_status(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"VPN Service: (?P<status>[A-Za-z0-9]+) \((?P<expiry>[^)]*\))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["status"]));
    }

    return output;
}

pub fn extract_expires_on(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"VPN Service: (?P<status>[A-Za-z0-9]+) \((?P<expiry>[^)]*)\)").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["expiry"]));
    }

    return output;
}

pub fn extract_update_status(line: String) -> bool {
    let mut output: bool = false;

    let re = Regex::new(r"^.*new.*version.*available.*$").unwrap();

    for _caps in re.captures_iter(&line) {
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

    let mut model: AccountInfo = AccountInfo {
        has_update: false,
        email: None,
        vpn_service_status: None,
        expires_on: None,
    };

    let parts = output_str.split("\n");

    for part in parts {
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

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
