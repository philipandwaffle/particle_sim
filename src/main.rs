use std::arch::x86_64::_mm256_zeroupper;
mod register_trait;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use config::structs::*;
use floating_cam::FloatingCamPlugin;
use particles::ParticlesPlugin;
use wall_bundles::{init_clear_box, init_opaque_box};

mod config;
mod floating_cam;
mod particles;
mod wall_bundles;

fn main() {
    let edit_mode = false;
    let profiling_mode = false;

    let cfg = Config::load_cfg("settings.json");

    let mut app = App::new();
    app.insert_resource(RapierConfiguration {
        gravity: Vec3::ZERO,
        ..default()
    })
    .insert_resource(cfg.camera)
    .insert_resource(cfg.spawn)
    .insert_resource(cfg.particle_properties)
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Particle Sim".into(),
                    resolution: (1900., 1280.).into(),
                    // present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::BorderlessFullscreen,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            // don't use linear sampling as image textures will be blurry
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(FloatingCamPlugin);

    if profiling_mode {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }

    if edit_mode {
        // app.add_plugin(InteractionDesignerPlugin);
    } else {
        app.add_plugin(ParticlesPlugin);
    }
    // app.add_startup_system(init_opaque_box);
    app.add_startup_system(init_clear_box);

    app.run();
}
