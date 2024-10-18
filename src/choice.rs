use std::{collections::HashMap, fs};

use bevy::prelude::*;
use serde_json::*;

use crate::scenario::*;
use crate::tree::*;


#[derive(Resource, Default)]
pub struct Choice {
    pub choices: HashMap<String, String>,
    pub choice: usize
}


pub fn set_choices(scenario_point: Res<ScenarioPoint>, mut choice: ResMut<Choice>) {
    let res = fs::read_to_string(format!("assets/scenario/{}.json", scenario_point.tree))
        .expect("cant read file");
    let headers: Value = serde_json::from_str(&res).unwrap();
    let choices: HashMap<String, String> = serde_json::from_value(headers["choice"].clone()).unwrap();

    choice.choices = choices;
}

pub fn show_choices(mut choice: ResMut<Choice>, mut scenario_point: ResMut<ScenarioPoint>, 
    tree: Res<Tree>) {
    for key in tree.node[&scenario_point.tree].clone() {
        println!("\n{}\n", choice.choices[&key])
    }
    scenario_point.showing = false;
    choice.choice = choice.choices.len()/2;
}

pub fn wait_for_input(keys: Res<ButtonInput<KeyCode>>, mut choice: ResMut<Choice>,
    tree: Res<Tree>, mut scenario_point: ResMut<ScenarioPoint>) {
    if keys.just_pressed(KeyCode::Space) {
        if choice.choices.len()/2 > choice.choice {
            choice.choice += 1;
        }
        else {
            choice.choice = 0;
        }
    }
    if keys.just_pressed(KeyCode::Enter) {
        let tree_prnt = &tree.node[&scenario_point.tree];
        scenario_point.tree = tree_prnt[choice.choice].to_string();
        scenario_point.choice = false;
        scenario_point.showing = true;
    }
}

pub fn run_if_is_choice(scenario_point: Res<ScenarioPoint>) -> bool {
    scenario_point.choice
}

pub fn run_if_is_not_choice(scenario_point: Res<ScenarioPoint>) -> bool {
    !scenario_point.choice
}

pub fn run_if_is_showing(scenario_point: Res<ScenarioPoint>, keys: Res<ButtonInput<KeyCode>>) -> bool {
    if keys.just_pressed(KeyCode::AltLeft) {
        println!("{}", scenario_point.showing);
    }
    scenario_point.showing
}