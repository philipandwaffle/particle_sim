use crate::config::structs::CameraSettings;

use super::controls::ControlState;

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
    cam_settings: Res<CameraSettings>,
) {
    match cam.get_single_mut() {
        Ok(mut transform) => {
            println!("received: {}", control_state.mouse_look_delta);
            let total_look_delta = (control_state.mouse_look_delta * -cam_settings.mouse_look_sen)
                + (control_state.button_look_delta * -cam_settings.button_look_sen);

            let pitch = Quat::from_axis_angle(Vec3::X, total_look_delta.y);
            transform.rotate_local(pitch);

            let yaw = Quat::from_axis_angle(Vec3::Y, total_look_delta.x);
            player_state.rotation *= yaw;
            transform.rotate(yaw);

            transform.translation +=
                player_state.rotation.mul_vec3(control_state.move_dir) * cam_settings.move_speed;

            control_state.move_dir = Vec3::ZERO
        }
        Err(_) => warn!("There is no player camera in the scene"),
    }
}
