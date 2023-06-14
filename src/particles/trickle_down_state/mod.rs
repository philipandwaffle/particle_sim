use bevy::prelude::*;

use self::root::Dreg;

use super::designers::{
    designer::Designer, interaction::interaction_designer::InteractionDesigner,
};
pub mod root;

#[bevy_trait_query::queryable]
pub trait Trickles {
    fn drip(&mut self, dreg: Dreg);
    fn peek(&self) -> &Dreg;
}

pub enum Contents {
    Vessel(Box<dyn Trickles + Send + Sync>),
}
impl Trickles for Contents {
    fn drip(&mut self, dreg: Dreg) {
        todo!()
    }

    fn peek(&self) -> &Dreg {
        todo!()
    }
}

#[derive(Component)]
pub struct Grid {
    cur_edit: UVec2,
    data: Vec<Vec<Box<dyn Trickles + Send + Sync>>>,
    is_drippy: bool,
}
impl Grid {
    fn apply_primary_nav_delta(&mut self, delta: Vec2) {
        if self.edit {
            return;
        }

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
        if self.edit {
            return;
        }
    }

    fn apply_primary_interact(&mut self, _: bool) {
        self.edit = true;
    }

    fn apply_secondary_interact(&mut self, _: bool) {
        self.edit = false;
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
        if self.is_drippy {
            (self.data[self.cur_edit.y as usize][self.cur_edit.x as usize]).drip(dreg);
        }
    }

    fn peek(&self) -> &Dreg {
        todo!()
    }
}
