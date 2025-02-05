use bevy::{input::mouse::AccumulatedMouseMotion, log, prelude::*};

use crate::{cube::Cube, interface::interface::UiResource, schedules::CubeScheduleSet};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (scale_cube_with_keys, move_cube_with_mouse).in_set(CubeScheduleSet::HandleUserInput),
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
