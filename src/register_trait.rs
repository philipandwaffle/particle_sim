use bevy::prelude::{App, Plugin};

use crate::{
    spatial_ui::{Grid as OtherGrid, NavControlled},
    spatial_ui_broken::{grid::Grid, vertex_line::VertexLine, Trickles},
};

pub struct RegisterTraitPlugin;
impl Plugin for RegisterTraitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn Trickles, Grid>();
        app.register_component_as::<dyn Trickles, VertexLine>();
        app.register_component_as::<dyn NavControlled, OtherGrid>();
    }
}
