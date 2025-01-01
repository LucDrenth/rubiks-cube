use bevy::{input::mouse::MouseMotion, log, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (camera_controls_keyboard, camera_controls_mouse));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(6.5, 7.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Camera controls to rotate around the center point
fn camera_controls_keyboard(
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
            Quat::from_rotation_x(rotation * time.delta_secs()),
        );
    }
    if let Some(rotation) = rotation_y {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(rotation * time.delta_secs()),
        );
    }
}

fn camera_controls_mouse(
    mut query: Query<&mut Transform, With<Camera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut moust_motion_event_reader: EventReader<MouseMotion>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        error!("Expected exactly 1 camera component");
        return;
    };

    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    let mut mouse_moved = Vec2::ZERO;

    for mouse_motion in moust_motion_event_reader.read() {
        mouse_moved += mouse_motion.delta;
    }

    mouse_moved /= 50.0;

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-mouse_moved.x));
    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-mouse_moved.y));
}
