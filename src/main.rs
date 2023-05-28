use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use floating_cam::FloatingCamPlugin;
use particles::{particle_metadata::AttractionFunc, ParticlesPlugin};
use wall_bundles::{init_clear_box, init_opaque_box};
mod floating_cam;
mod particles;
mod wall_bundles;
fn main() {
    let zero1: AttractionFunc<f32> = |_| return 0.0;
    let reat1: AttractionFunc<f32> = |x| {
        // let a = 2.0;
        // let b = (3.0 * a) / 2.0;
        if x < 3.0 {
            return -(x - 2.0).abs() + 1.0;
        } else {
            return 0.1;
        }
    };
    let reat2: AttractionFunc<f32> = |x| {
        // let a = 0.5;
        // let b = (3.0 * a) / 2.0;
        if x < 1.5 {
            return -(x - 1.0).abs() + 0.5;
        } else {
            return 0.1;
        }
    };

    let repl1: AttractionFunc<f32> = |x| {
        // let a = 0.1;
        if x < 1.2 {
            return x - 1.2;
        } else {
            return 0.0;
        }
    };

    let repl2: AttractionFunc<f32> = |x| {
        // let a = 0.1;
        if x < 2.5 {
            return -0.1;
        } else {
            return 0.0;
        }
    };
    let attr1: AttractionFunc<f32> = |x| {
        // let a = 5.5;
        if x < 20.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };

    // let matrix = vec![vec![zero, attr], vec![attr, zero]];
    // let matrix = vec![
    //     vec![repl1, zero1, zero1, zero1, zero1, zero1],
    //     vec![reat1, reat2, repl1, repl1, repl1, repl1],
    //     vec![reat1, attr1, reat2, repl1, repl1, repl1],
    //     vec![reat1, repl1, attr1, reat2, repl1, repl1],
    //     vec![reat1, repl1, repl1, attr1, reat2, repl1],
    //     vec![reat1, repl1, repl1, repl1, attr1, reat2],
    // ];

    let matrix = vec![
        vec![repl2, zero1, attr1, zero1, zero1, zero1],
        vec![reat1, repl1, zero1, zero1, zero1, zero1],
        vec![zero1, zero1, repl2, zero1, attr1, zero1],
        vec![zero1, zero1, reat1, repl1, zero1, zero1],
        vec![attr1, zero1, zero1, zero1, repl2, zero1],
        vec![zero1, zero1, zero1, zero1, reat1, repl1],
    ];
    let colors = vec![
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::PURPLE,
        Color::YELLOW,
        Color::ORANGE,
    ];

    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "FPS Cam".into(),
                        resolution: (1900., 1280.).into(),
                        present_mode: PresentMode::AutoVsync,
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
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(FloatingCamPlugin)
        .add_plugin(ParticlesPlugin {
            min: Vec3 {
                x: -2.0,
                y: 2.0,
                z: -2.0,
            },
            max: Vec3 {
                x: 2.0,
                y: 6.0,
                z: 2.0,
            },
            seed: Some(42),
            // seed: None,
            attraction_matrix: matrix,
            // type_id_counts: particles::Count::Set(vec![2, 32, 2, 31, 2, 31]),
            type_id_counts: particles::Count::Random(500),
            // type_id_counts: Some(vec![2, 0, 2, 0, 2, 0]),
            colors: colors,
            radius: 0.05,
            lin_damping: 0.5,
        })
        // .add_startup_system(init_opaque_box)
        .add_startup_system(init_clear_box)
        .run();
}
