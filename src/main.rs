use bevy::prelude::*;
use camera::CameraPlugin;
use controls::ControlsPlugin;
use cube::CubePlugin;
use schedules::SchedulesPlugin;

mod camera;
mod controls;
mod cube;
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
        .add_plugins(SchedulesPlugin)
        .add_systems(Startup, spawn_light)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 300.0,
            shadow_depth_bias: 0.8,
            ..default()
        },
        Transform::from_xyz(15.0, 15.0, 15.0),
    ));
}
