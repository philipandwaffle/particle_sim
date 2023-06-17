use std::cell::RefCell;

use bevy::{
    math::{uvec2, vec3},
    prelude::*,
};
use bevy_trait_query::One;

use self::{
    grid::{update_grid_containers, GridBundle},
    root::{update_root, Dreg},
    shaped_container::update_shaped_containers,
    vertex_line::update_vertex_lines,
    vessel_spawning::VesselType,
};
use crate::spatial_ui::root::Root;

pub mod grid;
pub mod root;
mod shaped_container;
mod vertex_line;
mod vessel_spawning;

pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn)
            .add_system(update_root)
            .add_system(update_grid_containers)
            .add_system(update_vertex_lines)
            .add_system(update_shaped_containers);
    }
}

#[bevy_trait_query::queryable]
pub trait Trickles {
    fn drip(&mut self, vessels: &RefCell<Query<One<&mut dyn Trickles>>>, dreg: Dreg);
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = VesselType::Grid((vec3(0.0, 0.0, -5.0), vec3(5.0, 5.0, 1.0), uvec2(4, 4)));
    let grid_entity = grid.spawn_vessel(&mut commands, &asset_server, &mut meshes, &mut materials);
    commands.insert_resource(Root::new(grid_entity));
}
