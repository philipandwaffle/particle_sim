use super::controls::ControlState;

use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingCam {
    pub mouse_look_sen: f32,
    pub button_look_sen: f32,
    pub speed: f32,
}
impl Default for FloatingCam {
    fn default() -> Self {
        Self {
            mouse_look_sen: 0.001,
            button_look_sen: 10.0,
            speed: 0.1,
        }
    }
}
// impl FloatingCam {
//     pub fn new(sen: f32, speed: f32) -> Self {
//         return Self {
//             sen: sen,
//             speed: speed,
//         };
//     }
// }

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
    mut cam: Query<(&FloatingCam, &mut Transform)>,
    mut cs: ResMut<ControlState>,
    mut ps: ResMut<PlayerState>,
) {
    match cam.get_single_mut() {
        Ok((fps_cam, mut transform)) => {
            let total_look_delta = (cs.mouse_look_delta * -fps_cam.mouse_look_sen)
                + (cs.button_look_delta * -fps_cam.button_look_sen);
            let pitch =
                Quat::from_axis_angle(Vec3::X, total_look_delta.y * -fps_cam.mouse_look_sen);
            transform.rotate_local(pitch);

            let yaw = Quat::from_axis_angle(Vec3::Y, total_look_delta.x * -fps_cam.mouse_look_sen);
            ps.rotation *= yaw;
            transform.rotate(yaw);

            transform.translation += ps.rotation.mul_vec3(cs.move_dir) * fps_cam.speed;

            cs.move_dir = Vec3::ZERO
        }
        Err(_) => warn!("There is no player camera in the scene"),
    }
}
