use serde::*;
use serde_json::*;

use std::fs;


#[derive(Deserialize, Serialize, Debug)]
pub struct Scenario {
    name: String,
    txt: String,
    image: String
}


pub fn get_scenario_block(block: String) -> Scenario {
    let res = fs::read_to_string("src/scenario.json").expect("cant read file");
    let scenario: Value = serde_json::from_str(&res).unwrap();
    let scn_block = scenario[block].to_string();

    return serde_json::from_str(&scn_block.to_string()).unwrap();
}