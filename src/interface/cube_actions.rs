use bevy::prelude::*;

use crate::{
    cube::{
        self,
        solver::{self, SolveStrategy},
        CubeRotationAnimation, CubeState, SequenceResource,
    },
    schedules::CubeScheduleSet,
};

use super::{
    interface::{CaptureClick, BUTTON_TEXT_COLOR, COLOR_DARK_GREY},
    widget::{button::UiButton, progress_bar::ProgressBar},
};

pub struct CubeActionsPlugin;

impl Plugin for CubeActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (scramble_button_action, solve_button_action).in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
pub struct ScrambleButton;
#[derive(Component)]
pub struct ScrambleButtonProgressBar;

#[derive(Component)]
pub struct SolveButton;

pub fn spawn(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    // scramble button
    parent
        .spawn((
            ScrambleButton,
            CaptureClick,
            UiButton,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.)),
                overflow: Overflow::clip(),
                ..default()
            },
            BorderColor(Color::srgb_u8(243, 200, 2)),
            BorderRadius::all(Val::Px(4.)),
            BackgroundColor(COLOR_DARK_GREY),
            BoxShadow {
                color: Color::BLACK,
                x_offset: Val::Px(3.),
                y_offset: Val::Px(3.),
                spread_radius: Val::Px(3.),
                blur_radius: Val::Px(1.),
            },
        ))
        .with_children(|parent| {
            // label
            parent.spawn((
                Text::new("scramble"),
                TextFont {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                Node {
                    margin: UiRect {
                        left: Val::Px(16.0),
                        right: Val::Px(16.),
                        top: Val::Px(8.),
                        bottom: Val::Px(8.),
                    },
                    ..default()
                },
                TextColor(BUTTON_TEXT_COLOR),
            ));

            // progress bar
            parent.spawn((
                ScrambleButtonProgressBar,
                ProgressBar::default(),
                Node {
                    width: Val::Percent(0.),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::ZERO,
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                BorderRadius::all(Val::Px(4.)),
            ));
        });

    // solve button
    parent
        .spawn((
            SolveButton,
            CaptureClick,
            UiButton,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(16.0),
                    right: Val::Px(16.),
                    top: Val::Px(8.),
                    bottom: Val::Px(8.),
                },
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BorderColor(Color::srgb_u8(243, 200, 2)),
            BorderRadius::px(4., 4., 4., 4.),
            BackgroundColor(COLOR_DARK_GREY),
            BoxShadow {
                color: Color::BLACK,
                x_offset: Val::Px(3.),
                y_offset: Val::Px(3.),
                spread_radius: Val::Px(3.),
                blur_radius: Val::Px(1.),
            },
        ))
        .with_child((
            Text::new("solve"),
            TextFont {
                font: asset_server.load("fonts/roboto.ttf"),
                font_size: 16.0,
                ..default()
            },
            TextColor(BUTTON_TEXT_COLOR),
        ));
}

fn scramble_button_action(
    mut scramble_button_query: Query<&Interaction, (With<ScrambleButton>, Changed<Interaction>)>,
    cube_query: Query<&cube::Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut progress_bar_query: Query<&mut ProgressBar, With<ScrambleButtonProgressBar>>,
) {
    let interaction = match scramble_button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let cube = match cube_query.get_single() {
        Ok(cube) => cube,
        Err(err) => {
            error!("failed to get cube: {err}");
            return;
        }
    };

    let scramble_length = 20;
    let rotation_duration = 0.15;

    let mut scramble_sequence = cube::create_random_scramble_sequence(cube.size(), scramble_length);
    for cube_rotation in scramble_sequence.iter_mut() {
        cube_rotation.animation = Some(CubeRotationAnimation {
            duration_in_seconds: rotation_duration,
            ease_function: Some(EaseFunction::Linear),
        });
    }

    if scramble_length > 2 {
        // ease out last rotations
        scramble_sequence[scramble_length - 2].animation = Some(CubeRotationAnimation {
            duration_in_seconds: rotation_duration * 1.3,
            ease_function: Some(EaseFunction::Linear),
        });
        scramble_sequence[scramble_length - 1].animation = Some(CubeRotationAnimation {
            duration_in_seconds: rotation_duration * 2.0,
            ease_function: Some(EaseFunction::CubicOut),
        });
    }

    let mut scramble_duration: f32 = 0.0;
    for rotation in &scramble_sequence {
        match &rotation.animation {
            Some(animation) => scramble_duration += animation.duration_in_seconds,
            None => (),
        }
    }

    sequence_resource.set(scramble_sequence);

    let mut progress_bar = match progress_bar_query.get_single_mut() {
        Ok(progress_bar) => progress_bar,
        Err(err) => {
            error!("failed to get scramble button progress bar: {err}");
            return;
        }
    };

    // scramble_duration is not exact because a rotation is measured in seconds, not in ticks.
    // For example, if a tick is 0.1 seconds and the rotation duration is 0.35, it takes 4
    // ticks (0.4 seconds) before the next rotation starts. We could calculate a more precise version,
    // but for now we'll just add 0.3 seconds to the scramble_duration to fix this.
    let progress_bar_duration = scramble_duration + 0.3;
    progress_bar.timer = Some(Timer::from_seconds(progress_bar_duration, TimerMode::Once));
}

fn solve_button_action(
    mut solve_button_query: Query<&Interaction, (With<SolveButton>, Changed<Interaction>)>,
    cube_state_query: Query<&CubeState>,
    mut sequence_resource: ResMut<SequenceResource>,
) {
    let interaction = match solve_button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let cube_state = match cube_state_query.get_single() {
        Ok(cube_state) => cube_state,
        Err(err) => {
            error!("failed to get cube state: {err}");
            return;
        }
    };

    let mut solve_sequence = solver::get_solve_sequence(SolveStrategy::Kociemba, cube_state);
    for cube_rotation in solve_sequence.iter_mut() {
        cube_rotation.animation = Some(CubeRotationAnimation {
            duration_in_seconds: 0.35,
            ease_function: Some(EaseFunction::CubicOut),
        });
    }

    sequence_resource.set(solve_sequence);
}
