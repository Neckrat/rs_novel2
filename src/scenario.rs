use serde::*;

use std::fs;


#[derive(Deserialize, Serialize, Debug)]
pub struct Scenario {
    pub id: u8,
    pub name: String,
    pub txt: String,
    pub image: String
}


pub fn get_scenario_block(block: usize) -> Scenario {
    let res = fs::read_to_string("src/scenario.json").expect("cant read file");
    
    let scn_point:Vec<Scenario> = serde_json::from_str(&res).unwrap();

    // let scenario: Value = serde_json::from_str(&res).unwrap();
    let scn_block = &scn_point[block];
    let serialized_scn_block = serde_json::to_string(&scn_block).unwrap();

    return serde_json::from_str(&serialized_scn_block).unwrap();
}