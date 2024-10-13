use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Tree {
    pub node: HashMap<String, String>
}

impl Tree {
    pub fn insert() {
        todo!()
    }
}