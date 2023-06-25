use std::collections::vec_deque;

use bevy::{
    math::{vec2, vec3},
    prelude::{
        default, shape, AssetServer, Assets, Bundle, Changed, Color, Commands, Component, Entity,
        MaterialMeshBundle, Mesh, Query, Res, StandardMaterial, Transform, Vec2, Vec3, With,
        Without,
    },
};
use bevy_rapier3d::parry::utils::center;

use super::{scale::ScaleBundle, NavControlled};
use crate::floating_cam::control_state::NavDelta;

#[derive(Bundle)]
pub struct VertexLineBundle {
    pub vertex_line: VertexLine,
    #[bundle]
    pub scale_bundle: ScaleBundle,
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
        println!("Spawning vertex line");
        let vertex_line = VertexLine::new(
            vertices,
            translation,
            scale,
            vertex_radius,
            line_thickness,
            commands,
            asset_server,
            meshes,
            materials,
        );
        let scale_bundle =
            ScaleBundle::new(translation, scale, 0, 5, 3, Color::WHITE, meshes, materials);
        return Self {
            vertex_line,
            scale_bundle: scale_bundle,
        };
    }
}

#[derive(Component)]
pub struct VertexLine {
    pub vertex_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub vertex_positions: Vec<Vec3>,
    pub cur_vertex_id: isize,
    pub num_vertices: usize,
    scale: Vec3,
    offset: Vec2,
    lower_bound: Vec2,
    upper_bound: Vec2,
}
impl NavControlled for VertexLine {
    fn trickle(&mut self, nav: NavDelta) {
        self.apply_primary_nav(nav.primary_nav);
        self.apply_secondary_nav(nav.secondary_nav);
    }
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

        let mut offset = translation.truncate();
        offset.x -= scale.x * 0.5;
        let lower_bound = (translation - scale / 2.0).truncate();
        let upper_bound = (translation + scale / 2.0).truncate();
        return Self {
            vertex_entities: point_entities,
            line_entities: line_entities,
            vertex_positions: point_positions,
            cur_vertex_id: 0,
            num_vertices: vertices,
            scale,
            offset,
            lower_bound,
            upper_bound,
        };
    }

    pub fn get_vertices(&self) -> Vec<Vec2> {
        return self
            .vertex_positions
            .iter()
            .map(|x| x.truncate() - self.offset)
            .collect::<Vec<Vec2>>();
    }

    fn apply_primary_nav(&mut self, delta: Vec2) {
        if self.cur_vertex_id == -1 || self.vertex_positions.is_empty() {
            return;
        }

        //todo! better error handling and logging
        let vertex_id = self.cur_vertex_id as usize;
        let cur_pos = &mut self.vertex_positions[vertex_id];
        let scaled_delta = vec2(delta.x * self.scale.x, delta.y * self.scale.y).extend(0.0);

        if vertex_id == 0 || vertex_id == self.num_vertices - 1 {
            cur_pos.y += scaled_delta.y;
        } else {
            *cur_pos += scaled_delta;
        }

        VertexLine::constrain_point(cur_pos, self.lower_bound, self.upper_bound);
    }

    fn constrain_point(pos: &mut Vec3, lower: Vec2, upper: Vec2) {
        if pos.x < lower.x {
            pos.x = lower.x;
        } else if pos.x > upper.x {
            pos.x = upper.x;
        }

        if pos.y < lower.y {
            pos.y = lower.y;
        } else if pos.y > upper.y {
            pos.y = upper.y;
        }
    }

    fn apply_secondary_nav(&mut self, delta: isize) {
        if delta == 0 {
            return;
        }

        // Aggregate current id with delta id
        self.cur_vertex_id += delta;

        // Check if new id is out of bounds and fix
        if self.cur_vertex_id == -1 {
            self.cur_vertex_id = self.num_vertices as isize - 1;
        } else if self.cur_vertex_id == self.num_vertices as isize {
            self.cur_vertex_id = 0;
        }
    }

    fn apply_primary_interact(&mut self, _: bool) {
        return;
    }

    fn apply_secondary_interact(&mut self, _: bool) {
        return;
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

// Update each vertex and line constituting a vertex line
pub fn update_vertex_lines(
    mut vertex_lines: Query<&mut VertexLine, Changed<VertexLine>>,
    mut points: Query<&mut Transform, (With<Vertex>, Without<Line>)>,
    mut lines: Query<&mut Transform, (With<Line>, Without<Vertex>)>,
) {
    // Loop through each vertex line
    for mut vertex_line in vertex_lines.iter_mut() {
        // Temp bindings
        let mut vertex_entities = vertex_line.vertex_entities.clone();
        let line_entities = vertex_line.line_entities.clone();
        let mut vertex_positions = vertex_line.vertex_positions.clone();
        let num_vertices = vertex_line.num_vertices.clone();

        // Loop through each vertex
        for i in 0..num_vertices.clone() {
            // Get vertex transform
            let vertex_translation = vertex_positions[i];
            let mut transform = match points.get_mut(vertex_entities[i]) {
                Ok(t) => t,
                Err(err) => panic!("Attempted to get a vertex that doesn't exist, {}", err),
            };

            // Update vertex translation if needs changed
            if transform.translation != vertex_translation {
                transform.translation = vertex_translation;
            }
        }

        // Loop through each line
        for i in 0..num_vertices.clone() - 1 {
            // Get line transform
            let mut transform = match lines.get_mut(line_entities[i]) {
                Ok(t) => t,
                Err(err) => panic!("Attempted to get a line that doesn't exist, {}", err),
            };

            // Get line's to and from translations
            let from = vertex_positions[i];
            let to = vertex_positions[i + 1];

            let dir = to - from;
            let dist = dir.length();

            // Update transform
            transform.scale = vec3(1.0, dist, 1.0);
            transform.translation = from + dir / 2.0;
            transform.look_to(Vec3::NEG_Z, dir)
        }

        let cur_id = vertex_line.cur_vertex_id;

        // Don't reorder the first and last point
        if cur_id < 1 || cur_id > (num_vertices - 2) as isize {
            continue;
        }
        let cur_id = cur_id as usize;

        // Get current and surrounding points
        let cur = vertex_positions[cur_id];
        let prev = vertex_positions[cur_id - 1];
        let next = vertex_positions[cur_id + 1];

        let mut swap_id_delta = 0;
        if cur.x > next.x {
            swap_id_delta = 1;
        } else if cur.x < prev.x {
            swap_id_delta = -1;
        }

        // Check if vertices need swapped
        if swap_id_delta != 0 {
            let swap_id = (cur_id as isize + swap_id_delta) as usize;
            vertex_entities.swap(cur_id, swap_id);
            vertex_positions.swap(cur_id, swap_id);
            vertex_line.cur_vertex_id += swap_id_delta;

            vertex_line.vertex_entities = vertex_entities;
            vertex_line.vertex_positions = vertex_positions;
        }
    }
}
