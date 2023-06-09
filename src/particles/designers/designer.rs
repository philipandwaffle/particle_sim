use bevy::{math::vec3, prelude::*};

use super::interaction::interaction_designer::InteractionDesigner;

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
    );
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
    ) {
        let designer_bundle = InteractionDesigner::new(
            vec3(0.0, 0.0, -0.5),
            vec3(5.0, 5.0, 0.0),
            5,
            0.5,
            commands,
            asset_server,
            meshes,
            materials,
        );
        commands.spawn(designer_bundle);
    }

    fn despawn(&self, commands: &mut Commands) {
        todo!()
    }
}
