use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll}, prelude::*};


#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct CameraSettings {
    pub target: Vec3,
    pub distance: f32
}

pub fn move_camera(
    time: Res<Time>,
    pressed: Res<ButtonInput<KeyCode>>,
    mut camera: Single<(&mut Transform, &mut CameraSettings), With<GameCamera>>,
) {
    let mut move_vec = Vec3::ZERO;

    if pressed.pressed(KeyCode::KeyW) {
        move_vec += camera.0.up().as_vec3();
    }
    if pressed.pressed(KeyCode::KeyS) {
        move_vec += camera.0.down().as_vec3();
    }
    if pressed.pressed(KeyCode::KeyD) {
        move_vec += camera.0.right().as_vec3();
    }
    if pressed.pressed(KeyCode::KeyA) {
        move_vec += camera.0.left().as_vec3();
    }

    move_vec.y = 0.0;
    let mov_value = move_vec.normalize_or_zero() * 3.0 * time.delta_secs();
    camera.0.translation += mov_value;  
    camera.1.target += mov_value;  
}

pub fn camera_zoom(
    scroll_motion: Res<AccumulatedMouseScroll>,
    mut camera: Single<(&mut Transform, &mut CameraSettings), With<GameCamera>>, 
) {
    let delta_zoom = scroll_motion.delta.y * 0.3;
    camera.1.distance -= delta_zoom;
    camera.1.distance = camera.1.distance.clamp(2.0, 12.0);
    camera.0.translation = camera.1.target - camera.0.forward() * camera.1.distance;
}

pub fn camera_rotation(
    button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera: Single<(&mut Transform, &mut CameraSettings), With<GameCamera>>,
) {
    let target = camera.1.target;
    let dist = camera.1.distance;
    let camera_transform = &mut camera.0;
    // camera_orbit bevy example
    if button.pressed(MouseButton::Right) {
        let delta = mouse_motion.delta;
        let delta_pitch = -delta.y * 0.003;
        let delta_yaw = -delta.x * 0.004;
        let (mut yaw, mut pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
        pitch += delta_pitch;
        pitch = pitch.clamp(-FRAC_PI_2, -0.1);
        yaw += delta_yaw;
        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        camera_transform.translation = target - camera_transform.forward() * dist;
        
    }
}

pub fn setup_camera(mut commands: Commands) {
    let target = Vec3::ZERO;
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(target, Vec3::Y),
        GameCamera,
        CameraSettings {target, distance: 7.0},
    ));
}
