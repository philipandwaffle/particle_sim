use bevy::prelude::*;

use super::{grid::GridBundle, vertex_line::VertexLineBundle};

pub enum VesselType {
    Grid((Vec3, Vec3, UVec2)),
    VertexLine((Vec3, Vec3, usize, f32, f32)),
}
impl VesselType {
    pub fn spawn_vessel(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Entity {
        match self {
            VesselType::Grid((translation, scale, dims)) => {
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
            VesselType::VertexLine((
                translation,
                scale,
                vertices,
                vertex_radius,
                line_thickness,
            )) => {
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
