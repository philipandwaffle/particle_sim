use bevy::{ecs::component, math::vec3, prelude::*, transform::commands};

use crate::floating_cam::controls::ControlState;

use self::bundles::{DesignerPoint, DesignerPointBundle};

mod bundles;
pub struct DesignerModePlugin;
impl Plugin for DesignerModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DesignerModeState::new(vec![0, 1, 2, 3, 4, 5]))
            .add_startup_system(spawn_design_room)
            .add_startup_system(spawn_design_terminal)
            .add_system(move_point);
    }
}

#[derive(Resource)]
pub struct DesignerModeState {
    pub point_order: Vec<usize>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl DesignerModeState {
    pub fn new(point_order: Vec<usize>) -> Self {
        return Self {
            point_order: point_order.clone(),
            cur_point_id: 0,
            num_points: point_order.len(),
        };
    }
}

fn spawn_design_room(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(DesignerPointBundle::new(
    //     "Movable Point".into(),
    //     1,
    //     0.5,
    //     vec3(0.0, 1.0, -5.0),
    //     &asset_server,
    //     &mut meshes,
    //     &mut materials,
    // ));
}

fn spawn_design_terminal(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    designer_mode_state: Res<DesignerModeState>,
) {
    let radius = 0.5;
    let num_points = designer_mode_state.num_points;

    let min = vec3(-5.0, 1.0, -5.0);
    let max = vec3(5.0, 1.0, -5.0);

    let dir = max - min;
    for id in 0..num_points {
        commands.spawn(DesignerPointBundle::new(
            "point_1".into(),
            id,
            radius,
            min + dir * (id as f32 / (num_points - 1) as f32),
            &asset_server,
            &mut meshes,
            &mut materials,
        ));
    }
}

fn move_point(
    mut designer_points: Query<(&mut DesignerPoint, &mut Transform)>,
    mut control_state: ResMut<ControlState>,
    mut designer_mode_state: ResMut<DesignerModeState>,
) {
    if control_state.design_point_id_delta != 0 {
        designer_mode_state.cur_point_id += control_state.design_point_id_delta;

        if designer_mode_state.cur_point_id == -1 {
            designer_mode_state.cur_point_id = designer_mode_state.num_points as isize - 1;
        } else if designer_mode_state.cur_point_id == designer_mode_state.num_points as isize {
            designer_mode_state.cur_point_id = 0;
        }
    }
    control_state.design_point_id_delta = 0;

    if control_state.design_point_delta == Vec2::ZERO {
        return;
    }

    let cur_id = designer_mode_state.cur_point_id;
    for (mut point, mut transform) in designer_points.iter_mut() {
        if point.id == cur_id as usize {
            transform.translation += control_state.design_point_delta.extend(0.0) * 0.05;
        }
    }

    control_state.design_point_delta = Vec2::ZERO;
}
