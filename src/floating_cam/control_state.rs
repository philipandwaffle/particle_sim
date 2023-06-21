use bevy::{
    prelude::{Resource, Vec2, Vec3},
    transform,
};

#[derive(Resource, Clone)]
pub struct TransformDelta {
    pub move_dir: Vec3,
    pub mouse_look: Vec2,
    pub button_look: Vec2,
}
impl TransformDelta {
    pub fn new(move_dir: Vec3, mouse_look: Vec2, button_look: Vec2) -> Self {
        return Self {
            move_dir,
            mouse_look,
            button_look,
        };
    }

    pub fn reset_move(&mut self) {
        self.move_dir = Vec3::ZERO;
    }
    pub fn reset_look(&mut self) {
        self.mouse_look = Vec2::ZERO;
        self.button_look = Vec2::ZERO;
    }
}
impl Default for TransformDelta {
    fn default() -> Self {
        Self {
            move_dir: Vec3::ZERO,
            mouse_look: Vec2::ZERO,
            button_look: Vec2::ZERO,
        }
    }
}

#[derive(Resource, Clone)]
pub struct NavDelta {
    pub primary_nav: Vec2,
    pub secondary_nav: isize,
    pub primary_interact: bool,
    pub secondary_interact: bool,
}
impl NavDelta {
    pub fn new(
        designer_primary_nav_delta: Vec2,
        designer_secondary_nav_delta: isize,
        designer_primary_interact: bool,
        designer_secondary_interact: bool,
    ) -> Self {
        return Self {
            primary_nav: designer_primary_nav_delta,
            secondary_nav: designer_secondary_nav_delta,
            primary_interact: designer_primary_interact,
            secondary_interact: designer_secondary_interact,
        };
    }

    pub fn reset(&mut self) {
        self.primary_nav = Vec2::ZERO;
        self.secondary_nav = 0;
        self.primary_interact = false;
        self.secondary_interact = false;
    }
}
impl Default for NavDelta {
    fn default() -> Self {
        Self {
            primary_nav: Vec2::ZERO,
            secondary_nav: 0,
            primary_interact: false,
            secondary_interact: false,
        }
    }
}

#[derive(Resource)]
pub struct ControlState {
    pub td: TransformDelta,
    pub nd: NavDelta,
}
impl ControlState {
    pub fn reset_move(&mut self) {
        self.td.reset_move()
    }
    pub fn reset_look(&mut self) {
        self.td.reset_look()
    }
    pub fn reset_nav(&mut self) {
        self.nd.reset()
    }
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            td: TransformDelta::default(),
            nd: NavDelta::default(),
        }
    }
}
