use crate::config::structs::CameraSettings;

use crate::floating_cam::control_state::ControlState;

use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingCam;

#[derive(Resource)]
pub struct PlayerState {
    pub rotation: Quat,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
        }
    }
}

pub fn transform_camera(
    mut cam: Query<&mut Transform, With<FloatingCam>>,
    mut control_state: ResMut<ControlState>,
    mut player_state: ResMut<PlayerState>,
) {
    match cam.get_single_mut() {
        Ok(mut transform) => {
            let total_look_delta = -(control_state.td.mouse_look + control_state.td.button_look);

            let pitch = Quat::from_axis_angle(Vec3::X, total_look_delta.y);
            transform.rotate_local(pitch);

            let yaw = Quat::from_axis_angle(Vec3::Y, total_look_delta.x);
            player_state.rotation *= yaw;
            transform.rotate(yaw);

            transform.translation += player_state.rotation.mul_vec3(control_state.td.move_dir);

            control_state.reset_look();
            control_state.reset_move();
        }
        Err(_) => warn!("There is no player camera in the scene"),
    }
}
