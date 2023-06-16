use std::cell::RefCell;

use bevy::prelude::*;
use bevy_trait_query::One;

use self::{
    grid::update_grid_containers,
    root::{update_root, Dreg},
    shaped_container::update_shaped_containers,
    vertex_line::update_vertex_lines,
};

pub mod grid;
pub mod root;
mod shaped_container;
mod vertex_line;

pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_grid_containers)
            .add_system(update_root)
            .add_system(update_vertex_lines)
            .add_system(update_shaped_containers);
    }
}

#[bevy_trait_query::queryable]
pub trait Trickles {
    fn drip(&mut self, vessels: &RefCell<Query<One<&mut dyn Trickles>>>, dreg: Dreg);
}

fn spawn() {
    
}
