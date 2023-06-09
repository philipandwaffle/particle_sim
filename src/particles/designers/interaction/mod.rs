use crate::floating_cam::controls::ControlState;
use bevy::{math::vec3, prelude::*};
use core::panic;

// use self::line::*;
use self::{
    interaction_designer::InteractionDesigner,
    line::{DesignerLine, LineBundle},
    point::*,
};

mod interaction_designer;
mod line;
mod point;

fn update_display(
    mut designers: Query<&mut InteractionDesigner, Changed<InteractionDesigner>>,
    mut points: Query<&mut Transform, (With<DesignerPoint>, Without<DesignerLine>)>,
    mut lines: Query<&mut Transform, (With<DesignerLine>, Without<DesignerPoint>)>,
) {
    for mut designer in designers.iter_mut() {
        let point_entities = &mut designer.point_entities;
        let line_entities = &mut designer.line_entities;
        let point_positions = &mut designer.point_positions;

        let num_points = designer.num_points.clone();

        for i in 0..num_points.clone() {
            let translation = point_positions[i];
            let mut transform = if let Ok(transform) = points.get_mut(point_entities[i]) {
                transform
            } else {
                panic!();
            };

            if transform.translation.truncate() != translation {
                transform.translation = translation.extend(transform.translation.y);
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

            let z = transform.translation.z.clone();
            let from = point_positions[i].extend(z);
            let to = point_positions[i + 1].extend(z);

            let dir = to - from;
            let dist = dir.length();
            transform.scale = vec3(1.0, dist, 1.0);
            transform.translation = from + dir / 2.0;
            transform.look_to(Vec3::NEG_Z, dir)
        }
    }
}

pub struct InteractionDesignerPlugin;
impl Plugin for InteractionDesignerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionDesignerState::new(6))
            // .add_startup_system(spawn_design_room)
            .add_startup_system(spawn_design_terminal)
            .add_system(move_point)
            .add_system(move_lines)
            // .add_system(save_graph)
            .add_system(reorder_points_and_lines);

        app.add_system(update_display);
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

fn spawn_design_terminal(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut designer_mode_state: ResMut<InteractionDesignerState>,
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