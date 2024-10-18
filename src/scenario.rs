use bevy::prelude::*;
use serde_json::*;
use serde::*;

use std:: fs;

use crate::{characters::*, next_tree_node, choice::*};


// #[derive(Deserialize, Serialize, Debug)]
// pub struct Heading {
//     music: String,
//     image: String,
//     scenario: Scenario,
//     pub choice: String
// }

#[derive(Deserialize, Serialize, Debug, Resource, Default)]
struct Scenario {
    id: u8,
    name: String,
    txt: String,
    image: String,
}

#[derive(Resource, Default)]
pub struct ScenarioPoint{
    pub point: usize,
    pub tree: String,
    pub len: usize,
    pub choice: bool,
    pub showing: bool,
}

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scenario>();
        app.init_resource::<ScenarioPoint>();
        app.add_systems(Update, (
            scenario_len,
            increment_scn_point.run_if(run_if_is_not_choice),
            next_tree_node,
            (
                scenario_block,       
                scenario, 
                dialog_box
            )
            .chain()
            .run_if(run_if_is_not_choice),
            // Множество систем если кол-во ветвей в дереве равняется единице
            (
                (
                    set_choices, 
                    show_choices
                )
                .chain()
                .run_if(run_if_is_showing),
                wait_for_input
            )
            .chain()
            .run_if(run_if_is_choice)
            // Множество систем если кол-во ветвей в дереве больше единицы
        ).chain());
    }
}

// Берёт информацию из JSON'a о следующей фразе, и передаёт её Res<Scenario> 
fn scenario_block(scenario_point: Res<ScenarioPoint>, mut scenario: ResMut<Scenario>) {
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
}

pub fn scenario_len(mut scenario_point: ResMut<ScenarioPoint>) {
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

pub fn increment_scn_point(mut scenario_point: ResMut<ScenarioPoint>, keys: Res<ButtonInput<KeyCode>>) {
    if scenario_point.len > scenario_point.point && keys.just_pressed(KeyCode::Space) {
        scenario_point.point += 1;
    }
}

fn dialog_box(query_chr: Query<&Character>, scenario: Res<Scenario>, 
    keys: Res<ButtonInput<KeyCode>>, mut scenario_point: ResMut<ScenarioPoint>) {
    for chr in &query_chr {
        if scenario.id == chr.id && (keys.just_pressed(KeyCode::Space) || scenario_point.showing) {
            println!("\n\n{}\n\n", chr.image);
            println!("{}", chr.name);
            println!("---------------------------------");
            println!("{}", chr.txt);
            scenario_point.showing = false;
        }
    }
}