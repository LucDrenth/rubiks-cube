use bevy::prelude::*;

use crate::{
    cube::{
        self, CubeCommandsResource, CurrentCubeSizeResource, SequenceResource,
        MINIMUM_SUPPORTED_CUBE_SIZE,
    },
    schedules::CubeScheduleSet,
};

use super::{
    cube_actions::{ScrambleButton, ScrambleButtonProgressBar, SolveButton},
    interface::{
        CaptureClick, BUTTON_BACKGROUND_COLOR, BUTTON_BORDER, BUTTON_BORDER_RADIUS, COLOR_BLUE,
        COLOR_MAIN, DEFAULT_FONT_BOLD,
    },
    widget::{
        button::{ButtonDisabledHandler, DisableButtonEvent, EnableButtonEvent, UiButton},
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
                        border: BUTTON_BORDER,
                        ..default()
                    },
                    BorderColor(COLOR_MAIN),
                    BUTTON_BORDER_RADIUS,
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                ))
                .with_child((
                    ImageNode {
                        image: chevron_right_image.clone(),
                        color: COLOR_BLUE,
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
                        font: asset_server.load(DEFAULT_FONT_BOLD),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(BUTTON_BACKGROUND_COLOR),
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
                        border: BUTTON_BORDER,
                        ..default()
                    },
                    BorderColor(COLOR_MAIN),
                    BUTTON_BORDER_RADIUS,
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                ))
                .with_child((
                    ImageNode {
                        image: chevron_right_image.clone(),
                        color: COLOR_BLUE,
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
    button_query: Query<
        (Entity, &Interaction, &ButtonDisabledHandler),
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
    scramble_button_query: Query<Entity, With<ScrambleButton>>,
    solve_button_query: Query<Entity, With<SolveButton>>,
    mut enable_button_event_writer: EventWriter<EnableButtonEvent>,
    mut disable_button_event_writer: EventWriter<DisableButtonEvent>,
) {
    let (button_entity, interaction, disable_button) = match button_query.get_single() {
        Ok(v) => v,
        Err(_) => return,
    };

    if disable_button.is_disabled() {
        return;
    }

    if *interaction != Interaction::Pressed {
        return;
    }

    let mut cube_size_label = cube_size_label_query.get_single_mut().unwrap();
    let current_cube_size = cube_size_resource.0;

    if current_cube_size == MINIMUM_SUPPORTED_CUBE_SIZE {
        warn!("can not decrease cube size below 1");
        return;
    }

    cube_size_label.0 = (current_cube_size - 1).to_string();
    sequence_resource.set(vec![]);
    cube_size_resource.0 = current_cube_size - 1;
    commands.run_system(cube_commands.despawn);
    commands.run_system(cube_commands.spawn);

    if cube_size_resource.0 == MINIMUM_SUPPORTED_CUBE_SIZE {
        disable_button_event_writer.send(DisableButtonEvent::new(button_entity));
    }

    let (mut progress_bar, mut node) = scramble_button_progress_bar_query.get_single_mut().unwrap();
    progress_bar.cancel(&mut node);

    enable_button_event_writer.send(EnableButtonEvent::new(scramble_button_query.single()));
    enable_button_event_writer.send(EnableButtonEvent::new(solve_button_query.single()));
}

fn increase_cube_size_button_action(
    mut commands: Commands,
    increase_size_button_query: Query<
        (&Interaction, &ButtonDisabledHandler),
        (With<CubeSizeUpButton>, Changed<Interaction>),
    >,
    decrease_size_button_query: Query<Entity, With<CubeSizeDownButton>>,
    mut cube_size_label_query: Query<&mut Text, With<CubeSizeLabel>>,
    mut sequence_resource: ResMut<SequenceResource>,
    mut cube_size_resource: ResMut<CurrentCubeSizeResource>,
    cube_commands: Res<CubeCommandsResource>,
    mut scramble_button_progress_bar_query: Query<
        (&mut ProgressBar, &mut Node),
        With<ScrambleButtonProgressBar>,
    >,
    scramble_button_query: Query<Entity, With<ScrambleButton>>,
    solve_button_query: Query<Entity, With<SolveButton>>,
    mut enable_button_event_writer: EventWriter<EnableButtonEvent>,
) {
    let (interaction, disable_button) = match increase_size_button_query.get_single() {
        Ok(v) => v,
        Err(_) => return,
    };

    if disable_button.is_disabled() {
        return;
    }

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

    let (mut progress_bar, mut node) = scramble_button_progress_bar_query.get_single_mut().unwrap();
    progress_bar.cancel(&mut node);

    enable_button_event_writer.send(EnableButtonEvent::new(decrease_size_button_query.single()));
    enable_button_event_writer.send(EnableButtonEvent::new(scramble_button_query.single()));
    enable_button_event_writer.send(EnableButtonEvent::new(solve_button_query.single()));
}
