use std::cell::RefCell;

use bevy::{math::vec3, prelude::*};
use bevy_trait_query::One;

use super::{
    root::Dreg,
    shaped_container::{ShapedContainer, ShapedContainerBundle},
    vertex_line::VertexLine,
    Trickles,
};

#[derive(Bundle)]
pub struct GridBundle {
    grid: Grid,
    transform: Transform,
}
impl GridBundle {
    pub fn new(
        width: usize,
        height: usize,
        translation: Vec3,
        scale: Vec3,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // calculate container scale and offset so that the containers use their centre as the anchor point
        let container_scale = scale / vec3(width as f32, height as f32, 1.0);
        let container_offset = (translation + (scale / 2.0)) - (container_scale / 2.0);

        // Pre-allocate container and contents vec
        let mut containers = Vec::with_capacity(height);
        let mut contents = Vec::with_capacity(height);

        // Init each container in the grid
        for i in 0..height {
            // Pre-allocate container and contents row
            let mut container_row = Vec::with_capacity(width);
            let mut contents_row = Vec::with_capacity(width);
            for j in 0..width {
                // Calculate the containers position
                let container_translation = vec3(
                    scale.x * (i as f32 / width as f32),
                    scale.y * (j as f32 / height as f32),
                    container_offset.z * 1.5,
                ) - container_offset;

                // Spawn container
                let container = commands
                    .spawn(ShapedContainerBundle::new(
                        container_translation,
                        container_scale,
                        Color::rgba(i as f32, j as f32, 0.0, 0.1),
                        meshes,
                        materials,
                    ))
                    .id();

                //todo! Implement loading pre-made matrices
                let vertex_line = VertexLine::new(
                    5,
                    container_translation,
                    container_scale,
                    0.01,
                    0.005,
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                let vessel = commands.spawn(vertex_line).id();

                container_row.push(container);
                contents_row.push(vessel);
            }
            containers.push(container_row);
            contents.push(contents_row);
        }

        return Self {
            grid: Grid {
                width: width,
                height: height,
                cur_edit: IVec2::ZERO,
                prev_edit: IVec2::ZERO,
                contents,
                containers: containers,
                prev_delta: Vec2::ZERO,
                consuming: false,
            },
            transform: Transform::from_translation(translation),
        };
    }
}

#[derive(Component)]
pub struct Grid {
    width: usize,
    height: usize,
    cur_edit: IVec2,
    prev_edit: IVec2,
    contents: Vec<Vec<Entity>>,
    containers: Vec<Vec<Entity>>,
    prev_delta: Vec2,
    consuming: bool,
}
impl Grid {
    fn apply_primary_nav_delta(&mut self, delta: Vec2) {
        if self.consuming {
            return;
        }

        // Filter out input that is continuous
        if self.prev_delta != Vec2::ZERO {
            self.prev_delta = delta;
            return;
        }
        self.prev_delta = delta;

        // Normalise delta so each component is either -1, 0 or 1
        let normalise = |x: f32| {
            if x > 0.0 {
                return 1;
            } else if x < 0.0 {
                return -1;
            } else {
                return 0;
            }
        };
        let delta = IVec2::new(normalise(delta.x), -normalise(delta.y));

        // Stop if there is no delta to apply
        if delta == IVec2::ZERO {
            return;
        }

        // Stop of delta results in out of bounds
        let new_edit_point = self.cur_edit + delta;
        if new_edit_point.x < 0
            || new_edit_point.x > self.width as i32 - 1
            || new_edit_point.y < 0
            || new_edit_point.y > self.height as i32 - 1
        {
            println!("invalid delta {:?} results in {:?}", delta, new_edit_point);
            return;
        }
        self.prev_edit = self.cur_edit;
        self.cur_edit += new_edit_point;
    }

    fn apply_secondary_nav_delta(&mut self, _: isize) {
        todo!();
    }

    fn apply_primary_interact(&mut self, _: bool) {
        todo!();
    }

    fn apply_secondary_interact(&mut self, _: bool) {
        todo!();
    }

    fn despawn(&self, commands: &mut Commands) {
        todo!()
    }
}
impl Trickles for Grid {
    fn drip(&mut self, vessels: &RefCell<Query<One<&mut dyn Trickles>>>, dreg: Dreg) {
        if !self.consuming {
            let vessel_entity = self.contents[self.cur_edit.y as usize][self.cur_edit.x as usize];
            let mut binding = vessels.borrow_mut();
            if let Ok(mut v) = binding.get_mut(vessel_entity) {
                v.drip(vessels, dreg);
            } else {
                panic!("Grid panic");
            };
        } else {
            self.apply_primary_nav_delta(dreg.primary_nav);
        }
    }
}

pub fn update_grid_containers(grids: Query<&Grid>, mut containers: Query<&mut ShapedContainer>) {
    for grid in grids.iter() {}
}
