use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{
    rotation::{FaceRotation, Rotation},
    CubeRotationEvent, Rotation3x3,
};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_stepper)
            .add_systems(Update, spacebar_stepper_handler.in_set(CubeScheduleSet::HandleUserInput))
            // app.add_systems(Startup, do_instant_rotations)
        ;
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
    commands.spawn(RotationStepper {
        steps: vec![
            Rotation3x3::U,
            Rotation3x3::R,
            Rotation3x3::UPrime,
            Rotation3x3::RPrime,
        ]
        .iter()
        .map(|e| e.into())
        .collect(),
        current: 0,
    });
}

fn spacebar_stepper_handler(
    mut query: Query<&mut RotationStepper>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let mut stepper = query.get_single_mut().unwrap();

    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    event_writer.send(stepper.step());
}

fn do_instant_rotations(mut event_writer: EventWriter<CubeRotationEvent>) {
    event_writer.send(CubeRotationEvent {
        rotation: Rotation::Face(FaceRotation::X(vec![1])),
        negative_direction: false,
        twice: true,
    });

    event_writer.send(CubeRotationEvent {
        rotation: Rotation::Face(FaceRotation::Y(vec![1])),
        negative_direction: false,
        twice: false,
    });

    // event_writer.send(CubeRotationEvent {
    //     rotation: Rotation::X(vec![-1]),
    //     counter_clockwise: false,
    //     twice: false,
    // });
}
