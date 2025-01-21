use bevy::{input::mouse::AccumulatedMouseMotion, log, prelude::*};

use crate::{
    cube::{Cube, CubeRotation, CubeRotationAnimation, CubeRotationEvent},
    interface::interface::UiResource,
    schedules::CubeScheduleSet,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                scale_cube_with_keys,
                rotate_cube_with_keys,
                move_cube_with_mouse,
            )
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

fn scale_cube_with_keys(
    mut cube_transform_query: Query<&mut Transform, With<Cube>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = match cube_transform_query.get_single_mut() {
        Ok(transform) => transform,
        Err(err) => {
            log::error!("failed to get cube transform: {err}");
            return;
        }
    };

    let speed = 1.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.scale += Vec3::splat(speed * time.delta_secs());
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.scale -= Vec3::splat(speed * time.delta_secs());
    } else {
        return;
    }

    transform.scale = transform.scale.clamp(Vec3::ZERO, Vec3::ONE);
}

fn rotate_cube_with_keys(
    cube_query: Query<&Cube>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let cube = match cube_query.get_single() {
        Ok(cube) => cube,
        Err(err) => {
            log::error!("failed to get cube: {err}");
            return;
        }
    };

    if cube.is_animating_rotation {
        return;
    }

    let animiation = CubeRotationAnimation {
        duration_in_seconds: 0.4,
        ease_function: Some(EaseFunction::CubicOut),
    };

    let mut cube_rotation: Option<CubeRotation> = None;

    if keyboard_input.pressed(KeyCode::KeyA) {
        cube_rotation = Some(CubeRotation::YPrime);
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        cube_rotation = Some(CubeRotation::Y);
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        cube_rotation = Some(CubeRotation::X);
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        cube_rotation = Some(CubeRotation::XPrime);
    } else if keyboard_input.pressed(KeyCode::KeyR) {
        cube_rotation = Some(CubeRotation::Z);
    } else if keyboard_input.pressed(KeyCode::KeyF) {
        cube_rotation = Some(CubeRotation::ZPrime);
    }

    match cube_rotation {
        Some(cube_rotation) => {
            let mut event: CubeRotationEvent = cube_rotation.into();
            event.animation = Some(animiation);
            event_writer.send(event);
        }
        None => return,
    }
}

fn move_cube_with_mouse(
    mut cube_transform_query: Query<&mut Transform, With<Cube>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    ui_resource: Res<UiResource>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let mut cube_transform = match cube_transform_query.get_single_mut() {
        Ok(cube_transform) => cube_transform,
        Err(err) => {
            log::error!("failed to get cube transform: {err}");
            return;
        }
    };

    if !mouse_input.pressed(MouseButton::Left) || ui_resource.did_handle_click {
        return;
    }

    let amount_to_move_cube = mouse_motion.delta / 10.0;

    cube_transform.translation += Vec3 {
        x: amount_to_move_cube.x,
        y: 0.0,
        z: amount_to_move_cube.y,
    };
}
