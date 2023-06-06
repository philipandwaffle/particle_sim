use bevy::prelude::*;

use crate::floating_cam::controls::ControlState;

use super::{
    interaction_designer::{point::DesignerPoint, DesignerModeState},
    interaction_rule::interaction::InteractionRule,
};

pub struct MatrixDesignerPlugin;
impl Plugin for MatrixDesignerPlugin {
    fn build(&self, app: &mut App) {
        let num_particles = 6;
        app.insert_resource(Matrix::new(num_particles))
            .insert_resource(MatrixDesignerState::new(num_particles))
            .add_system(save_graph);
    }
}

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

#[derive(Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl CellBundle {
    pub fn new(
        id: usize,
        transform: Transform,
        color: Color,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            cell: Cell::new(id),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(shape::Cube { size: 1.0 }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: transform,
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct Cell {
    pub id: usize,
}
impl Cell {
    pub fn new(id: usize) -> Self {
        return Self { id: id };
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

fn spawn_matrix_designer(matrix_designer_state: Res<MatrixDesignerState>, mut commands: Commands) {
    let num_particles = matrix_designer_state.num_particles;
    let num_cells = num_particles.pow(2);

    // let mut cur_spawn = matrix_designer_state.centre
    for i in 0..num_cells {
        // commands.spawn(bundle)
    }
}
