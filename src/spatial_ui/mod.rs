use bevy::prelude::*;

use self::{dreg::Dreg, grid::Grid};

mod dreg;
mod grid;
mod shaped_container;
mod vertex_line;

#[derive(Resource)]
pub struct Node {
    pub leaves: Option<Vec<Node>>,
    pub content: Entity,
    pub enabled: bool,
}
impl Node {
    pub fn new(content: Entity) -> Self {
        return Self {
            leaves: None,
            content,
            enabled: false,
        };
    }

    pub fn add_leaf(&mut self, content: Entity) {
        let leaf = Node::new(content);
        match self.leaves.as_mut() {
            Some(leaves) => leaves.push(leaf),
            None => self.leaves = Some(vec![leaf]),
        }
    }

    pub fn get_leaves(&self) -> Vec<Entity> {
        match &self.leaves {
            Some(leaves) => {
                let mut result = vec![self.content];
                for leaf in leaves {
                    result.append(&mut leaf.get_leaves());
                }
                return result;
            }
            None => return vec![self.content],
        }
    }
}

pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_nav_control);
    }
}

pub fn apply_nav_control() {}

#[bevy_trait_query::queryable]
pub trait NavControlled {
    fn trickle(&mut self, dreg: Dreg);
}
