use bevy::prelude::*;

use crate::particles::interaction_rule;

use super::{line::LineBundle, point::DesignerPointBundle};

// #[derive(Bundle)]
// pub struct InteractionDesignerBundle {
//     interaction_designer: InteractionDesigner,
//     transform: Transform,
// }
// impl InteractionDesignerBundle {
//     pub fn new(
//         translation: Vec3,
//         size: Vec3,
//         num_points: usize,
//         point_radius: f32,
//         commands: &mut Commands,
//         asset_server: &Res<AssetServer>,
//         meshes: &mut Assets<Mesh>,
//         materials: &mut Assets<StandardMaterial>,
//     ) -> Self {
//         let min = translation - size * 0.5;
//         let dir = translation + (size * 0.5) - min;

//         let mut point_entities = vec![];
//         let mut point_positions = vec![];
//         let mut line_entities = vec![];

//         for id in 0..num_points {
//             let pos = min + dir * (id as f32 / (num_points - 1) as f32);
//             let point = commands
//                 .spawn(DesignerPointBundle::new(
//                     "point_1".into(),
//                     id as usize,
//                     point_radius,
//                     pos,
//                     &asset_server,
//                     meshes,
//                     materials,
//                 ))
//                 .id();
//             point_entities.push(point);
//             point_positions.push(pos);
//         }

//         for id in 0..num_points - 1 {
//             let id = id as usize;

//             let line = commands
//                 .spawn(LineBundle::new(
//                     "".into(),
//                     id,
//                     point_entities[id],
//                     point_entities[id + 1],
//                     0.05,
//                     meshes,
//                     materials,
//                 ))
//                 .id();
//             line_entities.push(line);
//         }

//         return Self {
//             interaction_designer: InteractionDesigner::new(
//                 num_points,
//                 point_entities,
//                 line_entities,
//                 point_positions,
//             ),
//             transform: Transform::from_translation(translation),
//         };
//     }
// }

#[derive(Component)]
pub struct InteractionDesigner {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_positions: Vec<Vec3>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl InteractionDesigner {
    pub fn new(
        num_points: usize,
        translation: Vec3,
        size: Vec3,
        point_radius: f32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let min = translation - size * 0.5;
        let dir = translation + (size * 0.5) - min;

        let mut point_entities = vec![];
        let mut point_positions = vec![];
        let mut line_entities = vec![];

        for id in 0..num_points {
            let pos = min + dir * (id as f32 / (num_points - 1) as f32);
            let point = commands
                .spawn(DesignerPointBundle::new(
                    "point_1".into(),
                    id as usize,
                    point_radius,
                    pos,
                    &asset_server,
                    meshes,
                    materials,
                ))
                .id();
            point_entities.push(point);
            point_positions.push(pos);
        }

        for id in 0..num_points - 1 {
            let id = id as usize;

            let line = commands
                .spawn(LineBundle::new(
                    "".into(),
                    id,
                    point_entities[id],
                    point_entities[id + 1],
                    0.05,
                    meshes,
                    materials,
                ))
                .id();
            line_entities.push(line);
        }

        return Self {
            point_entities: point_entities,
            line_entities: line_entities,
            point_positions: point_positions,
            cur_point_id: -1,
            num_points,
        };
    }
}
