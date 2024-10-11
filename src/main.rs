mod characters;
mod scenario;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use scenario::*;

use characters::*;


#[derive(Resource, Default)]
struct ScenarioPoint{
    point: usize
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ScenarioPoint>()
        .add_systems(Startup, (setup, add_people).chain())
        .add_systems(Update, (scenario, dialog_box, increment_scn_point).chain()
            .run_if(input_just_pressed(KeyCode::Space)))
        .run();
}


fn setup(mut scenario_point: ResMut<ScenarioPoint>) {
    scenario_point.point = 0;
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Character {
            id: 0,
            name: "Анатолий".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
}

fn scenario(mut query_chr: Query<&mut Character>, scenario_point: Res<ScenarioPoint>) {
    let greetings = get_scenario_block(scenario_point.point);
    for mut chr in &mut query_chr {
        if chr.id == greetings.id {
            chr.name = greetings.name.to_string();
            chr.txt = greetings.txt.to_string();
            chr.image = greetings.image.to_string();
        }
    }
}

fn dialog_box(query_chr: Query<&Character>) {
    for chr in &query_chr {
        println!("\n\n{}\n\n", chr.image);
        println!("{}", chr.name);
        println!("---------------------------------");
        println!("{}", chr.txt);
    }
}

fn increment_scn_point(mut scenario_point: ResMut<ScenarioPoint>) {
        scenario_point.point += 1;
}