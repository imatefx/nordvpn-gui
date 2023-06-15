use regex::Regex;
use std::process::Command;

// Example code that deserializes and serializes the model.
extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::countries;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: countries = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

pub type Countries = Vec<Country>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub id: String,

    pub name: String,
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

pub fn is_update_status(line: String) -> bool {
    println!("here  {}", line);

    let mut output: bool = false;

    let re = Regex::new(r"^.*new.*version.*available.*$").unwrap();

    for caps in re.captures_iter(&line) {
        println!("{:?}", &caps[0]);
        output = true;
    }

    return output;
}

#[tauri::command]
pub fn nordvpn_connect_random() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["connect"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    //     let json = r#"{
    //     "hasUpdate": true,
    //     "email": "hello@hello.com",
    //     "vpnServiceStatus" : "Active",
    //     "expiresOn": "May 30th, 2024"
    // }"#;
    // let mut model: AccountInfo = serde_json::from_str(&json).unwrap();
    let mut model: Countries = vec![];

    // let return_str: String = "Hello".to_string();

    let parts = output_str.split(",");

    for part in parts {
        println!("{}", part);

        let val = &part.trim();

        let country: Country = Country {
            id: val.to_string(),
            name: val.to_string(),
        };

        model.push(country)

        // if model.email.is_none() {
        //     model.email = extract_email(part.to_string());
        // }
    }

    if is_update_status(model[0].id.clone()) {
        model.remove(0);
    }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", output_str)
}
