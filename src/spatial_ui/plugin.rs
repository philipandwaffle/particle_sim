use bevy::prelude::*;
use bevy_trait_query::One;

use crate::floating_cam::control_state::ControlState;

use super::{NavControlled, ReceiveNav};

pub struct SpatialUIPlugin;
impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_nav_control);
    }
}

pub fn apply_nav_control(
    mut receivers: Query<One<&mut dyn NavControlled>, With<ReceiveNav>>,
    mut cs: ResMut<ControlState>,
) {
    for mut receiver in receivers.iter_mut() {
        receiver.trickle(cs.nd.clone());
    }
}
