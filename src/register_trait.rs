use bevy::prelude::{App, Plugin};

use crate::particles::{
    designers::{designer::Designer, interaction::interaction_designer::InteractionDesigner},
    trickle_down_state::{Grid, Trickles},
};

pub struct RegisterTraitPlugin;
impl Plugin for RegisterTraitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn Trickles, Grid>();
    }
}