use bevy::{math::vec3, prelude::*};

use crate::particles::designers::interaction::interaction_designer::InteractionDesigner;

use super::cell::CellBundle;

#[derive(Component)]
pub struct MatrixDesigner {
    pub cell_entities: Vec<Vec<Entity>>,
    pub cell_values: Vec<Vec<Entity>>,
    pub cur_edit_point: IVec2,
    pub prev_edit_point: IVec2,
    pub num_particles: usize,
    pub prev_delta: Vec2,
    pub edit: bool,
}
impl MatrixDesigner {
    pub fn new(
        num_particles: usize,
        translation: Vec3,
        scale: Vec3,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // let mut cur_spawn = matrix_designer_state.centre
        let mut cell_scale = scale / num_particles as f32;
        // let centre = translation;

        let mut cells = Vec::with_capacity(num_particles);
        let mut values = Vec::with_capacity(num_particles);
        let cell_offset = (translation + (scale / 2.0)) - (cell_scale / 2.0);
        println!("Scale {}", scale.clone());
        for i in 0..num_particles {
            let mut cell_row = Vec::with_capacity(num_particles);
            let mut value_row = Vec::with_capacity(num_particles);
            for j in 0..num_particles {
                let id = (i * num_particles) + j;
                let j = num_particles - (1 + j);
                let cell_translation = vec3(
                    scale.x * (i as f32 / num_particles as f32),
                    scale.y * (j as f32 / num_particles as f32) as f32,
                    cell_offset.z * 1.5,
                ) - cell_offset;
                println!("spawning cell here {}", cell_translation.clone());
                let cell = commands
                    .spawn(CellBundle::new(
                        id,
                        cell_translation,
                        cell_scale,
                        Color::rgba(i as f32, j as f32, 0.0, 0.1),
                        meshes,
                        materials,
                    ))
                    .id();

                //todo! Implement loading pre-made matrices
                let cell_designer = InteractionDesigner::new(
                    3,
                    cell_translation,
                    cell_scale,
                    0.01,
                    0.005,
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                let value_entity = commands.spawn(cell_designer).id();

                cell_row.push(cell);
                value_row.push(value_entity);
            }
            cells.push(cell_row);
            values.push(value_row);
        }

        Self {
            cell_entities: cells,
            cell_values: values,
            cur_edit_point: IVec2::ZERO,
            prev_edit_point: IVec2::ZERO,
            num_particles,
            prev_delta: Vec2::ZERO,
            edit: false,
        }
    }
}
