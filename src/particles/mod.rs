use std::{sync::Arc, thread};

use bevy::{prelude::*, utils::Instant};
use bevy_rapier3d::prelude::*;
use random::Source;

use self::{
    particle_bundle::{Particle, ParticleBundle},
    particle_metadata::{AttractionFunc, ParticleMetadata},
};

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

pub enum Count {
    Random(usize),
    Set(Vec<usize>),
}
pub struct ParticlesPlugin {
    pub min: Vec3,
    pub max: Vec3,
    pub seed: Option<u64>,
    pub attraction_matrix: Vec<Vec<AttractionFunc<f32>>>,
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
        .insert_resource(ParticleMetadata::new(
            self.min,
            self.max,
            self.attraction_matrix.clone(),
            self.colors.clone(),
        ))
        .add_startup_system(spawn_particles)
        // .add_system(constrain_particles)
        // .add_system(move_particles);
        // .add_system(parallel_move_particles);
        .add_system(parallel_arc_move_particles);
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

fn move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let compare_vec = particles
        .iter()
        .map(|x| (x.0, x.1.type_id, x.3.translation))
        .collect::<Vec<(Entity, usize, Vec3)>>();

    // let compare_vec_b = compare_vec.clone();

    // for i in 0..compare_vec.len() {
    //     if compare_vec[i.clone()].0 != compare_vec_b[i].0 {
    //         println!("false");
    //     }
    // }

    // let now = Instant::now();
    for (entity, particle, mut velocity, transform) in particles.iter_mut() {
        let mut total_vel = Vec3::ZERO;
        for (compare_entity, compare_type_id, compare_translation) in compare_vec.iter() {
            if entity == *compare_entity {
                continue;
            }
            // let dir = *compare_translation - transform.translation;
            // let square_dist = dir.length_squared();

            // let attract_modifier = particle_metadata.attraction_matrix[particle.type_id]
            //     [*compare_type_id](square_dist);
            // total_vel += dir.normalize() * attract_modifier * 0.1;

            let dir = *compare_translation - transform.translation;
            let dist = dir.length();
            let attract_modifier =
                particle_metadata.attraction_matrix[particle.type_id][*compare_type_id](dist);
            if attract_modifier == 0.0 {
                continue;
            }
            total_vel += (dir / dist) * attract_modifier * 0.1;
        }
        velocity.linvel += total_vel;
    }
    // let elapsed_time = now.elapsed();
    // println!("{}", elapsed_time.as_nanos());
}

fn parallel_move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let compare_vec = particles
        .iter()
        .map(|(e, p, _, t)| (e, p.type_id, t.translation))
        .collect::<Vec<(Entity, usize, Vec3)>>();

    let num_threads = 16;
    let num_particles = compare_vec.len();

    let mut threads = Vec::with_capacity(num_threads);

    let mut cur_start = 0;
    let step = num_particles / num_threads;
    let mut cur_stop = step;

    for i in 0..num_threads {
        if i == num_threads - 1 {
            cur_stop = num_particles;
        }

        let start = cur_start.clone();
        let stop = cur_stop.clone();

        let attraction_matrix = particle_metadata.attraction_matrix.clone();
        let compare_vec = compare_vec.clone();

        threads.push(thread::spawn(move || {
            let mut velocities = Vec::with_capacity(stop - start);

            let particles = &compare_vec[start..stop];
            for i in 0..particles.len() {
                let mut total_vel = Vec3::ZERO;
                for (compare_entity, compare_type_id, compare_translation) in compare_vec.iter() {
                    if particles[i].0 == *compare_entity {
                        continue;
                    }

                    let dir = *compare_translation - particles[i].2;
                    let dist = dir.length();
                    let attract_modifier =
                        attraction_matrix[particles[i].1][*compare_type_id](dist);
                    if attract_modifier == 0.0 {
                        continue;
                    }
                    total_vel += (dir / dist) * attract_modifier * 0.1;
                }
                velocities.push(total_vel);
            }
            velocities
        }));

        cur_start += step;
        cur_stop += step;
    }

    let mut vels: Vec<Vec3> = Vec::with_capacity(num_particles);
    for handle in threads {
        vels.append(&mut handle.join().unwrap());
    }

    let mut i = 0;
    for (_, _, mut vel, _) in particles.iter_mut() {
        vel.linvel += vels[i];
        i += 1;
    }
}

fn parallel_arc_move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let compare_vec = Arc::new(
        particles
            .iter()
            .map(|(e, p, _, t)| (e, p.type_id, t.translation))
            .collect::<Vec<(Entity, usize, Vec3)>>(),
    );
    let attraction_matrix = Arc::new(particle_metadata.attraction_matrix.clone());

    let num_threads = 16;
    let num_particles = compare_vec.len();

    let mut threads = Vec::with_capacity(num_threads);

    let mut cur_start = 0;
    let step = num_particles / num_threads;
    let mut cur_stop = step;

    for i in 0..num_threads {
        if i == num_threads - 1 {
            cur_stop = num_particles;
        }

        let start = cur_start.clone();
        let stop = cur_stop.clone();

        let attraction_matrix = Arc::clone(&attraction_matrix);
        let compare_vec = Arc::clone(&compare_vec);

        threads.push(thread::spawn(move || {
            let mut velocities = Vec::with_capacity(stop - start);

            let particles = &compare_vec[start..stop];
            for i in 0..particles.len() {
                let mut total_vel = Vec3::ZERO;
                for (compare_entity, compare_type_id, compare_translation) in compare_vec.iter() {
                    if particles[i].0 == *compare_entity {
                        continue;
                    }

                    let dir = *compare_translation - particles[i].2;
                    let dist = dir.length();
                    let attract_modifier =
                        attraction_matrix[particles[i].1][*compare_type_id](dist);
                    if attract_modifier == 0.0 {
                        continue;
                    }
                    total_vel += (dir / dist) * attract_modifier * 0.1;
                }
                velocities.push(total_vel);
            }
            velocities
        }));

        cur_start += step;
        cur_stop += step;
    }

    let mut vels: Vec<Vec3> = Vec::with_capacity(num_particles);
    for handle in threads {
        vels.append(&mut handle.join().unwrap());
    }

    let mut i = 0;
    for (_, _, mut vel, _) in particles.iter_mut() {
        vel.linvel += vels[i];
        i += 1;
    }
}
