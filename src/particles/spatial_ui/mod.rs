use bevy::{math::vec3, prelude::*};

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
