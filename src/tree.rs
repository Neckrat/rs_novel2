use std::collections::HashMap;

use bevy::prelude::*;

use crate:: ScenarioPoint;


#[derive(Resource, Default)]
pub struct Tree {
    pub node: HashMap<String, Vec<String>>
}

// impl Tree {
//     pub fn insert() {
//         todo!()
//     }
// }


pub fn next_tree_node(tree: Res<Tree>, mut scenario_point: ResMut<ScenarioPoint>) {
    if scenario_point.len <= scenario_point.point {
        if tree.node[&scenario_point.tree].len() > 1 {  // Если количество ветвей > одного
            scenario_point.choice = true;
            scenario_point.showing = true;
            scenario_point.point = 0;
        }
        else {
            let tree_prnt = &tree.node[&scenario_point.tree];
            scenario_point.tree = tree_prnt[0].to_string();
            scenario_point.point = 0;
        }
    }
}