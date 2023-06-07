use bevy::prelude::*;

#[derive(Resource)]
pub struct MatrixDesignerState {
    pub edit_point: UVec2,
    pub scale: UVec2,
    pub centre: Vec2,
    pub num_particles: usize,
}
impl MatrixDesignerState {
    pub fn new(num_particles: usize) -> Self {
        Self {
            edit_point: UVec2::ZERO,
            scale: UVec2::ZERO,
            centre: Vec2::ZERO,
            num_particles,
        }
    }
}
