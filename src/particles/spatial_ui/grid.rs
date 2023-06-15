use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::egui::containers;

use super::{root::Dreg, shaped_container::ShapedContainer, Trickles};
use crate::particles::{
    designers::interaction::interaction_designer::InteractionDesigner,
    spatial_ui::shaped_container::ShapedContainerBundle,
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
        // let mut cur_spawn = matrix_designer_state.centre
        let mut cell_scale = scale / vec3(width as f32, height as f32, 1.0);

        let mut containers = Vec::with_capacity(height);
        let cell_offset = (translation + (scale / 2.0)) - (cell_scale / 2.0);
        for i in 0..height {
            let mut row = Vec::with_capacity(width);
            for j in 0..width {
                let cell_translation = vec3(
                    scale.x * (i as f32 / width as f32),
                    scale.y * (j as f32 / height as f32),
                    cell_offset.z * 1.5,
                ) - cell_offset;

                let cell = commands
                    .spawn(ShapedContainerBundle::new(
                        cell_translation,
                        cell_scale,
                        Color::rgba(i as f32, j as f32, 0.0, 0.1),
                        meshes,
                        materials,
                    ))
                    .id();

                //todo! Implement loading pre-made matrices
                let cell_designer = InteractionDesigner::new(
                    5,
                    cell_translation,
                    cell_scale,
                    0.01,
                    0.005,
                    commands,
                    asset_server,
                    meshes,
                    materials,
                );
                let value_entity = commands.spawn(cell_designer).id();

                row.push(cell);
            }
            containers.push(row);
        }

        return Self {
            grid: Grid {
                width: width,
                height: height,
                cur_edit: IVec2::ZERO,
                prev_edit: IVec2::ZERO,
                data: (),
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
    data: Vec<Vec<Box<dyn Trickles + Send + Sync>>>,
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

    fn spawn(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Entity {
        let designer_bundle = InteractionDesigner::new(
            5,
            vec3(0.0, 0.0, -0.5),
            vec3(5.0, 5.0, 0.0),
            0.5,
            0.05,
            commands,
            asset_server,
            meshes,
            materials,
        );
        let entity = commands.spawn(designer_bundle).id();
        return entity;
    }

    fn despawn(&self, commands: &mut Commands) {
        todo!()
    }
}
impl Trickles for Grid {
    fn drip(&mut self, dreg: Dreg) {
        if self.consuming {
            (self.data[self.cur_edit.y as usize][self.cur_edit.x as usize]).drip(dreg);
        }
    }

    fn peek(&self) -> &Dreg {
        todo!()
    }
}

fn update_grid_containers(grids: Query<&Grid>, mut containers: Query<&mut ShapedContainer>) {
    for grid in grids.iter() {}
}