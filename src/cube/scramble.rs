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

        // Prevent a rotation that directly negates the last event
        if let Some(previous_rotation) = result.last() {
            if new_rotation.negates(previous_rotation) {
                continue;
            }
        }

        // Prevent the same turn from being done more than 2 times in a row.
        // Doing it 3 times in a row wastes 2 turns, since it might as wel have been 1 turn in the negative direction.
        if result.len() >= 2
            && new_rotation.equals(&result[result.len() - 1])
            && new_rotation.equals(&result[result.len() - 2])
        {
            continue;
        }

        result.push(new_rotation);
    }

    result
}
