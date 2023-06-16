use std::cell::RefCell;

use bevy::{math::vec3, prelude::*};
use bevy_trait_query::One;

use self::root::Dreg;

use super::designers::{
    designer::Designer, interaction::interaction_designer::InteractionDesigner,
};
pub mod grid;
pub mod root;
mod shaped_container;
mod vertex_line;

#[bevy_trait_query::queryable]
pub trait Trickles {
    fn drip(&mut self, vessels: &RefCell<Query<One<&mut dyn Trickles>>>, dreg: Dreg);
}

pub enum Contents {
    Vessel(Box<dyn Trickles + Send + Sync>),
}
impl Trickles for Contents {
    fn drip(&mut self, vessels: &RefCell<Query<One<&mut dyn Trickles>>>, dreg: Dreg) {
        todo!()
    }
}
