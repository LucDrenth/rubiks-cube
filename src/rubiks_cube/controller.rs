use bevy::prelude::*;

use super::{rubiks_cube::CubeRotation3x3, CubeRotationEvent};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, do_test_rotation);
    }
}

fn do_test_rotation(mut event_writer: EventWriter<CubeRotationEvent>) {
    event_writer.send(CubeRotationEvent {
        rotation: CubeRotation3x3::Left,
        counter_clockwise: true,
        twice: false,
    });
}
