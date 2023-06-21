use bevy::prelude::Vec2;

#[derive(Clone)]
pub struct Nav {
    pub primary_nav: Vec2,
    pub secondary_nav: isize,
    pub primary_interact: bool,
    pub secondary_interact: bool,
}
impl Nav {
    pub fn new(
        primary_nav: Vec2,
        secondary_nav: isize,
        primary_interact: bool,
        secondary_interact: bool,
    ) -> Self {
        return Self {
            primary_nav,
            secondary_nav,
            primary_interact,
            secondary_interact,
        };
    }
}
