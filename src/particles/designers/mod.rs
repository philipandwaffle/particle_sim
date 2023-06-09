use std::ops::Deref;

use bevy::prelude::*;
use serde::__private::de;

use crate::floating_cam::controls::ControlState;
mod interaction;

pub struct DesignerPlugin;
impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DesignerStates::new())
            .add_system(spawn_designers)
            .add_system(update_designer);
    }
}

pub trait Designer {
    fn apply_primary_nav_delta(&mut self, delta: Vec2);
    fn apply_secondary_nav_delta(&self, delta: isize);
    fn apply_primary_interact(&self, interact: bool);
    fn apply_secondary_interact(&self, interact: bool);
    fn spawn(&self, commands: &mut Commands);
    fn despawn(&self, commands: &mut Commands);
}

#[derive(Resource)]
pub struct DesignerStates {
    pub designers: Vec<Box<dyn Designer + Send + Sync>>,
    pub cur_designer: isize,
}
impl DesignerStates {
    pub fn new() -> Self {
        return Self {
            designers: vec![],
            cur_designer: -1,
        };
    }
    pub fn update_state(&mut self, cs: &mut ControlState) {
        let i = self.cur_designer;
        if i == -1 {
            return;
        }

        let designer = &mut self.designers[self.cur_designer as usize];
        designer.apply_primary_nav_delta(cs.designer_primary_nav_delta);
        designer.apply_secondary_nav_delta(cs.designer_secondary_nav_delta);
        designer.apply_primary_interact(cs.designer_primary_interact);
        designer.apply_secondary_interact(cs.designer_secondary_interact);
    }

    pub fn spawn_designers(&self, commands: &mut Commands) {}
}

fn update_designer(
    mut designer_states: ResMut<DesignerStates>,
    mut control_state: ResMut<ControlState>,
) {
    designer_states.update_state(&mut control_state);
    control_state.reset_designer();
}

fn spawn_designers(mut designer_states: ResMut<DesignerStates>, mut commands: Commands) {
    for designer in designer_states.designers.iter_mut() {
        designer.spawn(&mut commands);
    }
}
