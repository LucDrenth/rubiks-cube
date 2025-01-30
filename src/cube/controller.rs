use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{cube::Cube, cube_state::CubeState, rotation::RotationAnimation, CubeRotationEvent};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SequenceResource {
            steps: vec![],
            current_step: 0,
        })
        .add_systems(
            Update,
            (
                check_solved_on_enter,
                sequence_handler,
                random_face_rotation_on_tab,
            )
                .chain()
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Resource)]
pub struct SequenceResource {
    pub steps: Vec<CubeRotationEvent>,
    current_step: usize,
}

impl SequenceResource {
    pub fn set(&mut self, steps: Vec<CubeRotationEvent>) {
        self.steps = steps;
        self.current_step = 0;
    }

    pub fn is_done(&self) -> bool {
        // TODO this will be true while still animating the last rotation
        self.current_step >= self.steps.len()
    }

    pub fn seconds_until_complete(&self) -> f32 {
        if self.is_done() {
            return 0.0;
        }

        let seconds_until_current_step_is_complete = match &self.steps[self.current_step].animation
        {
            Some(animation) => animation.duration_in_seconds, // TODO account for when animation is already ongoing
            None => 0.0,
        };

        let mut result = seconds_until_current_step_is_complete;

        for i in (self.current_step + 1)..self.steps.len() {
            if let Some(animation) = &self.steps[i].animation {
                result += animation.duration_in_seconds;
            }
        }

        return result;
    }
}

fn sequence_handler(
    cube_query: Query<&Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube component");
        return;
    };

    if cube.is_animating_rotation {
        return;
    }

    loop {
        if sequence_resource.current_step >= sequence_resource.steps.len() {
            return;
        }

        let rotation_event = sequence_resource.steps[sequence_resource.current_step].clone();
        event_writer.send(rotation_event);
        sequence_resource.current_step += 1;

        if sequence_resource.steps[sequence_resource.current_step - 1]
            .animation
            .is_some()
        {
            return;
        }
    }
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
