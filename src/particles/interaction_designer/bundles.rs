use bevy::prelude::*;
#[derive(Bundle)]
pub struct LineBundle {
    name: Name,
    line: Line,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl LineBundle {
    pub fn new(
        name: String,
        from: Vec3,
        to: Vec3,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            line: Line::new(from, to),
            mat: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: todo!(),
                        height: todo!(),
                        resolution: todo!(),
                        segments: todo!(),
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("textures/checker_board.png")),
                    base_color: Color::WHITE,
                    ..default()
                }),
                transform: Transform::from_translation(from),
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct Line {
    pub from: Vec3,
    pub to: Vec3,
}
impl Line {
    pub fn new(from: Vec3, to: Vec3) -> Self {
        return Self { from, to };
    }
}

#[derive(Bundle)]
pub struct DesignerPointBundle {
    name: Name,
    movable_point: DesignerPoint,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl DesignerPointBundle {
    pub fn new(
        name: String,
        id: usize,
        radius: f32,
        translation: Vec3,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            movable_point: DesignerPoint::new(id),
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
pub struct DesignerPoint {
    pub id: usize,
}
impl DesignerPoint {
    fn new(id: usize) -> Self {
        return Self { id };
    }
}
