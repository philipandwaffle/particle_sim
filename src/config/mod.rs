use core::panic;
use std::fs::File;
use std::io::BufReader;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::particles::Count;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Spawn {
    pub min: Vec3,
    pub max: Vec3,
    pub seed: Option<u64>,
    pub type_id_counts: Count,
    pub colors: Vec<Color>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParticleProperties {
    pub radius: f32,
    pub lin_damping: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    spawn: Spawn,
    particle_properties: ParticleProperties,
}

pub fn load_cfg() -> Config {
    let file = File::open("settings.json");
    if let Err(err) = file {
        panic!("Error opening file {:?}", err);
    }

    let reader = BufReader::new(file.unwrap());

    let json: Result<Config, serde_json::Error> = serde_json::from_reader(reader);
    match json {
        Ok(cfg) => return cfg,
        Err(err) => panic!("Error reading JSON {:?}", err),
    }
}
