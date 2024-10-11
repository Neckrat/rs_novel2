use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub id: u8,
    pub name: String,
    pub txt: String,
    pub image: String
}

