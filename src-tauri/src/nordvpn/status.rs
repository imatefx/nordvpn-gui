use regex::Regex;
use std::process::Command;

extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub status: Option<String>,

    pub hostname: Option<String>,

    pub ip: Option<String>,

    pub country: Option<String>,

    pub city: Option<String>,

    pub current_technology: Option<String>,

    pub current_protocol: Option<String>,

    pub transfer: Option<String>,

    pub uptime: Option<String>,
}

pub fn extract_status(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Status: (?P<resVal>([A-Za-z0-9]+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_hostname(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Hostname: (?P<resVal>([A-Za-z0-9]+(\.[A-Za-z0-9]+)+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_ip(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"IP: (?P<resVal>([A-Za-z0-9]+(\.[A-Za-z0-9]+)+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_country(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Country: (?P<resVal>([A-Za-z0-9]+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_city(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"City: (?P<resVal>([A-Za-z0-9]+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_current_technology(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Current technology: (?P<resVal>([A-Za-z0-9]+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_current_protocol(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Current protocol: (?P<resVal>([A-Za-z0-9]+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_transfer(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(
        r"Transfer: (?P<resVal>([0-9]*\.[0-9]+ [A-Za-z]+ received, [0-9]*\.[0-9]+ [A-Za-z]+ sent))",
    )
    .unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

pub fn extract_uptime(line: String) -> Option<String> {
    let mut output: Option<String> = None;

    let re = Regex::new(r"Uptime: (?P<resVal>([A-Za-z0-9]+( [A-Za-z0-9]+)+))").unwrap();

    for caps in re.captures_iter(&line) {
        output = Some(String::from(&caps["resVal"]));
    }

    return output;
}

#[tauri::command]
pub fn get_nordvpn_status() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["status"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let mut model: Status = Status {
        status: None,
        hostname: None,
        ip: None,
        country: None,
        city: None,
        current_technology: None,
        current_protocol: None,
        transfer: None,
        uptime: None,
    };

    let parts = output_str.split("\n");

    for part in parts {
        if model.status.is_none() {
            model.status = extract_status(part.to_string());
        }
        if model.hostname.is_none() {
            model.hostname = extract_hostname(part.to_string());
        }
        if model.ip.is_none() {
            model.ip = extract_ip(part.to_string());
        }
        if model.country.is_none() {
            model.country = extract_country(part.to_string());
        }
        if model.city.is_none() {
            model.city = extract_city(part.to_string());
        }
        if model.current_technology.is_none() {
            model.current_technology = extract_current_technology(part.to_string());
        }
        if model.current_protocol.is_none() {
            model.current_protocol = extract_current_protocol(part.to_string());
        }
        if model.transfer.is_none() {
            model.transfer = extract_transfer(part.to_string());
        }
        if model.uptime.is_none() {
            model.uptime = extract_uptime(part.to_string());
        }
    }

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
