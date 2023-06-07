use bevy::prelude::Resource;

use crate::particles::interaction_rule::interaction::InteractionRule;

#[derive(Resource)]
pub struct Matrix {
    pub data: Vec<Option<Box<dyn InteractionRule + Sync + Send>>>,
}
impl Matrix {
    pub fn new(num_particles: usize) -> Self {
        return Self {
            data: Vec::with_capacity(num_particles * num_particles),
        };
    }
}
