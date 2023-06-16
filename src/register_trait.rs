use bevy::prelude::{App, Plugin};

use crate::spatial_ui::{grid::Grid, Trickles};

pub struct RegisterTraitPlugin;
impl Plugin for RegisterTraitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn Trickles, Grid>();
    }
}
