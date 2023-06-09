use crate::floating_cam::controls::ControlState;
use bevy::{math::vec3, prelude::*};
use core::panic;

pub struct InteractionDesignerPlugin;
impl Plugin for InteractionDesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_display);
    }
}

// use self::line::*;
use self::{interaction_designer::InteractionDesigner, line::DesignerLine, point::*};

pub mod interaction_designer;
mod line;
mod point;

pub fn update_display(
    mut designers: Query<&mut InteractionDesigner, Changed<InteractionDesigner>>,
    mut points: Query<&mut Transform, (With<DesignerPoint>, Without<DesignerLine>)>,
    mut lines: Query<&mut Transform, (With<DesignerLine>, Without<DesignerPoint>)>,
) {
    for mut designer in designers.iter_mut() {
        let mut point_entities = designer.point_entities.clone();
        let line_entities = designer.line_entities.clone();
        let mut point_positions = designer.point_positions.clone();

        let num_points = designer.num_points.clone();

        for i in 0..num_points.clone() {
            let point_translation = point_positions[i];
            let mut transform = if let Ok(transform) = points.get_mut(point_entities[i]) {
                transform
            } else {
                panic!();
            };

            if transform.translation != point_translation {
                transform.translation = point_translation;
            }
        }

        for i in 0..num_points.clone() - 1 {
            let cur = point_positions[i];
            let next = point_positions[i + 1];

            if cur.x > next.y {
                point_entities.swap(i, i + 1);
                point_positions.swap(i, i + 1);
            }
        }

        for i in 0..num_points.clone() - 1 {
            let mut transform = if let Ok(transform) = lines.get_mut(line_entities[i]) {
                transform
            } else {
                panic!();
            };

            let from = point_positions[i];
            let to = point_positions[i + 1];

            let dir = to - from;
            let dist = dir.length();
            transform.scale = vec3(1.0, dist, 1.0);
            transform.translation = from + dir / 2.0;
            transform.look_to(Vec3::NEG_Z, dir)
        }

        designer.point_entities = point_entities;
        designer.point_positions = point_positions;
    }
}

#[derive(Resource)]
pub struct InteractionDesignerState {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_pos: Vec<Vec2>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl InteractionDesignerState {
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

// fn spawn_design_room(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     asset_server: Res<AssetServer>,
// ) {
// }

fn move_lines(
    mut designer_lines: Query<&mut Transform, (With<DesignerLine>, Without<DesignerPoint>)>,
    designer_points: Query<(&Transform, With<DesignerPoint>)>,
    designer_mode_state: Res<InteractionDesignerState>,
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
    mut designer_mode_state: ResMut<InteractionDesignerState>,
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
    mut designer_mode_state: ResMut<InteractionDesignerState>,
) {
    // Get the vec containing the order of the points
    let points = designer_mode_state.point_entities.clone();

    // Change current point if change triggered
    if control_state.designer_secondary_nav_delta != 0 {
        // Aggregate current id with delta id
        designer_mode_state.cur_point_id += control_state.designer_secondary_nav_delta;

        // Check if new id is out of bounds and fix
        if designer_mode_state.cur_point_id == -1 {
            designer_mode_state.cur_point_id = designer_mode_state.num_points as isize - 1;
        } else if designer_mode_state.cur_point_id == designer_mode_state.num_points as isize {
            designer_mode_state.cur_point_id = 0;
        }
    }
    control_state.designer_secondary_nav_delta = 0;

    // Return if there is no transform to apply
    if control_state.designer_primary_nav_delta == Vec2::ZERO {
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
            transform.translation += control_state.designer_primary_nav_delta.extend(0.0) * 0.05;
        }
    }

    control_state.designer_primary_nav_delta = Vec2::ZERO;
}
