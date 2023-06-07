use bevy::{prelude::*, utils::Instant};
use bevy_rapier3d::prelude::*;
use random::Source;
use serde::{Deserialize, Serialize};

use crate::config::structs::{ParticleProperties, Spawn};

use self::{
    interaction_designer::InteractionDesignerPlugin, interaction_rule::matrix::InteractionMatrix,
    matrix_designer::MatrixDesignerPlugin, movement_functions::move_particles,
    particle::ParticleBundle,
};

pub mod interaction_designer;
mod interaction_rule;
mod matrix_designer;
mod movement_functions;
mod particle;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Count {
    Random(usize),
    Set(Vec<usize>),
}
impl Default for Count {
    fn default() -> Self {
        Self::Random(0)
    }
}

pub struct ParticlesPlugin;
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InteractionMatrix::new(6))
            .add_plugin(MatrixDesignerPlugin)
            .add_plugin(InteractionDesignerPlugin)
            // .add_startup_system(spawn_particles)
            .add_system(move_particles);
        // .add_system(parallel_move_particles);
        // .add_system(parallel_arc_move_particles);
    }
}

fn get_rand_vec3s(min: Vec3, max: Vec3, count: usize, seed: u64) -> Vec<Vec3> {
    let mut source = random::default(seed);
    let rand_floats = source.iter().take(count * 3).collect::<Vec<f32>>();
    let mut vec3s = vec![];

    while vec3s.len() < count {
        let new_vec3 = Vec3::new(
            rand_floats[vec3s.len() * 3] * (max.x - min.x + 1.0) + min.x,
            rand_floats[vec3s.len() * 3 + 1] * (max.y - min.y + 1.0) + min.y,
            rand_floats[vec3s.len() * 3 + 2] * (max.z - min.z + 1.0) + min.z,
        );
        vec3s.push(new_vec3)
    }
    return vec3s;
}
fn get_rand_usizes(max: usize, count: usize, seed: u64) -> Vec<usize> {
    let mut source = random::default(seed);
    let rand_floats = source.iter().take(count).collect::<Vec<f32>>();
    return rand_floats
        .iter()
        .map(|x| (x * max as f32) as usize)
        .collect();
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    spawn_info: Res<Spawn>,
    particle_properties: Res<ParticleProperties>,
) {
    let seed = if let Some(s) = &spawn_info.seed {
        s.clone()
    } else {
        Instant::now().elapsed().as_nanos() as u64
    };

    let mut particle_count = 0;
    let type_ids: Vec<usize> = match &spawn_info.type_id_counts {
        Count::Random(count) => {
            particle_count = *count;
            get_rand_usizes(spawn_info.colors.len(), *count, seed)
        }
        Count::Set(counts) => {
            let mut ids = vec![];
            let mut id = 0;
            for count in counts.iter() {
                particle_count += *count;
                ids.append(&mut vec![id; *count]);
                id += 1;
            }
            ids
        }
    };

    let spawn_positions = get_rand_vec3s(spawn_info.min, spawn_info.max, particle_count, seed);

    let collider = Collider::ball(particle_properties.radius);
    for i in 0..particle_count {
        commands.spawn(ParticleBundle::new(
            type_ids[i].clone(),
            &spawn_info.colors,
            particle_properties.radius,
            spawn_positions[i].clone(),
            particle_properties.lin_damping,
            &asset_server,
            meshes.as_mut(),
            materials.as_mut(),
            collider.clone(),
        ));
    }
}

// #[allow(dead_code)]
// fn constrain_particles(
//     mut all: Query<&mut Transform, With<Particle>>,
//     particle_metadata: Res<ParticleMetadata>,
// ) {
//     let min = particle_metadata.min;
//     let max = particle_metadata.max;

//     for mut trans in all.iter_mut() {
//         let t = &mut trans.translation;
//         if t.x > max.x {
//             t.x = min.x;
//         } else if t.x < min.x {
//             t.x = max.x;
//         }

//         if t.y > max.y {
//             t.y = min.y;
//         } else if t.y < min.y {
//             t.y = max.y;
//         }

//         if t.z > max.z {
//             t.z = min.z;
//         } else if t.z < min.z {
//             t.z = max.z;
//         }
//     }
// }
