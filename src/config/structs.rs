use crate::particles::Count;
use bevy::prelude::*;
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct CameraSettings {
    pub mouse_look_sen: f32,
    pub button_look_sen: f32,
    pub move_speed: f32,
}

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Spawn {
    pub min: Vec3,
    pub max: Vec3,
    pub seed: Option<u64>,
    pub type_id_counts: Count,
    pub colors: Vec<Color>,
}

#[derive(Resource, Debug, Default, Serialize, Deserialize, Clone)]
pub struct ParticleProperties {
    pub radius: f32,
    pub lin_damping: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub camera: CameraSettings,
    pub spawn: Spawn,
    pub particle_properties: ParticleProperties,
}
impl Config {
    pub fn insert_config_resources(&self, app: &mut App) {
        app.insert_resource(self.camera.clone())
            .insert_resource(self.spawn.clone())
            .insert_resource(self.particle_properties.clone());
    }
    pub fn load_cfg(path: &str) -> Config {
        let file = File::open(path);
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
}
