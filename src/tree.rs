use std::collections::HashMap;

use bevy::prelude::*;

use crate:: ScenarioPoint;

#[derive(Resource, Default)]
pub struct Tree {
    pub node: HashMap<String, String>
}

impl Tree {
    pub fn insert() {
        todo!()
    }
}

pub fn next_tree_node(tree: Res<Tree>, mut scenario_point: ResMut<ScenarioPoint>) {
    if scenario_point.len <= scenario_point.point {
        let tree_prnt = &tree.node[&scenario_point.tree];
        scenario_point.tree = tree_prnt.to_string();
        scenario_point.point = 0;
    }
}