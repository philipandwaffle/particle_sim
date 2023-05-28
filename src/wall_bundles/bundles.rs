use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[allow(dead_code)]
#[derive(Bundle)]
pub struct ClearWallBundle {
    name: Name,
    collider: Collider,
    transform_bundle: TransformBundle,
}
impl ClearWallBundle {
    pub fn new(name: String, size: Vec3, pos: Vec3) -> Self {
        return Self {
            name: Name::new(name),
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(pos)),
        };
    }
}

#[allow(dead_code)]
#[derive(Bundle)]
pub struct WallBundle {
    name: Name,
    collider: Collider,
    material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl WallBundle {
    pub fn new(
        name: String,
        size: Vec3,
        pos: Vec3,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(asset_server.load("textures/checker_board.png")),
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
        };
    }
}
