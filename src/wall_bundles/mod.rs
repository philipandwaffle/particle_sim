use self::bundles::{ClearWallBundle, WallBundle};
use bevy::{math::vec3, prelude::*};
mod bundles;

#[allow(dead_code)]
pub fn init_opaque_box(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // ground
    commands.spawn(WallBundle::new(
        "Bottom".into(),
        vec3(20.0, 0.5, 20.0),
        vec3(0.0, -10.0, 0.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        "top".into(),
        vec3(20.0, 0.5, 20.0),
        vec3(0.0, 10.0, 0.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        "left".into(),
        vec3(0.5, 20.0, 20.0),
        vec3(-10.0, 0.0, 0.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        "right".into(),
        vec3(0.5, 20.0, 20.0),
        vec3(10.0, 0.0, 0.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        "front".into(),
        vec3(20.0, 20.0, 0.5),
        vec3(0.0, 0.0, -10.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        "back".into(),
        vec3(20.0, 20.0, 0.5),
        vec3(0.0, 0.0, 10.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));

    commands.spawn(WallBundle::new(
        "datum".into(),
        vec3(20.0, 20.0, 0.5),
        vec3(0.0, 0.0, -7.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));

    // LET THERE BE LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

#[allow(dead_code)]
pub fn init_clear_box(mut commands: Commands) {
    // ground
    commands.spawn(ClearWallBundle::new(
        "Bottom".into(),
        vec3(20.0, 0.5, 20.0),
        vec3(0.0, -10.0, 0.0),
    ));
    commands.spawn(ClearWallBundle::new(
        "top".into(),
        vec3(20.0, 0.5, 20.0),
        vec3(0.0, 10.0, 0.0),
    ));
    commands.spawn(ClearWallBundle::new(
        "left".into(),
        vec3(0.5, 20.0, 20.0),
        vec3(-10.0, 0.0, 0.0),
    ));
    commands.spawn(ClearWallBundle::new(
        "right".into(),
        vec3(0.5, 20.0, 20.0),
        vec3(10.0, 0.0, 0.0),
    ));
    commands.spawn(ClearWallBundle::new(
        "front".into(),
        vec3(20.0, 20.0, 0.5),
        vec3(0.0, 0.0, -10.0),
    ));
    commands.spawn(ClearWallBundle::new(
        "back".into(),
        vec3(20.0, 20.0, 0.5),
        vec3(0.0, 0.0, 10.0),
    ));

    // LET THERE BE LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
