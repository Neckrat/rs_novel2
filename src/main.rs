mod characters;
mod scenario;
mod tree;

use std::collections::HashMap;

use bevy::prelude::*;

use scenario::*;
use characters::*;
use tree::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenarioPlugin)
        .init_resource::<Tree>()
        .add_systems(Startup, (setup, add_people).chain())
        .add_systems(Update, next_tree_node)
        .run();
}


fn setup(mut scenario_point: ResMut<ScenarioPoint>, mut tree: ResMut<Tree>) {
    scenario_point.point = 0;
    scenario_point.tree = "".to_string();
    scenario_point.len = 0;

    tree.node = HashMap::from([
        ("".to_string(), "greetings1".to_string()),
        ("greetings1".to_string(), "greetings2".to_string()),
    ]);
}


