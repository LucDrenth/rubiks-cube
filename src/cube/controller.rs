use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{
    algorithms, create_scramble_sequence_from_algorithm, cube::Cube, cube_state::CubeState,
    rotation::RotationAnimation, CubeRotationEvent,
};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_stepper).add_systems(
            Update,
            (
                check_solved_on_enter,
                spacebar_stepper_handler,
                random_face_rotation_on_tab,
            )
                .chain()
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
pub struct RotationStepper {
    steps: Vec<CubeRotationEvent>,
    current: usize,
}

impl RotationStepper {
    fn step(&mut self) -> CubeRotationEvent {
        let event = self.steps[self.current].clone();

        self.current += 1;
        if self.current >= self.steps.len() {
            self.current = 0;
        }

        event
    }
}

fn init_stepper(mut commands: Commands) {
    let steps = create_scramble_sequence_from_algorithm(algorithms::size_3x3::flipped_pieces());
    commands.spawn(RotationStepper { steps, current: 0 });
}

#[derive(Component)]
pub struct LastRandomFaceRotationEvent(CubeRotationEvent);

fn random_face_rotation_on_tab(
    mut commands: Commands,
    mut last_random_face_rotation_query: Query<&mut LastRandomFaceRotationEvent>,
    cube_query: Query<&Cube>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    if !keyboard_input.pressed(KeyCode::Tab) {
        return;
    }

    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube component");
        return;
    };

    if cube.is_animating_rotation {
        return;
    }

    let mut rotation_event = match last_random_face_rotation_query.get_single_mut().ok() {
        Some(mut last_random_face_rotation) => loop {
            let new_rotation_event = CubeRotationEvent::random_face_rotation(cube.size());

            if new_rotation_event.negates(&last_random_face_rotation.0) {
                continue;
            }

            last_random_face_rotation.0 = new_rotation_event.clone();
            break new_rotation_event;
        },
        None => {
            let new_rotation_event = CubeRotationEvent::random_face_rotation(cube.size());
            commands.spawn(LastRandomFaceRotationEvent(new_rotation_event.clone()));
            new_rotation_event
        }
    };

    rotation_event.animation = Some(RotationAnimation {
        duration_in_seconds: 0.15,
        ease_function: None,
    });
    event_writer.send(rotation_event);
}

fn check_solved_on_enter(query: Query<&CubeState>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }

    let Ok(cube_state) = query.get_single() else {
        error!("Expected exactly 1 CubeState component");
        return;
    };

    if cube_state.is_solved() {
        info!("Cube is solved");
    } else {
        info!("Cube is not solved");
    }
}

fn spacebar_stepper_handler(
    mut stepper_query: Query<&mut RotationStepper>,
    cube_query: Query<&Cube>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube component");
        return;
    };

    if cube.is_animating_rotation {
        return;
    }

    let mut stepper = stepper_query.get_single_mut().unwrap();

    let mut rotation_event = stepper.step();
    rotation_event.animation = Some(RotationAnimation {
        duration_in_seconds: 0.25,
        ease_function: Some(EaseFunction::CubicOut),
    });
    event_writer.send(rotation_event);
}
