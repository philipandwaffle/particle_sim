use bevy::{math::vec3, prelude::*};

#[derive(Component)]
pub struct VertexLine {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_positions: Vec<Vec3>,
    pub cur_point_id: isize,
    pub num_points: usize,
}

#[derive(Bundle)]
pub struct VertexBundle {
    movable_point: Vertex,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl VertexBundle {
    pub fn new(
        id: usize,
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
    name: Name,
    line: DesignerLine,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl LineBundle {
    pub fn new(
        name: String,
        id: usize,
        from: Entity,
        to: Entity,
        thickness: f32,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            line: DesignerLine::new(id, from, to),
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
pub struct DesignerLine {
    pub id: usize,
    pub from: Entity,
    pub to: Entity,
}
impl DesignerLine {
    pub fn new(id: usize, from: Entity, to: Entity) -> Self {
        return Self { id, from, to };
    }
}

pub fn update_vertex_lines(
    mut vertex_lines: Query<&mut VertexLine, Changed<VertexLine>>,
    mut points: Query<&mut Transform, (With<Vertex>, Without<VertexLine>)>,
    mut lines: Query<&mut Transform, (With<VertexLine>, Without<Vertex>)>,
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
