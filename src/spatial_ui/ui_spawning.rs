use bevy::prelude::{
    AssetServer, Assets, Commands, Entity, Mesh, Res, ResMut, Resource, StandardMaterial, UVec2,
    Vec3,
};

use super::{grid::GridBundle, scale::Scale, vertex_line::VertexLineBundle, ReceiveNav};

#[derive(Resource)]
pub struct SpawnList {
    pub spawn: Vec<UIElement>,
}

pub enum UIElement {
    Grid {
        controllable: bool,
        translation: Vec3,
        scale: Vec3,
        scale_meta: Scale,
        dims: UVec2,
    },
    VertexLine {
        controllable: bool,
        translation: Vec3,
        scale: Vec3,
        scale_meta: Scale,
        vertices: usize,
        vertex_radius: f32,
        line_thickness: f32,
    },
}
impl UIElement {
    pub fn spawn_element(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Entity {
        match self {
            UIElement::Grid {
                controllable,
                translation,
                scale,
                scale_meta,
                dims,
            } => {
                let grid = GridBundle::new(
                    dims.clone(),
                    translation.clone(),
                    scale.clone(),
                    scale_meta.clone(),
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
            UIElement::VertexLine {
                controllable,
                translation,
                scale,
                scale_meta,
                vertices,
                vertex_radius,
                line_thickness,
            } => {
                let vertex_line = VertexLineBundle::new(
                    vertices.clone(),
                    translation.clone(),
                    scale.clone(),
                    scale_meta.clone(),
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

// Spawn the initial UI
pub fn spawn_ui(
    mut sp: ResMut<SpawnList>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Loop through each UI element to spawn
    for ui in sp.spawn.iter() {
        // Spawn UI
        ui.spawn_element(&mut commands, &asset_server, &mut meshes, &mut materials);
    }

    // Clear spawn list
    sp.spawn.clear();
}
