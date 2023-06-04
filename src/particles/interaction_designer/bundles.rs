use bevy::{ecs::component, prelude::*, transform::commands};

#[derive(Bundle)]
pub struct MovablePointBundle {
    name: Name,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl MovablePointBundle {
    pub fn new(
        name: String,
        radius: f32,
        translation: Vec3,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            mat: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: radius,
                        subdivisions: 10,
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
struct MovablePoint;
