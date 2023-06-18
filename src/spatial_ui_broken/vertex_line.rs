use std::cell::RefCell;

use bevy::{math::vec3, prelude::*, render::mesh};
use bevy_trait_query::One;

use super::{root::Dreg, Trickles};

#[derive(Bundle)]
pub struct VertexLineBundle {
    pub vertex_line: VertexLine,
    pub transform: Transform,
}
impl VertexLineBundle {
    pub fn new(
        vertices: usize,
        translation: Vec3,
        scale: Vec3,
        vertex_radius: f32,
        line_thickness: f32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            vertex_line: VertexLine::new(
                vertices,
                translation,
                scale,
                vertex_radius,
                line_thickness,
                commands,
                asset_server,
                meshes,
                materials,
            ),
            transform: Transform::from_translation(translation),
        };
    }
}

#[derive(Component)]
pub struct VertexLine {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_positions: Vec<Vec3>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
impl Trickles for VertexLine {
    fn drip(&mut self, _: Query<One<&mut dyn Trickles>>, dreg: Dreg) {}
}
impl VertexLine {
    pub fn new(
        vertices: usize,
        translation: Vec3,
        scale: Vec3,
        vertex_radius: f32,
        line_thickness: f32,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // Calculate the pos of the first vertex in a line
        let mut start = translation;
        start.x -= scale.x * 0.5;

        // Calculate the pos of the last vertex in a line
        let mut stop = translation;
        stop.x += scale.x * 0.5;
        // Relative to the start position
        stop -= start;

        // Pre-allocate vectors
        let mut point_entities = Vec::with_capacity(vertices);
        let mut point_positions = Vec::with_capacity(vertices);
        let mut line_entities = Vec::with_capacity(vertices - 1);

        // Spawn each vertex
        for i in 0..vertices {
            // Linearly interpolate between start and stop to get vertex pos
            let mut pos = start + (stop * (i as f32 / (vertices - 1) as f32));
            pos.z = translation.z;

            // Spawn vertex
            let vertex = commands
                .spawn(VertexBundle::new(
                    vertex_radius,
                    pos,
                    &asset_server,
                    meshes,
                    materials,
                ))
                .id();

            point_entities.push(vertex);
            point_positions.push(pos);
        }

        // Spawn each line
        for i in 0..vertices - 1 {
            let line = commands
                .spawn(LineBundle::new(
                    point_entities[i],
                    point_entities[i + 1],
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
            cur_point_id: 0,
            num_points: vertices,
        };
    }
}

#[derive(Bundle)]
pub struct VertexBundle {
    movable_point: Vertex,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl VertexBundle {
    pub fn new(
        radius: f32,
        translation: Vec3,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            movable_point: Vertex,
            mat: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: radius,
                        subdivisions: 16,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("textures/checker_board.png")),
                    base_color: Color::WHITE,
                    ..default()
                }),
                transform: Transform::from_translation(translation),
                ..default()
            },
        };
    }
}

#[derive(Component)]
pub struct Vertex;

#[derive(Bundle)]
pub struct LineBundle {
    line: Line,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl LineBundle {
    pub fn new(
        from: Entity,
        to: Entity,
        thickness: f32,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            line: Line::new(from, to),
            mat: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: thickness,
                        height: 1.0,
                        resolution: 6,
                        segments: 6,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct Line {
    pub from: Entity,
    pub to: Entity,
}
impl Line {
    pub fn new(from: Entity, to: Entity) -> Self {
        return Self { from, to };
    }
}

pub fn update_vertex_lines(
    mut vertex_lines: Query<&mut VertexLine, Changed<VertexLine>>,
    mut points: Query<&mut Transform, (With<Vertex>, Without<Line>)>,
    mut lines: Query<&mut Transform, (With<Line>, Without<Vertex>)>,
) {
    for mut designer in vertex_lines.iter_mut() {
        let mut point_entities = designer.point_entities.clone();
        let line_entities = designer.line_entities.clone();
        let mut point_positions = designer.point_positions.clone();

        let num_points = designer.num_points.clone();

        for i in 0..num_points.clone() {
            let point_translation = point_positions[i];
            let mut transform = if let Ok(transform) = points.get_mut(point_entities[i]) {
                transform
            } else {
                panic!("Update vertex line panic");
            };

            if transform.translation != point_translation {
                transform.translation = point_translation;
            }
        }

        for i in 0..num_points.clone() - 1 {
            // let mut transform = if let Ok(transform) = lines.get_mut(line_entities[i]) {
            //     transform
            // } else {
            //     panic!();
            // };

            let mut transform = match lines.get_mut(line_entities[i]) {
                Ok(t) => t,
                Err(err) => panic!("{:?}", err),
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
