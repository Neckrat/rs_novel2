use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub name: String,
    pub txt: String,
    pub image: String
}

#[derive(Component)]
pub struct Anatoliy;

impl Character {
    fn say(str: String) {
        println!("{}", str);
    }
}
