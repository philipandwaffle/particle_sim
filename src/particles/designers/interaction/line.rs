use bevy::prelude::*;

#[derive(Bundle)]
pub struct LineBundle {
    name: Name,
    line: DesignerLine,
    mat: MaterialMeshBundle<StandardMaterial>,
}
impl LineBundle {
    pub fn new(
        name: String,
        id: usize,
        from: Entity,
        to: Entity,
        thickness: f32,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        return Self {
            name: Name::new(name),
            line: DesignerLine::new(id, from, to),
            mat: MaterialMeshBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: thickness,
                        height: 1.0,
                        resolution: 6,
                        segments: 6,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                ..default()
            },
        };
    }
}
#[derive(Component)]
pub struct DesignerLine {
    pub id: usize,
    pub from: Entity,
    pub to: Entity,
}
impl DesignerLine {
    pub fn new(id: usize, from: Entity, to: Entity) -> Self {
        return Self { id, from, to };
    }
}
