use bevy::prelude::*;
use serde_json::*;
use serde::*;

use std::fs;

use crate::{characters::*, Tree};


#[derive(Deserialize, Serialize, Debug)]
struct Heading {
    music: String,
    image: String,
    scenario: Scenario,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scenario {
    pub id: u8,
    pub name: String,
    pub txt: String,
    pub image: String
}

#[derive(Resource, Default)]
pub struct ScenarioPoint{
    pub point: usize,
    pub tree: String
}


pub fn get_scenario_block(block: usize) -> Scenario {
    let res = fs::read_to_string("assets/scenario/greetings1.json").expect("cant read file");
    
    let headers:Value = serde_json::from_str(&res).unwrap();
    let music: String = headers["music"].to_string();
    let image: String = headers["image"].to_string();

    let scn_point: Vec<Scenario> = serde_json::from_value(headers["scenario"].clone()).unwrap();

    let scn_block = &scn_point[block];
    let serialized_scn_block = serde_json::to_string(&scn_block).unwrap();

    return serde_json::from_str(&serialized_scn_block).unwrap();
}

pub fn get_scenario_len() -> usize {
    let res = fs::read_to_string("assets/scenario/greetings1.json").expect("cant read file");
    
    let headers:Value = serde_json::from_str(&res).unwrap();
    let scn_point: Vec<Scenario> = serde_json::from_value(headers["scenario"].clone()).unwrap();

    return scn_point.len();
}

pub fn scenario(mut query_chr: Query<&mut Character>, scenario_point: Res<ScenarioPoint>) {
    let block = get_scenario_block(scenario_point.point);
    for mut chr in &mut query_chr {
        if chr.id == block.id {
            chr.name = block.name.to_string();
            chr.txt = block.txt.to_string();
            chr.image = block.image.to_string();
        }
    }
}

pub fn increment_scn_point(mut scenario_point: ResMut<ScenarioPoint>, tree: Res<Tree>) {
    if get_scenario_len() > scenario_point.point {
        scenario_point.point += 1;
    }
    else {
        let tree_prnt = &tree.node[""];
        scenario_point.tree = tree_prnt.to_string();
    }
}