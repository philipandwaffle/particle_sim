use bevy::{math::vec3, prelude::*};

use crate::floating_cam::controls::ControlState;

use self::{cell::CellBundle, matrix::Matrix, state::MatrixDesignerState};

use super::interaction_designer::{point::DesignerPoint, DesignerModeState};

mod cell;
mod matrix;
mod state;

pub struct MatrixDesignerPlugin;
impl Plugin for MatrixDesignerPlugin {
    fn build(&self, app: &mut App) {
        let num_particles = 6;
        app.insert_resource(Matrix::new(num_particles))
            .insert_resource(MatrixDesignerState::new(
                num_particles,
                vec3(5.0, 5.0, 1.0),
                vec3(0.0, 0.0, 5.0),
            ))
            .add_startup_system(spawn_matrix_designer)
            .add_system(save_graph);
    }
}

fn save_graph(
    mut control_state: ResMut<ControlState>,
    designer_mode_state: Res<DesignerModeState>,
    designer_points: Query<&Transform, With<DesignerPoint>>,
) {
    // Listen and check for key press
    if !control_state.save_designer_points {
        return;
    }
    control_state.save_designer_points = false;

    //
    let mut point_positions = Vec::with_capacity(designer_mode_state.num_points);
    for i in 0..designer_mode_state.num_points {
        let pos = if let Ok(transform) = designer_points.get(designer_mode_state.point_entities[i])
        {
            transform.translation
        } else {
            panic!();
        };

        point_positions.push(pos.truncate());
    }
}

fn spawn_matrix_designer(
    matrix_designer_state: Res<MatrixDesignerState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_particles = matrix_designer_state.num_particles;

    // let mut cur_spawn = matrix_designer_state.centre
    let mut cell_scale = matrix_designer_state.size / num_particles as f32;
    cell_scale.z = 1.0;
    let centre = matrix_designer_state.centre;

    for i in 0..num_particles {
        for j in 0..num_particles {
            let id = (i * num_particles) + j;
            let translation = vec3(
                i as f32 - num_particles as f32 / 2.0,
                j as f32 - num_particles as f32 / 2.0,
                2.0 * centre.z,
            ) - matrix_designer_state.centre;
            commands.spawn(CellBundle::new(
                id,
                translation,
                cell_scale,
                Color::GREEN,
                &mut meshes,
                &mut materials,
            ));
        }
    }
}
