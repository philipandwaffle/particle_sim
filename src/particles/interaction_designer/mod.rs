use core::panic;

use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::inspector_options::Target;

use crate::floating_cam::controls::ControlState;

use self::line::*;
use self::point::*;

mod line;
mod point;

pub struct DesignerModePlugin;
impl Plugin for DesignerModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DesignerModeState::new(6))
            .add_startup_system(spawn_design_room)
            .add_startup_system(spawn_design_terminal)
            .add_system(move_point)
            .add_system(move_lines);
    }
}

#[derive(Resource)]
pub struct DesignerModeState {
    pub points: Vec<Entity>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl DesignerModeState {
    pub fn new(num_points: usize) -> Self {
        return Self {
            points: Vec::with_capacity(num_points),
            cur_point_id: 0,
            num_points: num_points,
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
    mut designer_mode_state: ResMut<DesignerModeState>,
) {
    let radius = 0.5;
    let num_points = designer_mode_state.num_points;

    let min = vec3(-5.0, 1.0, -5.0);
    let max = vec3(5.0, 1.0, -5.0);

    let dir = max - min;
    for id in 0..num_points {
        let point = commands
            .spawn(DesignerPointBundle::new(
                "point_1".into(),
                id,
                radius,
                min + dir * (id as f32 / (num_points - 1) as f32),
                &asset_server,
                &mut meshes,
                &mut materials,
            ))
            .id();
        designer_mode_state.points.push(point);
    }

    for id in 0..num_points - 1 {
        commands.spawn(LineBundle::new(
            "".into(),
            id,
            designer_mode_state.points[id],
            designer_mode_state.points[id + 1],
            0.05,
            &mut meshes,
            &mut materials,
        ));
    }
}

fn move_lines(
    mut designer_lines: Query<(&mut Transform, &DesignerLine), Without<DesignerPoint>>,
    designer_points: Query<(&Transform, With<DesignerPoint>)>,
    designer_mode_state: Res<DesignerModeState>,
) {
    for (mut transform, line) in designer_lines.iter_mut() {
        let from = if let Ok((point, _)) = designer_points.get(line.from) {
            point.translation
        } else {
            panic!();
        };

        let to = if let Ok((point, _)) = designer_points.get(line.to) {
            point.translation
        } else {
            panic!()
        };

        let dir = to - from;
        let dist = dir.length();
        transform.scale = vec3(1.0, dist, 1.0);
        transform.translation = from + dir / 2.0;
        transform.look_to(Vec3::NEG_Z, dir)
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
