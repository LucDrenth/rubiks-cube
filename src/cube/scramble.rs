use bevy::prelude::*;

use crate::schedules::CubeStartupSet;

use super::{cube::Cube, CubeRotationEvent};

pub struct CubeScramblePlugin;

impl Plugin for CubeScramblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            apply_instant_scramble.in_set(CubeStartupSet::ApplyScramble),
        );
    }
}

fn apply_instant_scramble(
    cube_query: Query<&Cube>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube in query result");
        return;
    };

    let scramble_sequence = create_scramble_sequence(cube, 20);
    for event in scramble_sequence {
        event_writer.send(event);
    }
}

pub fn create_scramble_sequence(cube: &Cube, number_of_rotations: usize) -> Vec<CubeRotationEvent> {
    let mut result: Vec<CubeRotationEvent> = Vec::with_capacity(number_of_rotations);

    while result.len() < number_of_rotations {
        let new_rotation = CubeRotationEvent::random_face_rotation(cube);

        if let Some(previous_rotation) = result.last() {
            if new_rotation.negates(previous_rotation) {
                continue;
            }
        }

        result.push(new_rotation);
    }

    result
}
