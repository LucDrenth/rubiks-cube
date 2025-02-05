use bevy::prelude::*;

use crate::{
    cube::{
        self,
        solver::{self, SolveStrategy},
        CubeRotationAnimation, CubeRotationEvent, CubeState, SequenceResource,
    },
    schedules::CubeScheduleSet,
};

use super::{
    interface::{
        CaptureClick, BUTTON_BACKGROUND_COLOR, BUTTON_BORDER, BUTTON_BORDER_RADIUS,
        BUTTON_TEXT_COLOR, COLOR_MAIN, DEFAULT_FONT_BOLD,
    },
    widget::{
        self,
        button::{ButtonDisabledHandler, DisableButtonEvent, UiButton},
        dropdown::{Dropdown, DropdownOption},
        progress_bar::ProgressBar,
    },
};

const SCRAMBLING_ROTATION_SPEED: f32 = 0.15; // in seconds
const SOLVING_ROTATION_SPEED: f32 = 0.35; // in seconds

pub struct CubeActionsPlugin;

impl Plugin for CubeActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SequenceSpeedResource(SequenceSpeed::Multiplier(1.0)))
            .insert_resource(CurrentSequenceTypeResource(None))
            .add_systems(
                Update,
                (scramble_button_action, solve_button_action)
                    .chain()
                    .in_set(CubeScheduleSet::HandleUserInput),
            )
            .add_systems(
                Update,
                handle_sequence_speed_dropdown.in_set(CubeScheduleSet::HandleEvents),
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
#[derive(Component)]
struct SequenceSpeedDropdown;

#[derive(Component, Clone, Debug)]
enum SequenceSpeed {
    Multiplier(f32),
    Instant,
}

#[derive(Resource)]
struct SequenceSpeedResource(SequenceSpeed);

enum SequenceType {
    Scramble,
    Solve,
}

#[derive(Resource)]
struct CurrentSequenceTypeResource(Option<SequenceType>);

pub fn spawn(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    parent
        .spawn(Node {
            column_gap: Val::Px(8.),
            ..default()
        })
        .with_children(|parent| {
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
                    widget::dropdown::DropdownType::Select(4), // selected x1.0 by default
                ),
                SequenceSpeedDropdown,
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
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                    BoxShadow {
                        color: Color::BLACK,
                        x_offset: Val::Px(3.),
                        y_offset: Val::Px(3.),
                        spread_radius: Val::Px(3.),
                        blur_radius: Val::Px(1.),
                    },
                ))
                .with_children(|parent| {
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
                        BackgroundColor(BUTTON_BACKGROUND_COLOR),
                        BUTTON_BORDER_RADIUS,
                    ));

                    // label
                    parent.spawn((
                        Text::new("scramble"),
                        TextFont {
                            font: asset_server.load(DEFAULT_FONT_BOLD),
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
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                    BoxShadow {
                        color: Color::BLACK,
                        x_offset: Val::Px(3.),
                        y_offset: Val::Px(3.),
                        spread_radius: Val::Px(3.),
                        blur_radius: Val::Px(1.),
                    },
                ))
                .with_children(|parent| {
                    // progress bar
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
                        BackgroundColor(BUTTON_BACKGROUND_COLOR),
                        BorderRadius::all(Val::Px(4.)),
                    ));

                    // label
                    parent.spawn((
                        Text::new("solve"),
                        TextFont {
                            font: asset_server.load(DEFAULT_FONT_BOLD),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
                });
        });
}

