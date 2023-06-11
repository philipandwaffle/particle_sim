use bevy::prelude::*;

pub enum DesignerType {
    Interaction(Entity),
    Matrix(Entity),
}
impl DesignerType {
    pub fn apply_primary_nav_delta(&self) {}
    pub fn apply_secondary_nav_delta(&self) {}
    pub fn apply_primary_interact_delta(&self) {}
    pub fn apply_secondary_secondary_delta(&self) {}
}

#[derive(Resource)]
pub struct DesignerState {
    pub designers: Vec<DesignerType>,
    pub cur_designer: usize,
}
