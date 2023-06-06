use bevy::{input::mouse::MouseMotion, prelude::*};

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
    designer_point_up: KeyCode,
    designer_point_down: KeyCode,
    designer_point_left: KeyCode,
    designer_point_right: KeyCode,
    next_designer_point: KeyCode,
    prev_designer_point: KeyCode,
    save_designer_points: KeyCode,
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
            designer_point_up: KeyCode::I,
            designer_point_down: KeyCode::K,
            designer_point_left: KeyCode::J,
            designer_point_right: KeyCode::L,
            next_designer_point: KeyCode::O,
            prev_designer_point: KeyCode::U,
            save_designer_points: KeyCode::P,
        }
    }
}

#[derive(Resource)]
pub struct ControlState {
    pub move_dir: Vec3,
    pub mouse_look_delta: Vec2,
    pub button_look_delta: Vec2,
    pub design_point_delta: Vec2,
    pub design_point_id_delta: isize,
    pub save_designer_points: bool,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            move_dir: Vec3::ZERO,
            mouse_look_delta: Vec2::ZERO,
            button_look_delta: Vec2::ZERO,
            design_point_delta: Vec2::ZERO,
            design_point_id_delta: 0,
            save_designer_points: false,
        }
    }
}

fn update_control_state(
    mut control_state: ResMut<ControlState>,
    mut motion_evr: EventReader<MouseMotion>,
    input: Res<Input<KeyCode>>,
    bindings: Res<Bindings>,
) {
    // Update movement keys
    let cs = control_state.as_mut();
    if input.pressed(bindings.forward_key) {
        cs.move_dir.z -= 1.0
    }
    if input.pressed(bindings.backward_key) {
        cs.move_dir.z += 1.0
    }
    if input.pressed(bindings.right_key) {
        cs.move_dir.x += 1.0
    }
    if input.pressed(bindings.left_key) {
        cs.move_dir.x -= 1.0
    }

    if input.pressed(bindings.fly_up) {
        cs.move_dir.y += 1.0
    }
    if input.pressed(bindings.fly_down) {
        cs.move_dir.y -= 1.0
    }

    // Update mouse look
    let mut mouse_look_delta = Vec2::ZERO;
    for ev in motion_evr.iter() {
        mouse_look_delta += ev.delta;
    }

    // Update mouse button
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
    control_state.button_look_delta = button_look_delta;
    control_state.mouse_look_delta = mouse_look_delta;

    // Update design point
    let mut design_point_delta = Vec2::ZERO;
    if input.pressed(bindings.designer_point_up) {
        design_point_delta.y += 1.0;
    }
    if input.pressed(bindings.designer_point_down) {
        design_point_delta.y -= 1.0;
    }
    if input.pressed(bindings.designer_point_left) {
        design_point_delta.x -= 1.0;
    }
    if input.pressed(bindings.designer_point_right) {
        design_point_delta.x += 1.0;
    }
    control_state.design_point_delta = design_point_delta;

    // Update design point id
    let mut design_point_id_delta = 0;
    if input.just_pressed(bindings.next_designer_point) {
        design_point_id_delta += 1;
    }
    if input.just_pressed(bindings.prev_designer_point) {
        design_point_id_delta -= 1;
    }
    control_state.design_point_id_delta += design_point_id_delta;

    if input.just_pressed(bindings.save_designer_points) {
        control_state.save_designer_points = true;
    }
}
