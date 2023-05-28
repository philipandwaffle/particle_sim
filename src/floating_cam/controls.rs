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
        }
    }
}

#[derive(Resource)]
pub struct ControlState {
    pub move_dir: Vec3,
    pub look_delta: Vec2,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            move_dir: Vec3::ZERO,
            look_delta: Vec2::ZERO,
        }
    }
}

fn update_control_state(
    mut control_state: ResMut<ControlState>,
    mut motion_evr: EventReader<MouseMotion>,
    input: Res<Input<KeyCode>>,
    bindings: Res<Bindings>,
) {
    // Update keyboard
    // Y is forward and X is right
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

    // Update mouse
    let mut look_delta = Vec2::ZERO;
    for ev in motion_evr.iter() {
        look_delta += ev.delta;
    }
    control_state.look_delta = look_delta;
}
