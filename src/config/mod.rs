use core::panic;

use crate::particles::Count;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Spawn {
    pub min: Vec3,
    pub max: Vec3,
    pub seed: Option<u64>,
    pub type_id_counts: Count,
    // pub colors: Vec<Color>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParticleProperties {
    pub radius: f32,
    pub lin_damping: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    spawn: Spawn,
    // particle_properties: ParticleProperties,
}

pub fn load_cfg() -> Config {
    let res: Result<Config, confy::ConfyError> = confy::load("settings.json", None);
    match res {
        Ok(cfg) => return cfg,
        Err(err) => panic!("Error loading config {:?}", err),
    }
}
