use bevy::prelude::*;

use crate::{
    cube::{
        self,
        solver::{self, SolveStrategy},
        CubeRotationAnimation, CubeState, SequenceResource,
    },
    schedules::CubeScheduleSet,
};

const COLOR_YELLOW: Color = Color::srgb(0.952, 0.784, 0.007);

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

fn init_scramble_button(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    BackgroundColor(Color::srgb_u8(56, 56, 56)),
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
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

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
                    BackgroundColor(Color::srgb_u8(56, 56, 56)),
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
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
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

    let cube = cube_query.get_single().unwrap();

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

    let cube_state = cube_state_query.get_single().unwrap();
    let mut solve_sequence = solver::get_solve_sequence(SolveStrategy::Kociemba, cube_state);
    for cube_rotation in solve_sequence.iter_mut() {
        cube_rotation.animation = Some(CubeRotationAnimation {
            duration_in_seconds: 0.35,
            ease_function: Some(EaseFunction::CubicOut),
        });
    }

    sequence_resource.set(solve_sequence);
}
