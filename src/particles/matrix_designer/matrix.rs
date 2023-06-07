use bevy::prelude::{Resource, UVec2};

use crate::particles::interaction_rule::interaction::InteractionRule;

#[derive(Resource)]
pub struct Matrix {
    pub data: Vec<Vec<Option<Box<dyn InteractionRule + Sync + Send>>>>,
}
impl Matrix {
    pub fn new(num_particles: usize) -> Self {
        let mut data = Vec::with_capacity(num_particles);
        for i in 0..num_particles {
            data.push(Vec::with_capacity(num_particles));
            for _ in 0..num_particles {
                data[i].push(None);
            }
        }

        return Self { data };
    }
}
impl Matrix {
    pub fn set_cell(
        &mut self,
        rule: Option<Box<dyn InteractionRule + Sync + Send>>,
        cell_coords: UVec2,
    ) {
        self.data[cell_coords.y as usize][cell_coords.x as usize] = rule;
    }
}
