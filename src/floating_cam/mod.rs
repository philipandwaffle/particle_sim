use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::{
    camera::{transform_camera, FloatingCam, PlayerState},
    controls::ControlPlugin,
};

mod camera;
pub mod controls;

pub struct FloatingCamPlugin;
impl Plugin for FloatingCamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerState::default())
            .add_startup_system(spawn_floating_cam)
            .add_plugin(ControlPlugin)
            .add_system(transform_camera);
    }
}

fn spawn_floating_cam(mut commands: Commands) {
    commands
        .spawn(Name::new("Player"))
        .insert(FloatingCam)
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 0.99,
            angular_damping: 1.0,
        })
        .insert(Camera3dBundle {
            transform: Transform { ..default() },
            ..default()
        });
}
