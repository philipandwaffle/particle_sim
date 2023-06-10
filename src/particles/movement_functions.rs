use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{matrix::Matrix, particle::Particle};

#[allow(dead_code)]
pub fn move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    matrix: Res<Matrix>,
) {
    // let matrix = &interaction_matrix.matrix;
    let compare_vec = particles
        .iter()
        .map(|x| (x.0, x.1.type_id, x.3.translation))
        .collect::<Vec<(Entity, usize, Vec3)>>();

    for (entity, particle, mut velocity, transform) in particles.iter_mut() {
        let mut total_vel = Vec3::ZERO;
        for (compare_entity, compare_type_id, compare_translation) in compare_vec.iter() {
            if entity == *compare_entity {
                continue;
            }
            let dir = *compare_translation - transform.translation;
            let dist = dir.length();
            let attract_modifier = matrix.get_interaction(particle.type_id, *compare_type_id, dist);
            if attract_modifier == 0.0 {
                continue;
            }
            total_vel += (dir / dist) * attract_modifier * 0.1;
        }
        velocity.linvel += total_vel;
    }
}
/*
#[allow(dead_code)]
pub fn parallel_move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    particle_metadata: Res<ParticleMetadata>,
) {
    let compare_vec = particles
        .iter()
        .map(|(e, p, _, t)| (e, p.type_id, t.translation))
        .collect::<Vec<(Entity, usize, Vec3)>>();

    let num_threads = 6;
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
*/

/*
// #[allow(dead_code)]
pub fn parallel_arc_move_particles(
    mut particles: Query<(Entity, &Particle, &mut Velocity, &Transform)>,
    // particle_metadata: Res<ParticleMetadata>,
    interaction_matrix: Res<InteractionMatrix>,
) {
    if !interaction_matrix.is_added() {
        return;
    }

    let compare_vec = Arc::new(
        particles
            .iter()
            .map(|(e, p, _, t)| (e, p.type_id, t.translation))
            .collect::<Vec<(Entity, usize, Vec3)>>(),
    );

    let num_threads = 16;
    let num_particles = compare_vec.len();

    let mut threads = Vec::with_capacity(num_threads);

    let mut cur_start = 0;
    let step = num_particles / num_threads;
    let mut cur_stop = step;

    println!("Starting thread alloc");
    for i in 0..num_threads {
        if i == num_threads - 1 {
            cur_stop = num_particles;
        }

        let start = cur_start.clone();
        let stop = cur_stop.clone();

        let interaction_matrix = Arc::clone(&interaction_matrix.matrix);
        let compare_vec = Arc::clone(&compare_vec);

        println!("Pushing new thread");
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
                        (interaction_matrix[particles[i].1][*compare_type_id]).interact(dist);
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
    println!("Joining threads");
    for handle in threads {
        vels.append(&mut handle.join().unwrap());
        println!("Thread joined");
    }

    let mut i = 0;
    for (_, _, mut vel, _) in particles.iter_mut() {
        vel.linvel += vels[i];
        i += 1;
    }
}
*/
