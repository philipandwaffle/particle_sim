use bevy::{prelude::*, utils::Instant};
use bevy_rapier3d::prelude::*;
use random::Source;
use serde::{Deserialize, Serialize};

use self::{
    interaction_rule::{insert_interaction_matrix, matrix::InteractionMatrix},
    movement_functions::move_particles,
    particle_bundle::{Particle, ParticleBundle},
    particle_metadata::{AttractionFunc, ParticleMetadata},
};

pub mod attraction_functions;
mod interaction_rule;
mod movement_functions;
mod particle_bundle;
pub mod particle_metadata;

#[derive(Resource)]
pub struct ParticleSpawnInfo {
    pub count: usize,
    pub spawn_points: Vec<Vec3>,
    pub type_ids: Vec<usize>,
    pub radius: f32,
    pub lin_damping: f32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", content = "c")]
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

pub struct ParticlesPlugin {
    pub min: Vec3,
    pub max: Vec3,
    pub seed: Option<u64>,
    pub attraction_matrix: Vec<Vec<AttractionFunc>>,
    pub type_id_counts: Count,
    pub colors: Vec<Color>,
    pub radius: f32,
    pub lin_damping: f32,
}
impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        let seed = if let Some(s) = &self.seed {
            s.clone()
        } else {
            Instant::now().elapsed().as_nanos() as u64
        };

        let mut particle_count = 0;
        let type_ids: Vec<usize> = match &self.type_id_counts {
            Count::Random(count) => {
                particle_count = *count;
                get_rand_usizes(self.colors.len(), *count, seed)
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

        app.insert_resource(ParticleSpawnInfo {
            count: particle_count,
            spawn_points: get_rand_vec3s(self.min, self.max, particle_count, seed),
            type_ids: type_ids,
            radius: self.radius,
            lin_damping: self.lin_damping,
        })
        .insert_resource(InteractionMatrix::new(6))
        // .add_startup_system(insert_interaction_matrix)
        .insert_resource(ParticleMetadata::new(
            self.min,
            self.max,
            self.attraction_matrix.clone(),
            self.colors.clone(),
        ))
        .add_startup_system(spawn_particles)
        // .add_system(constrain_particles)
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
    spawn_info: Res<ParticleSpawnInfo>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let collider = Collider::ball(spawn_info.radius);
    for i in 0..spawn_info.count {
        commands.spawn(ParticleBundle::new(
            spawn_info.type_ids[i].clone(),
            &particle_metadata.colors,
            spawn_info.radius,
            spawn_info.spawn_points[i].clone(),
            spawn_info.lin_damping,
            meshes.as_mut(),
            materials.as_mut(),
            collider.clone(),
        ));
    }

    commands.remove_resource::<ParticleSpawnInfo>()
}

#[allow(dead_code)]
fn constrain_particles(
    mut all: Query<&mut Transform, With<Particle>>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let min = particle_metadata.min;
    let max = particle_metadata.max;

    for mut trans in all.iter_mut() {
        let t = &mut trans.translation;
        if t.x > max.x {
            t.x = min.x;
        } else if t.x < min.x {
            t.x = max.x;
        }

        if t.y > max.y {
            t.y = min.y;
        } else if t.y < min.y {
            t.y = max.y;
        }

        if t.z > max.z {
            t.z = min.z;
        } else if t.z < min.z {
            t.z = max.z;
        }
    }
}
