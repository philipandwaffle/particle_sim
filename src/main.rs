use std::arch::x86_64::_mm256_zeroupper;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec3,
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use config::load_cfg;
use floating_cam::FloatingCamPlugin;
use particles::{particle_metadata::AttractionFunc, ParticlesPlugin};
use wall_bundles::{init_clear_box, init_opaque_box};

use crate::particles::attraction_functions::get_fns;

mod config;
mod floating_cam;
mod particles;
mod wall_bundles;

fn development() {
    trait Animal: AnimalClone {
        fn speak(&self);
    }

    // Splitting AnimalClone into its own trait allows us to provide a blanket
    // implementation for all compatible types, without having to implement the
    // rest of Animal.  In this case, we implement it for all types that have
    // 'static lifetime (*i.e.* they don't contain non-'static pointers), and
    // implement both Animal and Clone.  Don't ask me how the compiler resolves
    // implementing AnimalClone for dyn Animal when Animal requires AnimalClone;
    // I have *no* idea why this works.
    trait AnimalClone {
        fn clone_box(&self) -> Box<dyn Animal>;
    }

    impl<T> AnimalClone for T
    where
        T: 'static + Animal + Clone,
    {
        fn clone_box(&self) -> Box<dyn Animal> {
            Box::new(self.clone())
        }
    }

    // We can now implement Clone manually by forwarding to clone_box.
    impl Clone for Box<dyn Animal> {
        fn clone(&self) -> Box<dyn Animal> {
            self.clone_box()
        }
    }

    #[derive(Clone)]
    struct Dog {
        name: String,
    }

    impl Dog {
        fn new(name: &str) -> Dog {
            Dog {
                name: name.to_string(),
            }
        }
    }

    impl Animal for Dog {
        fn speak(&self) {
            println!("{}: ruff, ruff!", self.name);
        }
    }

    #[derive(Clone)]
    struct AnimalHouse {
        animal: Box<dyn Animal>,
    }

    let house = AnimalHouse {
        animal: Box::new(Dog::new("Bobby")),
    };
    let house2 = house.clone();
    house2.animal.speak();
}

fn main() {
    let zero1: AttractionFunc = |_| return 0.0;
    let reat1: AttractionFunc = |x| {
        // let a = 2.0;
        // let b = (3.0 * a) / 2.0;
        if x < 3.0 {
            return -(x - 2.0).abs() + 1.0;
        } else {
            return 0.1;
        }
    };
    let reat2: AttractionFunc = |x| {
        // let a = 0.5;
        // let b = (3.0 * a) / 2.0;
        if x < 1.5 {
            return -(x - 1.0).abs() + 0.5;
        } else if x < 2.5 {
            return 0.1;
        } else {
            return 0.0;
        }
    };
    let reat3: AttractionFunc = |x| {
        // let a = 0.5;
        // let b = (3.0 * a) / 2.0;
        if x < 0.25 {
            return -(x - 0.375).abs() + 0.125;
        } else if x < 0.75 {
            return 0.1;
        } else {
            return 0.0;
        }
    };

    let repl1: AttractionFunc = |x| {
        // let a = 0.1;
        if x < 1.2 {
            return x - 1.2;
        } else {
            return 0.0;
        }
    };

    let repl2: AttractionFunc = |x| {
        // let a = 0.1;
        if x < 2.0 {
            return -0.1;
        } else {
            return 0.0;
        }
    };
    let attr1: AttractionFunc = |x| {
        // let a = 5.5;
        if x < 20.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };

    let cfg = load_cfg();
    println!("{:?}", cfg);

    let map = get_fns();
    let mut matrix = vec![];

    for row in cfg.particle_properties.attraction_matrix {
        let mut r = vec![];
        for cell in row {
            r.push(map[&cell]);
        }
        matrix.push(r);
    }

    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
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
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(FloatingCamPlugin)
        .add_plugin(ParticlesPlugin {
            min: cfg.spawn.min,
            max: cfg.spawn.max,
            seed: cfg.spawn.seed,
            attraction_matrix: matrix,
            type_id_counts: cfg.spawn.type_id_counts,
            colors: cfg.spawn.colors,
            radius: cfg.particle_properties.radius,
            lin_damping: cfg.particle_properties.lin_damping,
        })
        // .add_startup_system(init_opaque_box)
        .add_startup_system(init_clear_box)
        .run();
}
