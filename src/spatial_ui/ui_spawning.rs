use bevy::prelude::*;

use super::{grid::GridBundle, vertex_line::VertexLineBundle};

#[derive(Resource)]
pub struct SpawnList {
    pub spawn: Vec<UIType>,
}

pub enum UIType {
    Grid {
        translation: Vec3,
        scale: Vec3,
        dims: UVec2,
    },
    VertexLine {
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
                translation,
                scale,
                dims,
            } => {
                let grid = GridBundle::new(
                    dims.x as usize,
                    dims.y as usize,
                    translation.clone(),
                    scale.clone(),
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                return commands.spawn(grid).id();
            }
            UIType::VertexLine {
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
                return commands.spawn(vertex_line).id();
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
