mod characters;
mod scenario;
mod tree;

use std::collections::HashMap;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use scenario::*;
use characters::*;
use tree::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ScenarioPoint>()
        .init_resource::<Tree>()
        .add_systems(Startup, (setup, add_people).chain())
        .add_systems(Update, (scenario, dialog_box, increment_scn_point).chain()
            .run_if(input_just_pressed(KeyCode::Space)))
        .run();
}


fn setup(mut scenario_point: ResMut<ScenarioPoint>, mut tree: ResMut<Tree>) {
    scenario_point.point = 0;
    scenario_point.tree = "".to_string();

    tree.node = HashMap::from([
        ("".to_string(), "greetings1".to_string()),
        ("greetings1".to_string(), "greetings2".to_string()),
    ]);
}

fn dialog_box(query_chr: Query<&Character>, scenario_point: Res<ScenarioPoint>) {
    for chr in &query_chr {
        if get_scenario_block(scenario_point.point).id == chr.id {
            println!("\n\n{}\n\n", chr.image);
            println!("{}", chr.name);
            println!("---------------------------------");
            println!("{}", chr.txt);
        }
    }
}
