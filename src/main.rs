mod characters;
mod scenario;
mod tree;
mod choice;

use std::collections::HashMap;

use bevy::prelude::*;

use choice::*;
use scenario::*;
use characters::*;
use tree::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenarioPlugin)
        .init_resource::<Tree>()
        .init_resource::<Choice>()
        .add_systems(Startup, (setup, add_people, next_tree_node).chain())
        // .add_systems(Update, next_tree_node.before(increment_scn_point))
        .run();
}


fn setup(mut scenario_point: ResMut<ScenarioPoint>, mut tree: ResMut<Tree>, mut choice: ResMut<Choice>) {
    scenario_point.point = 0;
    scenario_point.tree = "".to_string();
    scenario_point.len = 0;
    scenario_point.choice = false;
    scenario_point.showing = true;

    tree.node = HashMap::from([
        ("".to_string(), vec!["greetings1".to_string()]),
        ("greetings1".to_string(), vec!["greetings2".to_string()]),
        ("greetings2".to_string(), vec!["greetings3".to_string(), "greetings4".to_string()]),
    ]);

    choice.choice = 0;
}


