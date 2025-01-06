use bevy::prelude::*;

use crate::{
    cube::{self, CubeRotationAnimation, SequenceResource},
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
            (update_ui_resource, scramble_button_action).in_set(CubeScheduleSet::HandleUserInput),
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

fn init_scramble_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            ScrambleButton,
            CaptureClick,
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(18.),
                right: Val::Px(16.),
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
}

fn scramble_button_action(
    mut scramble_button_query: Query<
        (&Interaction, &mut BorderColor),
        (With<ScrambleButton>, Changed<Interaction>),
    >,
    cube_query: Query<&cube::Cube>,
    mut sequence_resource: ResMut<SequenceResource>,
) {
    let (interaction, mut border_color) = match scramble_button_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    let cube = cube_query.get_single().unwrap();

    match interaction {
        Interaction::Pressed => {
            let scramble_length = 20;
            let rotation_duration = 0.15;

            let mut scramble_sequence =
                cube::create_random_scramble_sequence(cube.size(), scramble_length);
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
        Interaction::Hovered => {
            border_color.0 = COLOR_YELLOW;
        }
        Interaction::None => {
            border_color.0 = Color::BLACK;
        }
    };
}