fn scramble_button_action(
    scramble_button_query: Query<
        (Entity, &Interaction, &ButtonDisabledHandler),
        (With<ScrambleButton>, Changed<Interaction>),
    >,
    solve_button_query: Query<Entity, With<SolveButton>>,
    cube_query: Query<&cube::Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut progress_bar_query: Query<&mut ProgressBar, With<ScrambleButtonProgressBar>>,
    sequence_speed: Res<SequenceSpeedResource>,
    mut sequence_type: ResMut<CurrentSequenceTypeResource>,
    mut disable_button_event_writer: EventWriter<DisableButtonEvent>,
    time: Res<Time>,
) {
    let Ok((scramble_button_entity, interaction, disabled_handler)) =
        scramble_button_query.get_single()
    else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    if disabled_handler.is_disabled() {
        return;
    }

    let Ok(cube) = cube_query.get_single() else {
        error!("scramble_button_action: failed to get cube");
        return;
    };

    let scramble_length = (cube.size().0 + 1) as usize * 6;

    let mut scramble_sequence = cube::create_random_scramble_sequence(cube.size(), scramble_length);

    match sequence_speed.0 {
        SequenceSpeed::Multiplier(multiplier) => {
            for cube_rotation in scramble_sequence.iter_mut() {
                cube_rotation.animation = Some(CubeRotationAnimation {
                    duration_in_seconds: SCRAMBLING_ROTATION_SPEED / multiplier,
                    ease_function: Some(EaseFunction::Linear),
                });
            }

            ease_out_scramble_sequence(&mut scramble_sequence);
        }
        SequenceSpeed::Instant => (),
    }

    let mut scramble_duration: f32 = 0.0;
    for rotation in &scramble_sequence {
        if let Some(animation) = &rotation.animation {
            scramble_duration += animation.duration_in_seconds;
        }
    }

    sequence_resource.set(scramble_sequence);

    if scramble_duration == 0.0 {
        return;
    }

    let Ok(mut progress_bar) = progress_bar_query.get_single_mut() else {
        error!("scramble_button_action: failed to get scramble button progress bar");
        return;
    };

    // we subtract one tick because the first tick of the cube rotation animation will already be performed in the current frame.
    let progress_bar_duration = scramble_duration - time.delta_secs();
    progress_bar.set_timer(Timer::from_seconds(progress_bar_duration, TimerMode::Once));

    disable_button_event_writer.send(DisableButtonEvent {
        entity: scramble_button_entity,
        enable_after: Some(progress_bar_duration),
    });
    disable_button_event_writer.send(DisableButtonEvent {
        entity: solve_button_query.get_single().unwrap(),
        enable_after: Some(progress_bar_duration),
    });

    sequence_type.0 = Some(SequenceType::Scramble);
}

