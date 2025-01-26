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
    interface::{
        CaptureClick, BUTTON_BORDER, BUTTON_BORDER_RADIUS, BUTTON_TEXT_COLOR, COLOR_DARK_GREY,
        COLOR_MAIN,
    },
    widget::{
        self,
        button::{ButtonDisabledHandler, ButtonDisabledHandlerTimer, UiButton},
        dropdown::{Dropdown, DropdownOption},
        progress_bar::ProgressBar,
    },
};

pub struct CubeActionsPlugin;

impl Plugin for CubeActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SequenceSpeedResource(SequenceSpeed::Multiplier(1.0)))
            .add_systems(
                Update,
                (
                    handle_sequence_speed_dropdown,
                    scramble_button_action,
                    solve_button_action,
                )
                    .chain()
                    .in_set(CubeScheduleSet::HandleUserInput),
            );
    }
}

#[derive(Component)]
pub struct ScrambleButton;
#[derive(Component)]
pub struct ScrambleButtonProgressBar;

#[derive(Component)]
pub struct SolveButton;
#[derive(Component)]
pub struct SolveButtonProgressBar;

#[derive(Component, Clone, Debug)]
enum SequenceSpeed {
    Multiplier(f32),
    Instant,
}

#[derive(Resource)]
struct SequenceSpeedResource(SequenceSpeed);

