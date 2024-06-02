use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{cube::Cube, rotation::RotationAnimation, CubeRotationEvent, Rotation3x3};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_stepper).add_systems(
            Update,
            (spacebar_stepper_handler, random_face_rotation_on_tab)
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
        Rotation3x3::U,
        Rotation3x3::R,
        Rotation3x3::UPrime,
        Rotation3x3::RPrime,
    ]
    .iter()
    .map(|e| e.into())
    .collect();

    commands.spawn(RotationStepper { steps, current: 0 });
}

fn random_face_rotation_on_tab(
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

    let mut rotation_event = CubeRotationEvent::random_face_rotation(cube);
    rotation_event.animation = Some(RotationAnimation {
        duration_in_seconds: 0.15,
    });

    event_writer.send(rotation_event);
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
