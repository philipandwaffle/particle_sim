use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub type_id: usize,
}

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    damping: Damping,
}
impl ParticleBundle {
    pub fn new(
        type_id: usize,
        colors: &Vec<Color>,
        radius: f32,
        translation: Vec3,
        lin_damping: f32,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        collider: Collider,
    ) -> Self {
        let mut color = colors[type_id];

        return Self {
            particle: Particle { type_id },
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: radius,
                        subdivisions: 16,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(asset_server.load("textures/pickle.png")),
                    ..default()
                }),
                transform: Transform::from_translation(translation),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: collider,
            damping: Damping {
                linear_damping: lin_damping,
                angular_damping: 1.0,
            },
        };
    }
}