pub fn spawn(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    // sequence speed dropdown
    widget::dropdown::spawn::<SequenceSpeed>(
        Dropdown::new(
            vec![
                DropdownOption {
                    label: "instant".to_string(),
                    value: SequenceSpeed::Instant,
                },
                DropdownOption {
                    label: "x2.5".to_string(),
                    value: SequenceSpeed::Multiplier(2.5),
                },
                DropdownOption {
                    label: "x2.0".to_string(),
                    value: SequenceSpeed::Multiplier(2.),
                },
                DropdownOption {
                    label: "x1.5".to_string(),
                    value: SequenceSpeed::Multiplier(1.5),
                },
                DropdownOption {
                    label: "x1.0".to_string(),
                    value: SequenceSpeed::Multiplier(1.),
                },
                DropdownOption {
                    label: "x0.5".to_string(),
                    value: SequenceSpeed::Multiplier(0.5),
                },
                DropdownOption {
                    label: "x0.25".to_string(),
                    value: SequenceSpeed::Multiplier(0.25),
                },
            ],
            4, // x1.0
        ),
        parent,
        asset_server,
    );

    // scramble button
    parent
        .spawn((
            ScrambleButton,
            CaptureClick,
            UiButton,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: BUTTON_BORDER,
                overflow: Overflow::clip(),
                ..default()
            },
            BorderColor(COLOR_MAIN),
            BUTTON_BORDER_RADIUS,
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
                BUTTON_BORDER_RADIUS,
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
                border: BUTTON_BORDER,
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
        .with_children(|parent| {
            parent.spawn((
                Text::new("solve"),
                TextFont {
                    font: asset_server.load("fonts/roboto.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(BUTTON_TEXT_COLOR),
            ));

            parent.spawn((
                SolveButtonProgressBar,
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
}

fn scramble_button_action(
    mut scramble_button_query: Query<&Interaction, (With<ScrambleButton>, Changed<Interaction>)>,
    mut scramble_button_disable_handler_query: Query<
        &mut ButtonDisabledHandler,
        (With<ScrambleButton>, Without<SolveButton>),
    >,
    mut scramble_button_disable_handler_timer: Query<
        &mut ButtonDisabledHandlerTimer,
        (With<ScrambleButton>, Without<SolveButton>),
    >,
    mut solve_button_disable_handler_query: Query<
        &mut ButtonDisabledHandler,
        (With<SolveButton>, Without<ScrambleButton>),
    >,
    mut solve_button_disable_handler_timer: Query<
        &mut ButtonDisabledHandlerTimer,
        (With<SolveButton>, Without<ScrambleButton>),
    >,
    cube_query: Query<&cube::Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut progress_bar_query: Query<&mut ProgressBar, With<ScrambleButtonProgressBar>>,
    sequence_speed: Res<SequenceSpeedResource>,
) {
    let interaction = match scramble_button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut disable_button = scramble_button_disable_handler_query
        .get_single_mut()
        .unwrap();

    if disable_button.disabled {
        return;
    }

    let cube = match cube_query.get_single() {
        Ok(cube) => cube,
        Err(err) => {
            error!("failed to get cube: {err}");
            return;
        }
    };

    let scramble_length = (cube.size().0 + 1) as usize * 6;
    let rotation_duration = 0.15;

    let mut scramble_sequence = cube::create_random_scramble_sequence(cube.size(), scramble_length);
    let mut scramble_speed_multiplier = 1.0;

    match sequence_speed.0 {
        SequenceSpeed::Multiplier(multiplier) => {
            for cube_rotation in scramble_sequence.iter_mut() {
                cube_rotation.animation = Some(CubeRotationAnimation {
                    duration_in_seconds: rotation_duration / multiplier,
                    ease_function: Some(EaseFunction::Linear),
                });
            }

            if scramble_length > 2 {
                // ease out last rotations
                scramble_sequence[scramble_length - 2].animation = Some(CubeRotationAnimation {
                    duration_in_seconds: rotation_duration * 1.3 / multiplier,
                    ease_function: Some(EaseFunction::Linear),
                });
                scramble_sequence[scramble_length - 1].animation = Some(CubeRotationAnimation {
                    duration_in_seconds: rotation_duration * 2.0 / multiplier,
                    ease_function: Some(EaseFunction::CubicOut),
                });
            }

            scramble_speed_multiplier = multiplier;
        }
        SequenceSpeed::Instant => (),
    }

    let mut scramble_duration: f32 = 0.0;
    for rotation in &scramble_sequence {
        match &rotation.animation {
            Some(animation) => scramble_duration += animation.duration_in_seconds,
            None => (),
        }
    }

    sequence_resource.set(scramble_sequence);

    if scramble_duration == 0.0 {
        return;
    }

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
    let progress_bar_duration = scramble_duration + (0.3 / scramble_speed_multiplier);
    progress_bar.set_timer(Timer::from_seconds(progress_bar_duration, TimerMode::Once));

    let mut solve_button_disable_handler =
        solve_button_disable_handler_query.get_single_mut().unwrap();

    solve_button_disable_handler.disabled = true;
    disable_button.disabled = true;

    solve_button_disable_handler_timer
        .get_single_mut()
        .unwrap()
        .enable_after(progress_bar_duration);
    scramble_button_disable_handler_timer
        .get_single_mut()
        .unwrap()
        .enable_after(progress_bar_duration);
}

fn solve_button_action(
    mut solve_button_query: Query<&Interaction, (With<SolveButton>, Changed<Interaction>)>,
    mut solve_button_disable_handler_query: Query<
        &mut ButtonDisabledHandler,
        (With<SolveButton>, Without<ScrambleButton>),
    >,
    mut solve_button_disable_handler_timer: Query<
        &mut ButtonDisabledHandlerTimer,
        (With<SolveButton>, Without<ScrambleButton>),
    >,
    mut scramble_button_disable_handler_query: Query<
        &mut ButtonDisabledHandler,
        (With<ScrambleButton>, Without<SolveButton>),
    >,
    mut scramble_button_disable_handler_timer: Query<
        &mut ButtonDisabledHandlerTimer,
        (With<ScrambleButton>, Without<SolveButton>),
    >,
    cube_state_query: Query<&CubeState>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut progress_bar_query: Query<&mut ProgressBar, With<SolveButtonProgressBar>>,
    sequence_speed: Res<SequenceSpeedResource>,
) {
    let interaction = match solve_button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut disable_button = solve_button_disable_handler_query.get_single_mut().unwrap();

    if disable_button.disabled {
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

    let mut rotation_speed_multiplier = 1.0;
    match sequence_speed.0 {
        SequenceSpeed::Multiplier(multiplier) => {
            for cube_rotation in solve_sequence.iter_mut() {
                cube_rotation.animation = Some(CubeRotationAnimation {
                    duration_in_seconds: 0.35 / multiplier,
                    ease_function: Some(EaseFunction::CubicOut),
                });
            }

            rotation_speed_multiplier = multiplier;
        }
        SequenceSpeed::Instant => (),
    }

    let mut solve_duration: f32 = 0.0;
    for rotation in &solve_sequence {
        match &rotation.animation {
            Some(animation) => solve_duration += animation.duration_in_seconds,
            None => (),
        }
    }

    sequence_resource.set(solve_sequence);

    let mut progress_bar = match progress_bar_query.get_single_mut() {
        Ok(progress_bar) => progress_bar,
        Err(err) => {
            error!("failed to get solve button progress bar: {err}");
            return;
        }
    };

    if solve_duration == 0.0 {
        return;
    }

    // solve_duration is not exact because a rotation is measured in seconds, not in ticks.
    // For example, if a tick is 0.1 seconds and the rotation duration is 0.35, it takes 4
    // ticks (0.4 seconds) before the next rotation starts. We could calculate a more precise version,
    // but for now we'll just add 0.3 seconds to the solve_duration to fix this.
    let progress_bar_duration = solve_duration + (0.3 / rotation_speed_multiplier);
    progress_bar.set_timer(Timer::from_seconds(progress_bar_duration, TimerMode::Once));

    let mut scramble_button_disable_handler = scramble_button_disable_handler_query
        .get_single_mut()
        .unwrap();

    scramble_button_disable_handler.disabled = true;
    disable_button.disabled = true;

    solve_button_disable_handler_timer
        .get_single_mut()
        .unwrap()
        .enable_after(progress_bar_duration);
    scramble_button_disable_handler_timer
        .get_single_mut()
        .unwrap()
        .enable_after(progress_bar_duration);
}

fn handle_sequence_speed_dropdown(
    query: Query<(&SequenceSpeed, &Interaction), Changed<Interaction>>,
    mut sequence_speed_resource: ResMut<SequenceSpeedResource>,
) {
    let (new_sequence_speed, interaction) = match query.get_single() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    sequence_speed_resource.0 = new_sequence_speed.clone();
}
