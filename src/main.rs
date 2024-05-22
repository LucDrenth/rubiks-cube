use bevy::prelude::*;
use camera::CameraPlugin;
use rubiks_cube::RubiksCubePlugin;

mod camera;
mod rubiks_cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(RubiksCubePlugin)
        .run();
}
