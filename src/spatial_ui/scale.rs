use bevy::{
    prelude::{
        default, shape, AlphaMode, Assets, Bundle, Color, Component, MaterialMeshBundle, Mesh,
        StandardMaterial, Transform, Vec3,
    },
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Bundle)]
pub struct ScaleBundle {
    pub scale: Scale,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl ScaleBundle {
    pub fn new(
        translation: Vec3,
        scale: Vec3,
        start: i32,
        stop: i32,
        notches: u32,
        color: Color,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        println!("creating scale");
        let mesh = ScaleBundle::create_mesh(start, stop, notches);
        return Self {
            scale: Scale::new(start, stop, notches),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform {
                    translation,
                    scale,
                    ..default()
                },
                ..default()
            },
        };
    }

    fn create_mesh(start: i32, stop: i32, notches: u32) -> Mesh {
        println!("creating scale mesh");
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices = vec![
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ];
        let num_vertices = vertices.len();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(
            (0..num_vertices).map(|x| x as u32).collect(),
        )));
        mesh
    }
}

#[derive(Component)]
pub struct Scale {
    pub start: i32,
    pub stop: i32,
    pub notches: u32,
}
impl Scale {
    pub fn new(start: i32, stop: i32, notches: u32) -> Self {
        return Self {
            start,
            stop,
            notches,
        };
    }
}
