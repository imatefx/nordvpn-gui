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
pub fn get_nordvpn_countries() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["countries"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let mut model: Countries = vec![];

    let mut countries: Vec<&str> = [].to_vec();

    let parts: Vec<&str> = output_str.split(" \r").collect();

    let part_len = parts.len();

    if part_len == 3 {
        countries = parts[2].split(",").collect();
    } else if part_len == 2 {
        countries = parts[0].split(",").collect();
    }

    for country in countries {
        println!("{}", country);

        let val = &country.trim();

        let country: Country = Country {
            id: val.to_string(),
            name: val.to_string(),
        };

        model.push(country)
    }

    // if is_update_status(model[0].id.clone()) {
    //     model.remove(0);
    // }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
