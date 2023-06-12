use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use serde::de::value;

use crate::particles::designers::{
    interaction::interaction_designer::InteractionDesigner, DesignerStates,
};

use super::cell::CellBundle;

#[derive(Component)]
pub struct MatrixDesigner {
    pub cell_entities: Vec<Vec<Entity>>,
    pub cell_values: Vec<Vec<Entity>>,
    pub cur_edit_point: IVec2,
    pub prev_edit_point: IVec2,
    pub num_particles: usize,
    pub prev_delta: Vec2,
}
impl MatrixDesigner {
    pub fn new(
        num_particles: usize,
        translation: Vec3,
        size: Vec3,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // let mut cur_spawn = matrix_designer_state.centre
        let mut cell_scale = size / (num_particles - 1) as f32;
        cell_scale.z = 1.0;
        let centre = translation;

        let mut cells = Vec::with_capacity(num_particles);
        let mut values = Vec::with_capacity(num_particles);
        for i in 0..num_particles {
            let mut cell_row = Vec::with_capacity(num_particles);
            let mut value_row = Vec::with_capacity(num_particles);
            for j in 0..num_particles {
                let j = num_particles - (1 + j);
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
                        Color::rgba(i as f32, j as f32, 0.0, 0.1),
                        meshes,
                        materials,
                    ))
                    .id();

                //todo! Implement loading pre-made matrices
                let cell_designer = InteractionDesigner::new(
                    3,
                    translation,
                    cell_scale,
                    0.005,
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
        }
    }
}
