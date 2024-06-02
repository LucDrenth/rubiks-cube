use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, camera_controls);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(6.5, 7.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// Camera controls to rotate around the center point
fn camera_controls(
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        error!("Expected exactly 1 camera component");
        return;
    };

    let rotation_speed = 1.75;

    let rotation_y: Option<f32> = if keyboard_input.pressed(KeyCode::KeyA)
        || keyboard_input.pressed(KeyCode::ArrowLeft)
    {
        Some(-rotation_speed)
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        Some(rotation_speed)
    } else {
        None
    };

    let rotation_x: Option<f32> = if keyboard_input.pressed(KeyCode::KeyW)
        || keyboard_input.pressed(KeyCode::ArrowUp)
    {
        Some(-rotation_speed)
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        Some(rotation_speed)
    } else {
        None
    };

    if let Some(rotation) = rotation_x {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_x(rotation * time.delta_seconds()),
        );
    }
    if let Some(rotation) = rotation_y {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(rotation * time.delta_seconds()),
        );
    }
}
