use bevy::{ecs::component, prelude::*, transform::commands};

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
