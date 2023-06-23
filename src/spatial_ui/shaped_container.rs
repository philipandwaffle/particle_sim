use bevy::prelude::*;

#[derive(Bundle)]
pub struct ShapedContainerBundle {
    pub cell: ShapedContainer,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl ShapedContainerBundle {
    pub fn new(
        translation: Vec3,
        scale: Vec3,
        color: Color,
        content: Entity,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            cell: ShapedContainer::new(
                Color::Rgba {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.1,
                },
                content,
            ),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(shape::Cube { size: 1.0 }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),

                transform: Transform {
                    translation,
                    scale,
                    ..default()
                },
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct ShapedContainer {
    pub color: Color,
    pub content: Entity,
}
impl ShapedContainer {
    pub fn new(color: Color, content: Entity) -> Self {
        return Self { color, content };
    }
}

pub fn update_shaped_containers(
    containers: Query<(&Handle<StandardMaterial>, &ShapedContainer), Changed<ShapedContainer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // loop through each container that has changes
    for (material_handle, container) in containers.iter() {
        let mut material = materials.get_mut(&material_handle).unwrap();
        material.base_color = container.color;
    }
}
