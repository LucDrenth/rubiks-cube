use bevy::prelude::*;

use crate::{
    cube::{self, CubeCommandsResource, CurrentCubeSizeResource, SequenceResource},
    schedules::CubeScheduleSet,
};

use super::{
    cube_actions::ScrambleButtonProgressBar,
    interface::{CaptureClick, BUTTON_TEXT_COLOR, COLOR_DARK_GREY},
    widget::{
        button::{ButtonDisabledHandler, UiButton},
        progress_bar::ProgressBar,
    },
};

pub struct CubeSizePlugin;

impl Plugin for CubeSizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                decrease_cube_size_button_action,
                increase_cube_size_button_action,
            )
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
pub struct CubeSizeDownButton;
#[derive(Component)]
pub struct CubeSizeUpButton;
#[derive(Component)]
pub struct CubeSizeLabel;

pub fn spawn(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    let chevron_right_image = asset_server.load("icons/chevron-right.png");

    parent
        .spawn((Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::ZERO.with_left(Val::Px(32.0)),
            position_type: PositionType::Absolute,
            right: Val::Px(8.0),
            ..default()
        },))
        .with_children(|parent| {
            // size-down button
            parent
                .spawn((
                    CubeSizeDownButton,
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
}

fn decrease_cube_size_button_action(
    mut commands: Commands,
    mut button_query: Query<
        (&Interaction, &mut ButtonDisabledHandler),
        (With<CubeSizeDownButton>, Changed<Interaction>),
    >,
    mut cube_size_label_query: Query<&mut Text, With<CubeSizeLabel>>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut cube_size_resource: ResMut<CurrentCubeSizeResource>,
    cube_commands: Res<CubeCommandsResource>,
    mut scramble_button_progress_bar_query: Query<
        (&mut ProgressBar, &mut Node),
        With<ScrambleButtonProgressBar>,
    >,
) {
    let (interaction, mut disable_button) = match button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if disable_button.0 {
        return;
    }

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut cube_size_label = cube_size_label_query.get_single_mut().unwrap();
    let current_cube_size = cube_size_resource.0;

    if current_cube_size == 2 {
        warn!("can not decrease cube size below 2");
        return;
    }

    cube_size_label.0 = (current_cube_size - 1).to_string();
    sequence_resource.set(vec![]);
    cube_size_resource.0 = current_cube_size - 1;
    commands.run_system(cube_commands.despawn);
    commands.run_system(cube_commands.spawn);

    if cube_size_resource.0 == 2 {
        disable_button.0 = true;
    }

    let (mut progress_bar, mut node) = scramble_button_progress_bar_query.get_single_mut().unwrap();
    progress_bar.cancel(&mut node);
}

fn increase_cube_size_button_action(
    mut commands: Commands,
    increase_size_button_query: Query<&Interaction, (With<CubeSizeUpButton>, Changed<Interaction>)>,
    mut decrease_size_button_query: Query<&mut ButtonDisabledHandler, With<CubeSizeDownButton>>,
    mut cube_size_label_query: Query<&mut Text, With<CubeSizeLabel>>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut cube_size_resource: ResMut<CurrentCubeSizeResource>,
    cube_commands: Res<CubeCommandsResource>,
    mut scramble_button_progress_bar_query: Query<
        (&mut ProgressBar, &mut Node),
        With<ScrambleButtonProgressBar>,
    >,
) {
    let interaction = match increase_size_button_query.get_single() {
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

    match decrease_size_button_query.get_single_mut() {
        Ok(mut disable_button) => {
            disable_button.0 = false;
        }
        Err(err) => {
            warn!("decrease cube size button not found: {}", err)
        }
    };

    let (mut progress_bar, mut node) = scramble_button_progress_bar_query.get_single_mut().unwrap();
    progress_bar.cancel(&mut node);
}
