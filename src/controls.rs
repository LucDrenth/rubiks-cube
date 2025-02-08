use bevy::prelude::*;

use crate::{cube::Cube, schedules::CubeScheduleSet};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (scale_cube_with_keys,).in_set(CubeScheduleSet::HandleUserInput),
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
            error!("failed to get cube transform: {err}");
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

    transform.scale = transform.scale.max(Vec3::ZERO);
}
