use bevy::prelude::*;

use self::{grid::Grid, nav::Nav};

pub mod grid;
mod nav;
mod node;
pub mod plugin;
mod shaped_container;
pub mod vertex_line;
mod vessel_spawning;

#[derive(Component)]
pub struct ReceiveNav;

#[bevy_trait_query::queryable]
pub trait NavControlled {
    fn trickle(&mut self, nav: Nav);
}
