use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{cube::Cube, CubeRotationEvent, Rotation3x3};

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

fn random_face_rotation_on_tab(
    cube_query: Query<&Cube>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube component");
        return;
    };

    event_writer.send(CubeRotationEvent::random_face_rotation(cube));
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
