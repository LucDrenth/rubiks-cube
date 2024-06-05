use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{
    cube::{Cube, Piece},
    rotation::RotationAnimation,
    CubeRotationEvent, Rotation3x3,
};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_stepper).add_systems(
            Update,
            (
                spacebar_stepper_handler,
                random_face_rotation_on_tab,
                check_solved_on_enter,
            )
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
    let steps: Vec<CubeRotationEvent> = vec![
        // Rotation3x3::U,
        // Rotation3x3::R,
        // Rotation3x3::UPrime,
        // Rotation3x3::RPrime,
        Rotation3x3::B2,
        Rotation3x3::R2,
        Rotation3x3::UPrime,
        Rotation3x3::B2,
        Rotation3x3::L2,
        Rotation3x3::F2,
        Rotation3x3::D2,
        Rotation3x3::R2,
        Rotation3x3::DPrime,
        Rotation3x3::L,
        Rotation3x3::RPrime,
        Rotation3x3::F,
        Rotation3x3::R2,
        Rotation3x3::F,
        Rotation3x3::L,
        Rotation3x3::R,
    ]
    .iter()
    .rev()
    .map(|e| e.into())
    .collect();

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
            let new_rotation_event = CubeRotationEvent::random_face_rotation(cube);

            if new_rotation_event.negates(&last_random_face_rotation.0) {
                continue;
            }

            last_random_face_rotation.0 = new_rotation_event.clone();
            break new_rotation_event;
        },
        None => {
            let new_rotation_event = CubeRotationEvent::random_face_rotation(cube);
            commands.spawn(LastRandomFaceRotationEvent(new_rotation_event.clone()));
            new_rotation_event
        }
    };

    rotation_event.animation = Some(RotationAnimation {
        duration_in_seconds: 0.15,
    });
    event_writer.send(rotation_event);
}

fn check_solved_on_enter(pieces_query: Query<&Piece>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }

    if Cube::is_solved(pieces_query) {
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
    });
    event_writer.send(rotation_event);
}
