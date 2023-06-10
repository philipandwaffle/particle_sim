use bevy::{core::Zeroable, input::mouse::MouseMotion, prelude::*};

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlState::default());
        app.insert_resource(Bindings::default());
        app.add_system(update_control_state);
    }
}

#[derive(Resource)]
pub struct Bindings {
    forward_key: KeyCode,
    backward_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
    fly_up: KeyCode,
    fly_down: KeyCode,
    look_up: KeyCode,
    look_down: KeyCode,
    look_left: KeyCode,
    look_right: KeyCode,
    next_mode: KeyCode,
    prev_mode: KeyCode,
    designer_nav_up: KeyCode,
    designer_nav_down: KeyCode,
    designer_nav_left: KeyCode,
    designer_nav_right: KeyCode,
    designer_nav_next: KeyCode,
    designer_nav_prev: KeyCode,
    designer_primary: KeyCode,
    designer_secondary: KeyCode,
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            forward_key: KeyCode::W,
            backward_key: KeyCode::S,
            left_key: KeyCode::A,
            right_key: KeyCode::D,
            fly_up: KeyCode::Space,
            fly_down: KeyCode::LShift,
            look_up: KeyCode::Up,
            look_down: KeyCode::Down,
            look_left: KeyCode::Left,
            look_right: KeyCode::Right,
            next_mode: KeyCode::Key1,
            prev_mode: KeyCode::Key2,
            designer_nav_up: KeyCode::I,
            designer_nav_down: KeyCode::K,
            designer_nav_left: KeyCode::J,
            designer_nav_right: KeyCode::L,
            designer_nav_next: KeyCode::O,
            designer_nav_prev: KeyCode::U,
            designer_primary: KeyCode::Return,
            designer_secondary: KeyCode::Escape,
        }
    }
}

#[derive(Resource)]
pub struct ControlState {
    pub move_dir_delta: Vec3,
    pub mouse_look_delta: Vec2,
    pub button_look_delta: Vec2,
    pub designer_primary_nav_delta: Vec2,
    pub designer_secondary_nav_delta: isize,
    pub designer_primary_interact: bool,
    pub designer_secondary_interact: bool,
}
impl ControlState {
    pub fn reset_move(&mut self) {
        self.move_dir_delta = Vec3::ZERO;
    }
    pub fn reset_look(&mut self) {
        self.mouse_look_delta = Vec2::ZERO;
        self.button_look_delta = Vec2::ZERO;
    }
    pub fn reset_designer(&mut self) {
        self.designer_primary_nav_delta = Vec2::ZERO;
        self.designer_secondary_nav_delta = 0;
        self.designer_primary_interact = false;
        self.designer_secondary_interact = false;
    }
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            move_dir_delta: Vec3::ZERO,
            mouse_look_delta: Vec2::ZERO,
            button_look_delta: Vec2::ZERO,
            designer_primary_nav_delta: Vec2::ZERO,
            designer_secondary_nav_delta: 0,
            designer_primary_interact: false,
            designer_secondary_interact: false,
        }
    }
}

fn update_control_state(
    mut control_state: ResMut<ControlState>,
    mut motion_evr: EventReader<MouseMotion>,
    input: Res<Input<KeyCode>>,
    bindings: Res<Bindings>,
) {
    // Calculate movement delta
    let mut move_dir_delta = Vec3::ZERO;
    if input.pressed(bindings.forward_key) {
        move_dir_delta.z -= 1.0
    }
    if input.pressed(bindings.backward_key) {
        move_dir_delta.z += 1.0
    }
    if input.pressed(bindings.right_key) {
        move_dir_delta.x += 1.0
    }
    if input.pressed(bindings.left_key) {
        move_dir_delta.x -= 1.0
    }
    if input.pressed(bindings.fly_up) {
        move_dir_delta.y += 1.0
    }
    if input.pressed(bindings.fly_down) {
        move_dir_delta.y -= 1.0
    }

    // Calculate mouse look delta
    let mut mouse_look_delta = Vec2::ZERO;
    for ev in motion_evr.iter() {
        mouse_look_delta += ev.delta;
    }

    // Calculate button look delta
    let mut button_look_delta = Vec2::ZERO;
    if input.pressed(bindings.look_up) {
        button_look_delta.y -= 1.0;
    }
    if input.pressed(bindings.look_down) {
        button_look_delta.y += 1.0;
    }
    if input.pressed(bindings.look_left) {
        button_look_delta.x -= 1.0;
    }
    if input.pressed(bindings.look_right) {
        button_look_delta.x += 1.0;
    }

    // Calculate primary nav delta
    let mut design_primary_nav_delta = Vec2::ZERO;
    if input.pressed(bindings.designer_nav_up) {
        design_primary_nav_delta.y += 1.0;
    }
    if input.pressed(bindings.designer_nav_down) {
        design_primary_nav_delta.y -= 1.0;
    }
    if input.pressed(bindings.designer_nav_left) {
        design_primary_nav_delta.x -= 1.0;
    }
    if input.pressed(bindings.designer_nav_right) {
        design_primary_nav_delta.x += 1.0;
    }

    // Calculate secondary nav delta
    let mut design_secondary_nav_delta = 0;
    if input.just_pressed(bindings.designer_nav_next) {
        design_secondary_nav_delta += 1;
    }
    if input.just_pressed(bindings.designer_nav_prev) {
        design_secondary_nav_delta -= 1;
    }

    apply_new_control_state(
        &mut control_state,
        move_dir_delta,
        mouse_look_delta,
        button_look_delta,
        design_primary_nav_delta,
        design_secondary_nav_delta,
        input.just_pressed(bindings.designer_primary),
        input.just_pressed(bindings.designer_secondary),
    )
}

fn apply_new_control_state(
    cs: &mut ControlState,
    move_dir_delta: Vec3,
    mouse_look_delta: Vec2,
    button_look_delta: Vec2,
    design_primary_nav_delta: Vec2,
    design_secondary_nav_delta: isize,
    designer_primary_interact: bool,
    designer_secondary_interact: bool,
) {
    // Update move delta
    if move_dir_delta != Vec3::ZERO {
        cs.move_dir_delta += move_dir_delta;
    }

    // Update look delta
    if mouse_look_delta != Vec2::ZERO {
        cs.mouse_look_delta += mouse_look_delta;
    }
    if button_look_delta != Vec2::ZERO {
        cs.button_look_delta += button_look_delta;
    }

    // Update designer navigation
    if design_primary_nav_delta != Vec2::ZERO {
        cs.designer_primary_nav_delta += design_primary_nav_delta;
    }
    if design_secondary_nav_delta != 0 {
        cs.designer_secondary_nav_delta += design_secondary_nav_delta;
    }

    // Update designer interact
    if cs.designer_primary_interact != designer_primary_interact {
        cs.designer_primary_interact = designer_primary_interact;
    }
    if cs.designer_secondary_interact != designer_secondary_interact {
        cs.designer_secondary_interact = designer_secondary_interact;
    }
}
