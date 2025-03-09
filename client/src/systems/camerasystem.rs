use avian3d::parry::na::clamp;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::math::{Dir3, Vec3};
use bevy::prelude::{ButtonInput, Camera3d, Commands, Component, EventReader, MouseButton, PerspectiveProjection, Query, Res, Transform, With, Without};
use bevy::utils::default;
use shared::globalcomponents::PlayerCombatant;

#[derive(Component)]
pub struct PlayerCamera;

pub struct Orbit{
    distance: f32,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    min_pitch: f32,
    max_pitch: f32,
}

#[derive(Component)]
pub struct CameraConfigs{
    orbit:  Orbit,
}


pub fn create_camera(
    mut commands: Commands,
){
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
        CameraConfigs{
            orbit: Orbit {
                distance: 5.0,
                yaw: -90.0,
                pitch: 0.0,
                sensitivity: 0.3,
                min_pitch: -89.0,
                max_pitch: 89.0,
            },
        },
        PerspectiveProjection{
            fov: 60.0_f32.to_radians(),
            ..default()
        },
        PlayerCamera,
    ));
}

pub fn mouse_moving_camera(
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut CameraConfigs), With<PlayerCamera>>,
    mut evr_motion: EventReader<MouseMotion>,
){
    let mut camera_configs = match camera_query.get_single_mut() {
        Ok(camera_configs) => camera_configs,
        Err(_) => return
    };
    let mut camera_orbit = &mut camera_configs.orbit;

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                camera_orbit.distance = clamp(camera_orbit.distance - ev.y, 5.0, 30.0);
            }
            MouseScrollUnit::Pixel => {
                camera_orbit.distance = clamp(camera_orbit.distance - ev.y, 5.0, 30.0);
            }
        }
    }

    if buttons.pressed(MouseButton::Right) {
        let mut delta_x: f32 = 0.0;
        let mut delta_y: f32 = 0.0;

        for ev in evr_motion.read() {
            delta_x += ev.delta.x * camera_orbit.sensitivity;
            delta_y += ev.delta.y * camera_orbit.sensitivity;
        }

        delta_x = delta_x.round();
        delta_y = delta_y.round();

        if delta_x.abs() / delta_y.abs() < 0.2 {
            delta_x = 0.0;
        }

        if delta_y.abs() / delta_x.abs() < 0.2 {
            delta_y = 0.0;
        }

        camera_orbit.yaw += delta_x;
        camera_orbit.pitch = (camera_orbit.pitch - delta_y).clamp(camera_orbit.min_pitch, camera_orbit.max_pitch);
    } else {
        
    }
}

pub fn update_camera(
    mut camera_query: Query<(&mut Transform,&mut CameraConfigs), (Without<PlayerCombatant>, With<PlayerCamera>)>,
    character_query: Query<&Transform, With<PlayerCombatant>>
){
    let transform_character = match character_query.get_single() {
        Ok(character_transform) => character_transform,
        Err(_) => return
    };

    for (mut transform_camera,camera_configs) in camera_query.iter_mut() {
        let mut camera_orbit = &camera_configs.orbit;
        let (yaw, pitch) = (camera_orbit.yaw.to_radians(), camera_orbit.pitch.to_radians());
        let camera_position = Vec3::new(
            camera_orbit.distance * yaw.cos() * pitch.cos(),
            camera_orbit.distance * pitch.sin(),
            camera_orbit.distance * yaw.sin() * pitch.cos(),
        );

        let character_translation = transform_character.translation;
        let adjusted_translation = character_translation + camera_position;

        transform_camera.translation = adjusted_translation;
        transform_camera.look_at(character_translation, Vec3::Y)
    }
}