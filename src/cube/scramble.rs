use bevy::prelude::*;

use crate::schedules::CubeStartupSet;

use super::{
    algorithms,
    cube::{Cube, CubeSize},
    CubeRotationEvent,
};

pub struct CubeScramblePlugin;

impl Plugin for CubeScramblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            apply_instant_scramble.in_set(CubeStartupSet::ApplyScramble),
        );
    }
}

// For debugging
fn apply_instant_scramble(
    cube_query: Query<&Cube>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let Ok(_cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube in query result");
        return;
    };

    let scramble_sequence =
        create_scramble_sequence_from_algorithm(algorithms::size_3x3::flipped_pieces());

    for event in scramble_sequence {
        event_writer.send(event);
    }
}

pub fn create_scramble_sequence_from_algorithm<T>(algorithm: Vec<T>) -> Vec<CubeRotationEvent>
where
    T: Into<CubeRotationEvent>,
{
    algorithm
        .into_iter()
        .map(|rotation| rotation.into())
        .collect()
}

pub fn create_random_scramble_sequence(
    cube_size: &CubeSize,
    number_of_rotations: usize,
) -> Vec<CubeRotationEvent> {
    create_scramble_sequence_with_strategy(
        cube_size,
        number_of_rotations,
        &mut CubeRotationEvent::random_face_rotation,
    )
}

fn create_scramble_sequence_with_strategy(
    cube_size: &CubeSize,
    number_of_rotations: usize,
    random_event_strategy: &mut dyn FnMut(&CubeSize) -> CubeRotationEvent,
) -> Vec<CubeRotationEvent> {
    let mut result: Vec<CubeRotationEvent> = Vec::with_capacity(number_of_rotations);

    while result.len() < number_of_rotations {
        let new_rotation = random_event_strategy(cube_size);

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

#[cfg(test)]
mod tests {
    use crate::cube::{
        create_random_scramble_sequence, cube::CubeSize, rotation::Rotation, CubeRotationEvent,
    };

    use super::create_scramble_sequence_with_strategy;

    struct FakeRandomEventProvider {
        events: Vec<CubeRotationEvent>,
        current: usize,
    }

    impl FakeRandomEventProvider {
        fn new(events: Vec<CubeRotationEvent>) -> Self {
            Self { events, current: 0 }
        }

        fn next(&mut self) -> CubeRotationEvent {
            self.current += 1;
            self.events[self.current - 1].clone()
        }
    }

    #[test]
    fn test_create_random_scramble_sequence() {
        let cube_size = CubeSize(3);
        let sequence_length = 5;
        let sequence = create_random_scramble_sequence(&cube_size, sequence_length);
        assert_eq!(sequence.len(), sequence_length);
    }

    #[test]
    fn test_scramble_sequence_skips_directly_negating_sequence() {
        let cube_size = CubeSize(3);

        // 2nd event negates 1st event
        let mut event_provider = FakeRandomEventProvider::new(vec![
            CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
            CubeRotationEvent {
                rotation: Rotation::face_z(1),
                negative_direction: false,
                twice: false,
                animation: None,
            },
        ]);

        let sequence =
            create_scramble_sequence_with_strategy(&cube_size, 2, &mut |_| event_provider.next());

        assert_eq!(sequence.len(), 2);
        assert!(sequence[0].equals(&event_provider.events[0]));
        assert!(sequence[1].equals(&event_provider.events[2]));
    }

    #[test]
    fn test_scramble_sequence_skips_triple_same_rotation() {
        let cube_size = CubeSize(3);

        // 3th event is the same as the 1st and 2nd
        let mut event_provider = FakeRandomEventProvider::new(vec![
            CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotationEvent {
                rotation: Rotation::face_y(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
            CubeRotationEvent {
                rotation: Rotation::face_x(1),
                negative_direction: true,
                twice: false,
                animation: None,
            },
        ]);

        let sequence =
            create_scramble_sequence_with_strategy(&cube_size, 3, &mut |_| event_provider.next());

        assert_eq!(sequence.len(), 3);
        assert!(sequence[0].equals(&event_provider.events[0]));
        assert!(sequence[1].equals(&event_provider.events[1]));
        assert!(sequence[2].equals(&event_provider.events[3]));
    }
}