fn solve_button_action(
    solve_button_query: Query<
        (Entity, &Interaction, &ButtonDisabledHandler),
        (With<SolveButton>, Changed<Interaction>),
    >,
    scramble_button_query: Query<Entity, With<ScrambleButton>>,
    cube_state_query: Query<&CubeState>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut progress_bar_query: Query<&mut ProgressBar, With<SolveButtonProgressBar>>,
    sequence_speed: Res<SequenceSpeedResource>,
    mut sequence_type: ResMut<CurrentSequenceTypeResource>,
    mut disable_button_event_writer: EventWriter<DisableButtonEvent>,
    time: Res<Time>,
) {
    let Ok((solve_button_entity, interaction, disabled_handler)) = solve_button_query.get_single()
    else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    if disabled_handler.is_disabled() {
        return;
    }

    let Ok(cube_state) = cube_state_query.get_single() else {
        error!("solve_button_action: failed to get cube state");
        return;
    };

    let mut solve_sequence = solver::get_solve_sequence(SolveStrategy::Kociemba, cube_state);

    match sequence_speed.0 {
        SequenceSpeed::Multiplier(multiplier) => {
            for cube_rotation in solve_sequence.iter_mut() {
                cube_rotation.animation = Some(CubeRotationAnimation {
                    duration_in_seconds: SOLVING_ROTATION_SPEED / multiplier,
                    ease_function: Some(EaseFunction::CubicOut),
                });
            }
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

    if solve_duration == 0.0 {
        return;
    }

    let Ok(mut progress_bar) = progress_bar_query.get_single_mut() else {
        error!("solve_button_action: failed to get solve button progress bar");
        return;
    };

    // we subtract one tick because the first tick of the cube rotation animation will already be performed in the current frame.
    let progress_bar_duration = solve_duration - time.delta_secs();
    progress_bar.set_timer(Timer::from_seconds(progress_bar_duration, TimerMode::Once));

    disable_button_event_writer.send(DisableButtonEvent {
        entity: scramble_button_query.get_single().unwrap(),
        enable_after: Some(progress_bar_duration),
    });
    disable_button_event_writer.send(DisableButtonEvent {
        entity: solve_button_entity,
        enable_after: Some(progress_bar_duration),
    });

    sequence_type.0 = Some(SequenceType::Solve);
}

fn handle_sequence_speed_dropdown(
    query: Query<(&SequenceSpeed, &Interaction), Changed<Interaction>>,
    mut sequence_speed_resource: ResMut<SequenceSpeedResource>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut scramble_button_progress_bar_query: Query<
        &mut ProgressBar,
        (
            With<ScrambleButtonProgressBar>,
            Without<SolveButtonProgressBar>,
        ),
    >,
    mut solve_button_progress_bar_query: Query<
        &mut ProgressBar,
        (
            With<SolveButtonProgressBar>,
            Without<ScrambleButtonProgressBar>,
        ),
    >,
    scramble_button_entity_query: Query<Entity, With<ScrambleButton>>,
    solve_button_entity_query: Query<Entity, With<SolveButton>>,
    sequence_type_resource: Res<CurrentSequenceTypeResource>,
    mut disable_button_event_writer: EventWriter<DisableButtonEvent>,
) {
    let Ok((new_sequence_speed, interaction)) = query.get_single() else {
        return;
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    sequence_speed_resource.0 = new_sequence_speed.clone();

    if sequence_resource.is_done() {
        return;
    }

    let sequence_type = match &sequence_type_resource.0 {
        Some(sequence_type) => sequence_type,
        None => return,
    };

    // update sequence
    {
        let rotation_speed = match sequence_type {
            SequenceType::Scramble => SCRAMBLING_ROTATION_SPEED,
            SequenceType::Solve => SOLVING_ROTATION_SPEED,
        };
        let ease_function = EaseFunction::Linear;

        for sequence_step in &mut sequence_resource.steps {
            match sequence_speed_resource.0 {
                SequenceSpeed::Multiplier(multipler) => {
                    sequence_step.animation = Some(CubeRotationAnimation {
                        duration_in_seconds: rotation_speed / multipler,
                        ease_function: Some(ease_function),
                    })
                }
                SequenceSpeed::Instant => {
                    sequence_step.animation = None;
                }
            }
        }

        ease_out_scramble_sequence(&mut sequence_resource.steps);
    }

    // update progress bar
    {
        let mut progress_bar = match sequence_type {
            SequenceType::Scramble => scramble_button_progress_bar_query.get_single_mut().unwrap(),
            SequenceType::Solve => solve_button_progress_bar_query.get_single_mut().unwrap(),
        };

        let progress_bar_duration = match sequence_speed_resource.0 {
            SequenceSpeed::Multiplier(_) => sequence_resource.seconds_until_complete(),
            SequenceSpeed::Instant => 0.0,
        };

        progress_bar.update_timer(progress_bar_duration);

        disable_button_event_writer.send(DisableButtonEvent {
            entity: scramble_button_entity_query.get_single().unwrap(),
            enable_after: Some(progress_bar_duration),
        });
        disable_button_event_writer.send(DisableButtonEvent {
            entity: solve_button_entity_query.get_single().unwrap(),
            enable_after: Some(progress_bar_duration),
        });
    }
}

fn ease_out_scramble_sequence(sequence: &mut Vec<CubeRotationEvent>) {
    if sequence.len() < 2 {
        return;
    }

    let sequence_len = sequence.len();

    match &mut sequence[sequence_len - 2].animation {
        Some(animation) => {
            animation.duration_in_seconds *= 1.3;
        }
        None => (),
    }

    match &mut sequence[sequence_len - 1].animation {
        Some(animation) => {
            animation.duration_in_seconds *= 2.0;
            animation.ease_function = Some(EaseFunction::CubicOut);
        }
        None => (),
    }
}
