use bevy::prelude::{
    AssetServer, Assets, Commands, Entity, Mesh, Res, ResMut, Resource, StandardMaterial, UVec2,
    Vec3,
};

use super::{grid::GridBundle, vertex_line::VertexLineBundle, ReceiveNav};

#[derive(Resource)]
pub struct SpawnList {
    pub spawn: Vec<UIType>,
}

pub enum UIType {
    Grid {
        controllable: bool,
        translation: Vec3,
        scale: Vec3,
        dims: UVec2,
    },
    VertexLine {
        controllable: bool,
        translation: Vec3,
        scale: Vec3,
        vertices: usize,
        vertex_radius: f32,
        line_thickness: f32,
    },
}
impl UIType {
    pub fn spawn_vessel(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Entity {
        match self {
            UIType::Grid {
                controllable,
                translation,
                scale,
                dims,
            } => {
                let grid = GridBundle::new(
                    dims.clone(),
                    translation.clone(),
                    scale.clone(),
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                if controllable.clone() {
                    return commands.spawn((grid, ReceiveNav)).id();
                } else {
                    return commands.spawn(grid).id();
                }
            }
            UIType::VertexLine {
                controllable,
                translation,
                scale,
                vertices,
                vertex_radius,
                line_thickness,
            } => {
                let vertex_line = VertexLineBundle::new(
                    vertices.clone(),
                    translation.clone(),
                    scale.clone(),
                    vertex_radius.clone(),
                    line_thickness.clone(),
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );

                if controllable.clone() {
                    return commands.spawn((vertex_line, ReceiveNav)).id();
                } else {
                    return commands.spawn(vertex_line).id();
                }
            }
        }
    }
}

pub fn spawn_ui(
    mut sp: ResMut<SpawnList>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ui in sp.spawn.iter() {
        ui.spawn_vessel(&mut commands, &asset_server, &mut meshes, &mut materials);
    }
    sp.spawn.clear();
}
