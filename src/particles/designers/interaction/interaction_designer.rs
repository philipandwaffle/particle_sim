use bevy::prelude::*;

use super::{line::LineBundle, point::DesignerPointBundle};

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
        scale: Vec3,
        point_radius: f32,
        line_thickness: f32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        let mut min = translation;
        min.x -= scale.x * 0.5;

        let mut dir = translation;
        dir.x += scale.x * 0.5;
        dir -= min;
        println!("min: {:?} dir: {:?}", min, dir);

        let mut point_entities = vec![];
        let mut point_positions = vec![];
        let mut line_entities = vec![];

        for id in 0..num_points {
            let mut pos = min + (dir * (id as f32 / (num_points - 1) as f32));
            pos.z = translation.z;
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
                    line_thickness,
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
