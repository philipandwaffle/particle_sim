use bevy::prelude::*;

#[derive(Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl CellBundle {
    pub fn new(
        id: usize,
        transform: Transform,
        color: Color,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            cell: Cell::new(id),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(shape::Cube { size: 1.0 }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                }),
                transform: transform,
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct Cell {
    pub id: usize,
}
impl Cell {
    pub fn new(id: usize) -> Self {
        return Self { id: id };
    }
}
