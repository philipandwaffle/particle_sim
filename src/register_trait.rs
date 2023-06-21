use bevy::prelude::{App, Plugin};

use crate::spatial_ui::{grid::Grid, vertex_line::VertexLine, NavControlled};

pub struct RegisterTraitPlugin;
impl Plugin for RegisterTraitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn NavControlled, Grid>();
        app.register_component_as::<dyn NavControlled, VertexLine>();
    }
}
