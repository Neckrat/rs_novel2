mod characters;
mod scenario;

use bevy::prelude::*;
use scenario::*;

use characters::*;


#[derive(Component)]
struct Name(String);


fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, scenario)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((
        Character {
            name: "Анатолий".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
        Name("Анатолий".to_string())));  // может нафиг и имя, и вообще какие-то данные ввиде структур, если всё есть в json'e
}

fn scenario(query_chr: Query<&Character, With<Name>>) {
    let greetings = get_scenario_block("greetings".to_string());
    println!("{:?}", greetings);
    for chr in &query_chr {
        
    }
}

