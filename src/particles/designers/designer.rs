use bevy::{math::vec3, prelude::*};

use crate::particles::designers::matrix::matrix_designer::MatrixDesigner;

use super::interaction::interaction_designer::InteractionDesigner;

pub struct RegisterTraitPlugin;
impl Plugin for RegisterTraitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn Designer, InteractionDesigner>()
            .register_component_as::<dyn Designer, MatrixDesigner>();
    }
}

#[bevy_trait_query::queryable]
pub trait Designer {
    fn apply_primary_nav_delta(&mut self, delta: Vec2);
    fn apply_secondary_nav_delta(&mut self, delta: isize);
    fn apply_primary_interact(&mut self, interact: bool);
    fn apply_secondary_interact(&mut self, interact: bool);
    fn spawn(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Entity;
    fn despawn(&self, commands: &mut Commands);
}

impl Designer for InteractionDesigner {
    fn apply_primary_nav_delta(&mut self, delta: Vec2) {
        if self.cur_point_id == -1 || self.point_positions.is_empty() {
            return;
        }
        //todo! better error handling and logging
        self.point_positions[self.cur_point_id as usize].x += delta.x;
        self.point_positions[self.cur_point_id as usize].y += delta.y;
    }

    fn apply_secondary_nav_delta(&mut self, delta: isize) {
        if delta == 0 {
            return;
        }

        // Aggregate current id with delta id
        self.cur_point_id += delta;

        // Check if new id is out of bounds and fix
        if self.cur_point_id == -1 {
            self.cur_point_id = self.num_points as isize - 1;
        } else if self.cur_point_id == self.num_points as isize {
            self.cur_point_id = 0;
        }
    }

    fn apply_primary_interact(&mut self, _: bool) {
        return;
    }

    fn apply_secondary_interact(&mut self, _: bool) {
        return;
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

impl Designer for MatrixDesigner {
    fn apply_primary_nav_delta(&mut self, delta: Vec2) {
        // Filter out input that is continuous
        if self.prev_delta != Vec2::ZERO {
            self.prev_delta = delta;
            return;
        }
        self.prev_delta = delta;

        // if !self.needs_update(){
        //     return;
        // }

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
        let num_particles = self.num_particles as i32;
        let new_edit_point = self.cur_edit_point + delta;
        if new_edit_point.x < 0
            || new_edit_point.x > num_particles - 1
            || new_edit_point.y < 0
            || new_edit_point.y > num_particles - 1
        {
            println!("invalid delta {:?} results in {:?}", delta, new_edit_point);
            return;
        }

        self.prev_edit_point = self.prev_edit_point;
        self.cur_edit_point += delta;
    }

    fn apply_secondary_nav_delta(&mut self, _: isize) {
        return;
    }

    fn apply_primary_interact(&mut self, _: bool) {
        return;
    }

    fn apply_secondary_interact(&mut self, _: bool) {
        return;
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
