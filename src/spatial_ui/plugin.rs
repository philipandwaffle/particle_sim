use bevy::prelude::*;
use bevy_trait_query::One;

use crate::floating_cam::controls::ControlState;

use super::{nav::Nav, NavControlled, ReceiveNav};

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
    let nav = Nav::new(
        cs.designer_primary_nav_delta,
        cs.designer_secondary_nav_delta,
        cs.designer_primary_interact,
        cs.designer_secondary_interact,
    );
    for mut receiver in receivers.iter_mut() {
        receiver.trickle(nav.clone());
    }
}
