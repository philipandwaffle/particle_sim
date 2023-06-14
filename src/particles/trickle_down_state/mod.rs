use bevy::prelude::*;

use self::root::Dreg;

use super::designers::{
    designer::Designer, interaction::interaction_designer::InteractionDesigner,
};
pub mod root;

#[bevy_trait_query::queryable]
pub trait Trickles {
    fn drip(&mut self, dreg: Dreg) {}
}

pub enum Contents {
    Consumer(Box<dyn Designer + Send + Sync>),
}
impl Trickles for Contents {}

#[derive(Component)]
pub struct Grid {}
impl Trickles for Grid {}
