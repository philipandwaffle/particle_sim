use bevy::{ecs::component, math::vec3, prelude::*, transform::commands};

use self::bundles::MovablePointBundle;

mod bundles;
pub struct DesignerPlugin;
impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_design_room);
    }
}

fn spawn_design_room(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MovablePointBundle::new(
        "Movable Point".into(),
        Vec3::ONE,
        vec3(0.0, 20.0, 0.0),
        &asset_server,
        &mut meshes,
        &mut materials,
    ));
}
