use bevy::prelude::Commands;

use self::matrix::InteractionMatrix;

mod interaction;
pub mod matrix;

pub fn development() {
    interaction::development();
}

pub fn insert_interaction_matrix(mut commands: Commands) {
    // let im = InteractionMatrix::new(6);

    // commands.insert_resource(im);
}
