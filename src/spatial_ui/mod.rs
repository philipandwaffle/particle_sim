use bevy::prelude::*;

use crate::floating_cam::control_state::NavDelta;

use self::grid::Grid;

pub mod grid;
mod node;
pub mod plugin;
mod shaped_container;
pub mod vertex_line;
mod vessel_spawning;

#[derive(Component)]
pub struct ReceiveNav;

#[bevy_trait_query::queryable]
pub trait NavControlled {
    fn trickle(&mut self, nav: NavDelta);
}
