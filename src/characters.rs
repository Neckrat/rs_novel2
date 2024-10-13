use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub id: u8,
    pub name: String,
    pub txt: String,
    pub image: String
}


pub fn add_people(mut commands: Commands) {
    commands.spawn((
        Character {
            id: 0,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
    commands.spawn((
        Character {
            id: 1,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
    commands.spawn((
        Character {
            id: 2,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
    commands.spawn((
        Character {
            id: 3,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
    commands.spawn((
        Character {
            id: 4,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));
    commands.spawn((
        Character {
            id: 5,
            name: "".to_string(),
            txt: "".to_string(), 
            image: "".to_string()
        }, 
    ));  // на экране может быть до 5 сущностей, а для первого лица выделен нулевой слот
}
