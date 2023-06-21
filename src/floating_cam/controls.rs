use bevy::{core::Zeroable, input::mouse::MouseMotion, prelude::*};

use super::control_state::ControlState;

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
    primary_nav_up: KeyCode,
    primary_nav_down: KeyCode,
    primary_nav_left: KeyCode,
    primary_nav_right: KeyCode,
    secondary_nav_next: KeyCode,
    secondary_nav_prev: KeyCode,
    primary_interact: KeyCode,
    secondary_interact: KeyCode,
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
            primary_nav_up: KeyCode::I,
            primary_nav_down: KeyCode::K,
            primary_nav_left: KeyCode::J,
            primary_nav_right: KeyCode::L,
            secondary_nav_next: KeyCode::O,
            secondary_nav_prev: KeyCode::U,
            primary_interact: KeyCode::Return,
            secondary_interact: KeyCode::Escape,
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
    if input.pressed(bindings.primary_nav_up) {
        design_primary_nav_delta.y += 1.0;
    }
    if input.pressed(bindings.primary_nav_down) {
        design_primary_nav_delta.y -= 1.0;
    }
    if input.pressed(bindings.primary_nav_left) {
        design_primary_nav_delta.x -= 1.0;
    }
    if input.pressed(bindings.primary_nav_right) {
        design_primary_nav_delta.x += 1.0;
    }

    // Calculate secondary nav delta
    let mut design_secondary_nav_delta = 0;
    if input.just_pressed(bindings.secondary_nav_next) {
        design_secondary_nav_delta += 1;
    }
    if input.just_pressed(bindings.secondary_nav_prev) {
        design_secondary_nav_delta -= 1;
    }

    apply_new_control_state(
        &mut control_state,
        move_dir_delta,
        mouse_look_delta,
        button_look_delta,
        design_primary_nav_delta,
        design_secondary_nav_delta,
        input.just_pressed(bindings.primary_interact),
        input.just_pressed(bindings.secondary_interact),
    )
}

fn apply_new_control_state(
    cs: &mut ControlState,
    move_dir: Vec3,
    mouse_look: Vec2,
    button_look: Vec2,
    primary_nav: Vec2,
    secondary_nav: isize,
    primary_interact: bool,
    secondary_interact: bool,
) {
    // Update move delta
    if move_dir != Vec3::ZERO {
        cs.td.move_dir += move_dir;
    }

    // Update look delta
    if mouse_look != Vec2::ZERO {
        cs.td.mouse_look += mouse_look;
    }
    if button_look != Vec2::ZERO {
        cs.td.button_look += button_look;
    }

    // Update designer navigation
    if primary_nav != Vec2::ZERO {
        cs.nd.primary_nav += primary_nav;
    }
    if secondary_nav != 0 {
        cs.nd.secondary_nav += secondary_nav;
    }

    // Update designer interact
    if cs.nd.primary_interact != primary_interact {
        cs.nd.primary_interact = primary_interact;
    }
    if cs.nd.secondary_interact != secondary_interact {
        cs.nd.secondary_interact = secondary_interact;
    }
}
