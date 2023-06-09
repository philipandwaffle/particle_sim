use bevy::prelude::*;

#[derive(Resource)]
pub struct MatrixDesignerState {
    pub edit_point: UVec2,
    pub size: Vec3,
    pub centre: Vec3,
    pub num_particles: usize,
}
impl MatrixDesignerState {
    pub fn new(num_particles: usize, size: Vec3, centre: Vec3) -> Self {
        Self {
            edit_point: UVec2::ZERO,
            size,
            centre,
            num_particles,
        }
    }
}
