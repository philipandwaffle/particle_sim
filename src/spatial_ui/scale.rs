use bevy::{
    a11y::accesskit::Vec2,
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
        let mesh = ScaleBundle::create_mesh(notches);
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

    fn create_mesh(notches: u32) -> Mesh {
        println!("creating scale mesh");

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let notches = 10;
        let notch_thickness = 0.01;
        let notch_height = 0.05;
        let gap_width = (1.0 - (notch_thickness * notches as f32)) / (notches as f32 - 1.0);
        let gap_height = 0.02;
        let mut vertices = vec![];

        let mut x = 0.0;

        ScaleBundle::add_rectangle(
            &mut vertices,
            &[x, notch_height],
            &[x + notch_thickness, -notch_height],
        );
        x += notch_thickness;

        for _ in 0..notches - 1 {
            ScaleBundle::add_rectangle(
                &mut vertices,
                &[x, gap_height],
                &[x + gap_width, -gap_height],
            );
            x += gap_width;

            ScaleBundle::add_rectangle(
                &mut vertices,
                &[x, notch_height],
                &[x + notch_thickness, -notch_height],
            );
            x += notch_thickness;
        }

        let vertices = vertices
            .iter()
            .map(|x| [x[0] - 0.5, x[1], x[2]])
            .collect::<Vec<[f32; 3]>>();
        let num_vertices = vertices.len();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(
            (0..num_vertices).map(|x| x as u32).collect(),
        )));
        mesh
    }

    fn add_rectangle(vertices: &mut Vec<[f32; 3]>, tl: &[f32; 2], br: &[f32; 2]) {
        vertices.push([br[0], br[1], 0.0]);
        vertices.push([tl[0], tl[1], 0.0]);
        vertices.push([tl[0], br[1], 0.0]);
        vertices.push([br[0], br[1], 0.0]);
        vertices.push([br[0], tl[1], 0.0]);
        vertices.push([tl[0], tl[1], 0.0]);
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
