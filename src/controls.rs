use bevy::{input::mouse::MouseMotion, prelude::*};

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
            (rotate_cube_with_keys, move_cube_with_mouse).in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

fn rotate_cube_with_keys(
    cube_query: Query<&mut Cube>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let cube = cube_query.get_single().unwrap();
    if cube.is_animating_rotation {
        return;
    }

    let animiation = CubeRotationAnimation {
        duration_in_seconds: 0.4,
        ease_function: Some(EaseFunction::CubicOut),
    };

    let mut cube_rotation: Option<CubeRotation> = None;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        cube_rotation = Some(CubeRotation::YPrime);
    } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        cube_rotation = Some(CubeRotation::Y);
    } else if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        cube_rotation = Some(CubeRotation::X);
    } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
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
    mut cube_query: Query<&mut Transform, With<Cube>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    ui_resource: Res<UiResource>,
    mut moust_motion_event_reader: EventReader<MouseMotion>,
) {
    let mut cube_transform = cube_query.get_single_mut().unwrap();

    if !mouse_input.pressed(MouseButton::Left) || ui_resource.did_handle_click {
        return;
    }

    let mut mouse_moved = Vec2::ZERO;

    for mouse_motion in moust_motion_event_reader.read() {
        mouse_moved += mouse_motion.delta;
    }

    mouse_moved /= 10.0;

    cube_transform.translation += Vec3 {
        x: mouse_moved.x,
        y: 0.0,
        z: mouse_moved.y,
    };
}
