use bevy::prelude::Component;

use crate::floating_cam::control_state::NavDelta;

pub mod grid;
pub mod plugin;
mod shaped_container;
mod ui_spawning;
pub mod vertex_line;

#[derive(Component)]
pub struct ReceiveNav;

#[bevy_trait_query::queryable]
pub trait NavControlled {
    fn trickle(&mut self, nav: NavDelta);
}
