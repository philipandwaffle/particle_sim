use bevy::{math::vec3, prelude::*};

use super::cell::CellBundle;

#[derive(Component)]
pub struct MatrixDesigner {
    pub cell_entities: Vec<Vec<Entity>>,
    pub edit_point: IVec2,
    pub num_particles: usize,
}
impl MatrixDesigner {
    pub fn new(
        num_particles: usize,
        translation: Vec3,
        size: Vec3,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // let mut cur_spawn = matrix_designer_state.centre
        let mut cell_scale = size / (num_particles - 1) as f32;
        cell_scale.z = 1.0;
        let centre = translation;

        let mut cells = Vec::with_capacity(num_particles);
        for i in 0..num_particles {
            let mut row = Vec::with_capacity(num_particles);
            for j in 0..num_particles {
                let id = (i * num_particles) + j;
                let translation = vec3(
                    i as f32 - (num_particles - 1) as f32 / 2.0,
                    j as f32 - (num_particles - 1) as f32 / 2.0,
                    2.0 * centre.z,
                ) - translation;

                let cell = commands
                    .spawn(CellBundle::new(
                        id,
                        translation,
                        cell_scale,
                        Color::rgb(i as f32, j as f32, 0.0),
                        meshes,
                        materials,
                    ))
                    .id();
                row.push(cell);
            }
            cells.push(row);
        }

        Self {
            cell_entities: cells,
            edit_point: IVec2::ZERO,
            num_particles,
        }
    }
}
