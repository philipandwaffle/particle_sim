use crate::floating_cam::controls::ControlState;
use bevy::{math::vec3, prelude::*};
use core::panic;

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
            .add_system(move_lines)
            .add_system(save_graph)
            .add_system(reorder_points_and_lines);
    }
}

#[derive(Resource)]
pub struct DesignerModeState {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_pos: Vec<Vec2>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl DesignerModeState {
    pub fn new(num_points: usize) -> Self {
        return Self {
            point_entities: Vec::with_capacity(num_points),
            line_entities: Vec::with_capacity(num_points - 1),
            point_pos: Vec::with_capacity(num_points),
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
        designer_mode_state.point_entities.push(point);
    }

    for id in 0..num_points - 1 {
        let line = commands
            .spawn(LineBundle::new(
                "".into(),
                id,
                designer_mode_state.point_entities[id],
                designer_mode_state.point_entities[id + 1],
                0.05,
                &mut meshes,
                &mut materials,
            ))
            .id();
        designer_mode_state.line_entities.push(line);
    }
}

fn move_lines(
    mut designer_lines: Query<&mut Transform, (With<DesignerLine>, Without<DesignerPoint>)>,
    designer_points: Query<(&Transform, With<DesignerPoint>)>,
    designer_mode_state: Res<DesignerModeState>,
) {
    for i in 0..designer_mode_state.num_points - 1 {
        let mut transform =
            if let Ok(transform) = designer_lines.get_mut(designer_mode_state.line_entities[i]) {
                transform
            } else {
                panic!();
            };

        let from =
            if let Ok((point, _)) = designer_points.get(designer_mode_state.point_entities[i]) {
                point.translation
            } else {
                panic!();
            };

        let to = if let Ok((point, _)) =
            designer_points.get(designer_mode_state.point_entities[i + 1])
        {
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

fn reorder_points_and_lines(
    designer_points: Query<&Transform>,
    mut designer_mode_state: ResMut<DesignerModeState>,
) {
    // Loop through each point in order
    for i in 0..designer_mode_state.num_points {
        // Get id and transform of the current point
        let transform =
            if let Ok(transform) = designer_points.get(designer_mode_state.point_entities[i]) {
                transform
            } else {
                panic!();
            };

        // Don't reorder the first and last point
        if i == 0 || i == designer_mode_state.num_points - 1 {
            continue;
        }

        // Get surrounding points
        let prev_point = designer_points
            .get(designer_mode_state.point_entities[i - 1])
            .unwrap()
            .translation;
        let next_point = designer_points
            .get(designer_mode_state.point_entities[i + 1])
            .unwrap()
            .translation;

        // Swap point order
        if transform.translation.x < prev_point.x {
            println!("swapping {} and {}", i, i - 1);
            designer_mode_state.point_entities.swap(i, i - 1);
        }
        if transform.translation.x > next_point.x {
            println!("swapping {} and {}", i, i + 1);
            designer_mode_state.point_entities.swap(i, i + 1);
        }
    }
}

fn move_point(
    mut designer_points: Query<(&DesignerPoint, &mut Transform)>,
    mut control_state: ResMut<ControlState>,
    mut designer_mode_state: ResMut<DesignerModeState>,
) {
    // Get the vec containing the order of the points
    let points = designer_mode_state.point_entities.clone();

    // Change current point if change triggered
    if control_state.design_point_id_delta != 0 {
        // Aggregate current id with delta id
        designer_mode_state.cur_point_id += control_state.design_point_id_delta;

        // Check if new id is out of bounds and fix
        if designer_mode_state.cur_point_id == -1 {
            designer_mode_state.cur_point_id = designer_mode_state.num_points as isize - 1;
        } else if designer_mode_state.cur_point_id == designer_mode_state.num_points as isize {
            designer_mode_state.cur_point_id = 0;
        }
    }
    control_state.design_point_id_delta = 0;

    // Return if there is no transform to apply
    if control_state.design_point_delta == Vec2::ZERO {
        return;
    }

    // Get current id
    let cur_id = designer_mode_state.cur_point_id;

    // Loop through each point in order
    for i in 0..designer_mode_state.num_points {
        // Get id and transform of the current point
        let (id, mut transform) =
            if let Ok((designer_point, transform)) = designer_points.get_mut(points[i]) {
                (designer_point.id, transform)
            } else {
                panic!();
            };

        // Apply transform to the selected point
        if id == cur_id as usize {
            transform.translation += control_state.design_point_delta.extend(0.0) * 0.05;
        }
    }

    control_state.design_point_delta = Vec2::ZERO;
}

fn save_graph(
    mut control_state: ResMut<ControlState>,
    mut designer_mode_state: ResMut<DesignerModeState>,
    designer_points: Query<&Transform, With<DesignerPoint>>,
) {
    if !control_state.save_designer_points {
        return;
    }
    control_state.save_designer_points = false;

    for i in 0..designer_mode_state.num_points {
        let pos = if let Ok(transform) = designer_points.get(designer_mode_state.point_entities[i])
        {
            transform.translation
        } else {
            panic!();
        };

        designer_mode_state.point_pos[i] = pos.truncate();
    }
}
