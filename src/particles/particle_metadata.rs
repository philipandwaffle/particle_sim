use bevy::prelude::*;

pub type AttractionFunc<F> = fn(f: F) -> f32;

#[derive(Resource)]
pub struct ParticleMetadata {
    pub min: Vec3,
    pub max: Vec3,
    pub attraction_matrix: Vec<Vec<AttractionFunc<f32>>>,
    pub colors: Vec<Color>,
}

impl ParticleMetadata {
    pub fn new(
        min: Vec3,
        max: Vec3,
        attraction_matrix: Vec<Vec<AttractionFunc<f32>>>,
        colors: Vec<Color>,
    ) -> Self {
        let height = attraction_matrix.len();

        if colors.len() != height {
            panic!("Colors is malformed {:?}.len != {}", colors, height);
        }

        for row in attraction_matrix.iter() {
            if row.len() != height {
                panic!("Row is malformed, {:?}.len != {}", row, height);
            }
        }

        return Self {
            min,
            max,
            attraction_matrix,
            colors,
        };
    }
}
