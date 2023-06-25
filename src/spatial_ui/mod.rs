use bevy::prelude::Component;

use crate::floating_cam::control_state::NavDelta;

pub mod grid;
pub mod plugin;
mod scale;
mod shaped_container;
mod ui_spawning;
pub mod vertex_line;

// Marks an entity that can receive nav control
#[derive(Component)]
pub struct ReceiveNav;

// Implement this to allow it to process nav control
#[bevy_trait_query::queryable]
pub trait NavControlled {
    fn trickle(&mut self, nav: NavDelta);
}
