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

        let cur_id = designer.cur_point_id;

        // Don't reorder the first and last point
        if cur_id < 1 || cur_id > (num_points - 2) as isize {
            continue;
        }
        let cur_id = cur_id as usize;

        // Get current and surrounding points
        let cur = point_positions[cur_id];
        let prev = point_positions[cur_id - 1];
        let next = point_positions[cur_id + 1];

        let mut swap_id_delta = 0;
        if cur.x > next.x {
            swap_id_delta = 1;
        } else if cur.x < prev.x {
            swap_id_delta = -1;
        }

        if swap_id_delta != 0 {
            let swap_id = (cur_id as isize + swap_id_delta) as usize;
            point_entities.swap(cur_id, swap_id);
            point_positions.swap(cur_id, swap_id);

            designer.cur_point_id += swap_id_delta;
        }

        designer.point_entities = point_entities;
        designer.point_positions = point_positions;
    }
}
