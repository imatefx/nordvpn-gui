use std::process::Command;

extern crate serde;

use serde::{Deserialize, Serialize};

pub type Cities = Vec<City>;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub id: String,

    pub name: String,
}

#[tauri::command]
pub fn get_nordvpn_cities(country: String) -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["cities", &country]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();
    let mut model: Cities = vec![];

    let mut cities: Vec<&str> = [].to_vec();

    let parts: Vec<&str> = output_str.split(" \r").collect();

    let part_len = parts.len();

    if part_len == 3 {
        cities = parts[2].split(",").collect();
    } else if part_len == 2 {
        cities = parts[0].split(",").collect();
    }

    for city in cities {
        let val = &city.trim();

        let city: City = City {
            id: val.to_string(),
            name: val.to_string(),
        };

        model.push(city)
    }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
