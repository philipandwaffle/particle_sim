use bevy::prelude::*;

#[derive(Component)]
pub struct InteractionDesigner {
    pub point_entities: Vec<Entity>,
    pub line_entities: Vec<Entity>,
    pub point_positions: Vec<Vec2>,
    pub cur_point_id: isize,
    pub num_points: usize,
}
