use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use serde_json::*;
use serde::*;

use std::fs;

use crate::characters::*;


#[derive(Deserialize, Serialize, Debug)]
struct Heading {
    music: String,
    image: String,
    scenario: Scenario,
}

#[derive(Deserialize, Serialize, Debug, Resource, Default)]
pub struct Scenario {
    pub id: u8,
    pub name: String,
    pub txt: String,
    pub image: String
}

#[derive(Resource, Default)]
pub struct ScenarioPoint{
    pub point: usize,
    pub tree: String,
    pub len: usize
}

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scenario>();
        app.init_resource::<ScenarioPoint>();
        app.add_systems(Update, scenario_len);
        app.add_systems(Update, (scenario_block, scenario, increment_scn_point, dialog_box).chain()
            .run_if(input_just_pressed(KeyCode::Space)));
    }
}


fn scenario_block(mut scenario_point: ResMut<ScenarioPoint>, mut scenario: ResMut<Scenario>) {
    let res = fs::read_to_string(format!("assets/scenario/{}.json", scenario_point.tree))
        .expect("cant read file");
    
    let headers:Value = serde_json::from_str(&res).unwrap();
    let music: String = headers["music"].to_string();
    let image: String = headers["image"].to_string();

    let scn_point: Vec<Scenario> = serde_json::from_value(headers["scenario"].clone()).unwrap();

    let scn_block = &scn_point[scenario_point.point];
    let serialized_scn_block = serde_json::to_string(&scn_block).unwrap();

    let scn: Scenario = serde_json::from_str(&serialized_scn_block).unwrap();

    scenario.id = scn.id;
    scenario.name = scn.name;
    scenario.txt = scn.txt;
    scenario.image = scn.image;

    scenario_point.len = scn_point.len();
}

fn scenario_len(mut scenario_point: ResMut<ScenarioPoint>) {
    let res = fs::read_to_string(format!("assets/scenario/{}.json", scenario_point.tree))
        .expect("cant read file");

    let headers:Value = serde_json::from_str(&res).unwrap();
    let scn_point: Vec<Scenario> = serde_json::from_value(headers["scenario"].clone()).unwrap();

    scenario_point.len = scn_point.len();
}

fn scenario(mut query_chr: Query<&mut Character>, scenario: Res<Scenario>) {
    for mut chr in &mut query_chr {
        if chr.id == scenario.id {
            chr.name = scenario.name.to_string();
            chr.txt = scenario.txt.to_string();
            chr.image = scenario.image.to_string();
        }
    }
}

fn increment_scn_point(mut scenario_point: ResMut<ScenarioPoint>) {
    if scenario_point.len > scenario_point.point {
        scenario_point.point += 1;
    }
}

fn dialog_box(query_chr: Query<&Character>, scenario: Res<Scenario>) {
    for chr in &query_chr {
        if scenario.id == chr.id {
            println!("\n\n{}\n\n", chr.image);
            println!("{}", chr.name);
            println!("---------------------------------");
            println!("{}", chr.txt);
        }
    }
}