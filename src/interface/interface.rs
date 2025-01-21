use bevy::{log, prelude::*};

use crate::{
    cube::{
        self,
        solver::{self, SolveStrategy},
        CubeCommandsResource, CubeRotationAnimation, CubeState, CurrentCubeSizeResource,
        SequenceResource,
    },
    schedules::CubeScheduleSet,
};

const COLOR_YELLOW: Color = Color::srgb(0.952, 0.784, 0.007);
const COLOR_DARK_GREY: Color = Color::srgb(0.21875, 0.21875, 0.21875);
const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Resource)]
pub struct UiResource {
    pub did_handle_click: bool,
}

/// Add this component to a ui element to not let a click event bubble up to the world
#[derive(Component)]
pub struct CaptureClick;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiResource {
            did_handle_click: false,
        })
        .add_systems(Startup, init_scramble_button)
        .add_systems(
            Update,
            (
                update_ui_resource,
                buttons_hover_effect,
                scramble_button_action,
                solve_button_action,
                decrease_cube_size_button_action,
                increase_cube_size_button_action,
            )
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

fn update_ui_resource(
    mut ui_resource: ResMut<UiResource>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    capture_click_query: Query<&Interaction, (With<CaptureClick>, Changed<Interaction>)>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        ui_resource.did_handle_click = false;
    }

    for interaction in capture_click_query.iter() {
        match interaction {
            Interaction::Pressed => {
                ui_resource.did_handle_click = true;
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

#[derive(Component)]
pub struct ScrambleButton;
#[derive(Component)]
pub struct SolveButton;
#[derive(Component)]
pub struct CubeSizeDownButton;
#[derive(Component)]
pub struct CubeSizeUpButton;
#[derive(Component)]
pub struct CubeSizeLabel;

fn init_scramble_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let chevron_right_image = asset_server.load("icons/chevron-right.png");

    // background element
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                padding: UiRect::px(16.0, 16.0, 8.0, 8.0),
                width: Val::Percent(100.),
                column_gap: Val::Px(8.),
                border: UiRect {
                    left: Val::ZERO,
                    right: Val::ZERO,
                    top: Val::ZERO,
                    bottom: Val::Px(2.0),
                },
                ..default()
            },
            BackgroundColor(Color::srgb_u8(155, 155, 155)),
            BorderColor(COLOR_YELLOW),
        ))
        .with_children(|parent| {
            // scramble button
            parent
                .spawn((
                    ScrambleButton,
                    CaptureClick,
                    Button,
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
                    Text::new("scramble"),
                    TextFont {
                        font: asset_server.load("fonts/roboto.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(BUTTON_TEXT_COLOR),
                ));

            // solve button
            parent
                .spawn((
                    SolveButton,
                    CaptureClick,
                    Button,
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

            // cube size controls
            parent
                .spawn((
                    // TODO maximum margin-left
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::ZERO.with_left(Val::Px(32.0)),
                        position_type: PositionType::Absolute,
                        right: Val::Px(8.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // size-down button
                    parent
                        .spawn((
                            CubeSizeDownButton,
                            CaptureClick,
                            Button,
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
                        ))
                        .with_child((
                            ImageNode {
                                image: chevron_right_image.clone(),
                                color: BUTTON_TEXT_COLOR,
                                flip_x: true,
                                ..default()
                            },
                            Node {
                                width: Val::Px(16.0),
                                height: Val::Px(16.0),
                                ..default()
                            },
                        ));

                    // cube size indicator
                    parent
                        .spawn(Node {
                            margin: UiRect::horizontal(Val::Px(8.0)),
                            ..default()
                        })
                        .with_child((
                            CubeSizeLabel,
                            Text::new(cube::DEFAULT_CUBE_SIZE.to_string()),
                            TextFont {
                                font: asset_server.load("fonts/roboto-bold.ttf"),
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(COLOR_DARK_GREY),
                        ));

                    // size-up button
                    parent
                        .spawn((
                            CubeSizeUpButton,
                            CaptureClick,
                            Button,
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
                        ))
                        .with_child((
                            ImageNode {
                                image: chevron_right_image.clone(),
                                color: BUTTON_TEXT_COLOR,
                                ..default()
                            },
                            Node {
                                width: Val::Px(16.0),
                                height: Val::Px(16.0),
                                ..default()
                            },
                        ));
                });
        });
}

fn buttons_hover_effect(mut query: Query<(&Interaction, &mut BorderColor), Changed<Interaction>>) {
    for (interaction, mut border_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => (),
            Interaction::Hovered => {
                border_color.0 = COLOR_YELLOW;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        };
    }
}

fn scramble_button_action(
    mut scramble_button_query: Query<&Interaction, (With<ScrambleButton>, Changed<Interaction>)>,
    cube_query: Query<&cube::Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
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
            log::error!("failed to get cube: {err}");
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

    sequence_resource.set(scramble_sequence);
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
            log::error!("failed to get cube state: {err}");
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

fn decrease_cube_size_button_action(
    mut commands: Commands,
    mut button_query: Query<&Interaction, (With<CubeSizeDownButton>, Changed<Interaction>)>,
    mut cube_size_label_query: Query<&mut Text, With<CubeSizeLabel>>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut cube_size_resource: ResMut<CurrentCubeSizeResource>,
    cube_commands: Res<CubeCommandsResource>,
) {
    let interaction = match button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut cube_size_label = cube_size_label_query.get_single_mut().unwrap();
    let current_cube_size = cube_size_resource.0;

    if current_cube_size == 2 {
        log::warn!("can not decrease cube size below 2");
        return;
    }

    cube_size_label.0 = (current_cube_size - 1).to_string();
    sequence_resource.set(vec![]);
    cube_size_resource.0 = current_cube_size - 1;
    commands.run_system(cube_commands.despawn);
    commands.run_system(cube_commands.spawn);
}

fn increase_cube_size_button_action(
    mut commands: Commands,
    mut button_query: Query<&Interaction, (With<CubeSizeUpButton>, Changed<Interaction>)>,
    mut cube_size_label_query: Query<&mut Text, With<CubeSizeLabel>>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut cube_size_resource: ResMut<CurrentCubeSizeResource>,
    cube_commands: Res<CubeCommandsResource>,
) {
    let interaction = match button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut cube_size_label = cube_size_label_query.get_single_mut().unwrap();
    let current_cube_size = cube_size_resource.0;

    cube_size_label.0 = (current_cube_size + 1).to_string();
    sequence_resource.set(vec![]);
    cube_size_resource.0 = current_cube_size + 1;
    commands.run_system(cube_commands.despawn);
    commands.run_system(cube_commands.spawn);
}
