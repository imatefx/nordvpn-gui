use std::process::Command;

extern crate serde;

use serde::{Deserialize, Serialize};

pub type Groups = Vec<Group>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,

    pub name: String,
}

#[tauri::command]
pub fn get_nordvpn_groups() -> String {
    let mut nord_vpn_cli = Command::new("nordvpn");

    nord_vpn_cli.args(["groups"]);
    let nord_vpn_cli_account_output = nord_vpn_cli.output().expect("failed to execute process");
    let output_str: String = String::from_utf8(nord_vpn_cli_account_output.stdout).unwrap();

    let mut model: Groups = vec![];

    let mut groups: Vec<&str> = [].to_vec();

    let parts: Vec<&str> = output_str.split(" \r").collect();

    let part_len = parts.len();

    if part_len == 3 {
        groups = parts[2].split(",").collect();
    } else if part_len == 2 {
        groups = parts[0].split(",").collect();
    }

    for group in groups {
        let val = &group.trim();

        let group: Group = Group {
            id: val.to_string(),
            name: val.to_string(),
        };

        model.push(group)
    }

    let json_stringify = serde_json::to_string(&model).unwrap();

    format!("{:#?}", json_stringify)
}
