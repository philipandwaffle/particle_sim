use bevy::{
    math::{uvec2, vec3},
    prelude::{App, Plugin, Query, ResMut, With},
};
use bevy_trait_query::One;

use super::{
    grid::update_grid_containers,
    scale::Scale,
    shaped_container::update_shaped_containers,
    ui_spawning::{spawn_ui, SpawnList, UIElement},
    vertex_line::update_vertex_lines,
    NavControlled, ReceiveNav,
};
use crate::floating_cam::control_state::ControlState;

// Sets up the spatial UI
pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnList {
            spawn: vec![UIElement::Grid {
                controllable: true,
                translation: vec3(0.0, 0.0, -10.0),
                scale: vec3(20.0, 20.0, 3.0),
                scale_meta: Scale::new(10, 0.01, 0.01, 0.025, 0.01),
                dims: uvec2(5, 5),
            }],
        })
        .add_startup_system(spawn_ui)
        .add_system(update_grid_containers)
        .add_system(update_shaped_containers)
        .add_system(update_vertex_lines)
        .add_system(apply_nav_control);
    }
}

// Applies control state to each receiver
pub fn apply_nav_control(
    mut receivers: Query<One<&mut dyn NavControlled>, With<ReceiveNav>>,
    mut cs: ResMut<ControlState>,
) {
    // loop through each receiver
    for mut receiver in receivers.iter_mut() {
        // Pass nav delta
        receiver.trickle(cs.nd.clone());
    }
    // Reset nav delta
    cs.nd.reset();
}
