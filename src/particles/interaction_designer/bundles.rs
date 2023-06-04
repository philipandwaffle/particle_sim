use bevy::{ecs::component, prelude::*, transform::commands};

#[derive(Bundle)]
pub struct MovablePointBundle {
    name: Name,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl MovablePointBundle {
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
            mat: MaterialMeshBundle {
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

#[derive(Component)]
struct MovablePoint;
