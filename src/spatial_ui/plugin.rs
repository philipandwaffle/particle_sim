use bevy::{
    math::{uvec2, vec3},
    prelude::*,
};
use bevy_trait_query::One;

use crate::floating_cam::control_state::ControlState;

use super::{
    ui_spawning::{spawn_ui, SpawnList, UIType},
    NavControlled, ReceiveNav,
};

pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnList {
            spawn: vec![UIType::Grid {
                translation: vec3(0.0, 0.0, -2.5),
                scale: vec3(5.0, 5.0, 1.0),
                dims: uvec2(5, 5),
            }],
        })
        .add_startup_system(spawn_ui)
        .add_system(apply_nav_control);
    }
}

pub fn apply_nav_control(
    mut receivers: Query<One<&mut dyn NavControlled>, With<ReceiveNav>>,
    mut cs: ResMut<ControlState>,
) {
    for mut receiver in receivers.iter_mut() {
        receiver.trickle(cs.nd.clone());
    }
    cs.nd.reset();
}
