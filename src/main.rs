use bevy::prelude::*;
use camera::CameraPlugin;
use controls::ControlsPlugin;
use cube::CubePlugin;
use interface::interface::InterfacePlugin;
use schedules::SchedulesPlugin;

mod camera;
mod controls;
#[allow(dead_code)]
mod cube;
mod interface;
mod schedules;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rubiks cube".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(CubePlugin)
        .add_plugins(InterfacePlugin)
        .add_plugins(SchedulesPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(91, 145, 222)))
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            intensity: 25_000_000.,
            range: 50.0,
            shadow_depth_bias: 0.8,
            ..default()
        },
        Transform::from_xyz(11.5, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 150.0,
    });
}
