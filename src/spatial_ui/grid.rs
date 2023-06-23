use bevy::{
    math::vec3,
    prelude::{
        AssetServer, Assets, Bundle, Changed, Color, Commands, Component, Entity, IVec2, Mesh,
        Query, Res, StandardMaterial, Transform, UVec2, Vec2, Vec3,
    },
};

use super::{
    shaped_container::{ShapedContainer, ShapedContainerBundle},
    vertex_line::VertexLine,
    NavControlled, ReceiveNav,
};
use crate::floating_cam::control_state::NavDelta;

#[derive(Bundle)]
pub struct GridBundle {
    grid: Grid,
    transform: Transform,
}
impl GridBundle {
    pub fn new(
        dims: UVec2,
        translation: Vec3,
        scale: Vec3,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // calculate container scale and offset so that the containers use their centre as the anchor point
        let container_scale = scale / dims.extend(1).as_vec3();
        let container_offset = (translation + (scale / 2.0)) - (container_scale / 2.0);

        // Pre-allocate container and contents vec
        let height = dims.y as usize;
        let mut containers = Vec::with_capacity(height);

        // Init each container in the grid
        for i in 0..height {
            // Pre-allocate container and contents row
            let width = dims.x as usize;
            let mut container_row = Vec::with_capacity(width);
            for j in 0..width {
                // Calculate the containers position
                let container_translation = vec3(
                    scale.x * (j as f32 / width as f32),
                    scale.y * (i as f32 / height as f32),
                    container_offset.z * 1.5,
                ) - container_offset;

                //todo! Implement loading pre-made matrices
                // Create a vertex line
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
                // Spawn vertex line
                let vertex_line_entity = commands.spawn(vertex_line).id();

                // Spawn container with the vertex line inside it
                let container_entity = commands
                    .spawn(ShapedContainerBundle::new(
                        container_translation,
                        container_scale,
                        Color::rgba(i as f32, j as f32, 0.0, 0.1),
                        vertex_line_entity,
                        meshes,
                        materials,
                    ))
                    .id();

                container_row.push(container_entity);
            }
            containers.push(container_row);
        }
        // Reverse along the y-axis
        containers.reverse();

        return Self {
            grid: Grid {
                dims: dims,
                cur_edit: IVec2::ZERO,
                prev_edit: IVec2::ZERO,
                containers: containers,
                prev_delta: Vec2::ZERO,
                trickle: false,
                trickle_toggled: false,
            },
            transform: Transform::from_translation(translation),
        };
    }
}

#[derive(Component)]
pub struct Grid {
    pub dims: UVec2,
    pub cur_edit: IVec2,
    pub prev_edit: IVec2,
    pub containers: Vec<Vec<Entity>>,
    pub prev_delta: Vec2,
    pub trickle: bool,
    pub trickle_toggled: bool,
}
impl Grid {
    fn apply_primary_nav(&mut self, delta: Vec2) {
        // Check if the primary nav needs applied
        if self.trickle {
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
            || new_edit_point.x > self.dims.x as i32 - 1
            || new_edit_point.y < 0
            || new_edit_point.y > self.dims.y as i32 - 1
        {
            println!("invalid delta {:?} results in {:?}", delta, new_edit_point);
            return;
        }
        self.cur_edit = new_edit_point;
    }

    fn apply_primary_interact(&mut self, b: bool) {
        // Set flags if they need set
        if b && !self.trickle {
            self.trickle = true;
            self.trickle_toggled = true;
        }
    }

    fn apply_secondary_interact(&mut self, b: bool) {
        // Set flags if they need set
        if b && self.trickle {
            self.trickle = false;
            self.trickle_toggled = true;
        }
    }
}
impl NavControlled for Grid {
    fn trickle(&mut self, nav: NavDelta) {
        // Apply ui navigation
        self.apply_primary_nav(nav.primary_nav);

        self.apply_primary_interact(nav.primary_interact);
        self.apply_secondary_interact(nav.secondary_interact);
    }
}

pub fn update_grid_containers(
    mut commands: Commands,
    mut grids: Query<&mut Grid, Changed<Grid>>,
    mut containers: Query<&mut ShapedContainer>,
) {
    // Loop through each grid
    for mut grid in grids.iter_mut() {
        let cur = grid.cur_edit;
        let prev = grid.prev_edit;

        // Get current and previous container entity
        let cur_container_entity = grid.containers[cur.y as usize][cur.x as usize];
        let prev_container_entity = grid.containers[prev.y as usize][prev.x as usize];

        // Check the current edit point has changed
        if cur != prev {
            // Change the current container color
            let mut cur_container = containers.get_mut(cur_container_entity).unwrap();
            cur_container.color = Color::rgba(0.0, 1.0, 0.0, 0.1);

            // Change the previous container color
            let mut prev_container = containers.get_mut(prev_container_entity).unwrap();
            prev_container.color = Color::rgba(1.0, 0.0, 0.0, 0.1);

            // Update previous edit point
            grid.prev_edit = grid.cur_edit;
        }

        // Check if the grid's trickle has been toggled
        if grid.trickle_toggled {
            // Reset toggled
            grid.trickle_toggled = false;

            // Get current and previous container content
            let cur_content = containers.get(cur_container_entity).unwrap().content;
            let prev_content = containers.get(prev_container_entity).unwrap().content;

            // Manage the component marking entities that can receive nav control
            commands.entity(prev_content).remove::<ReceiveNav>();
            commands.entity(cur_content).insert(ReceiveNav);
        }
    }
}
