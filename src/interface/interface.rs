use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::{
    cube_actions::{self, CubeActionsPlugin},
    cube_size::{self, CubeSizePlugin},
};

pub const COLOR_YELLOW: Color = Color::srgb(0.952, 0.784, 0.007);
pub const COLOR_DARK_GREY: Color = Color::srgb(0.21875, 0.21875, 0.21875);
pub const BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

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
        .add_plugins(CubeSizePlugin)
        .add_plugins(CubeActionsPlugin)
        .add_systems(Startup, init_scramble_button)
        .add_systems(
            Update,
            (update_ui_resource, buttons_hover_effect).in_set(CubeScheduleSet::HandleUserInput),
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

fn init_scramble_button(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            cube_actions::spawn(parent, &asset_server);
            cube_size::spawn(parent, &asset_server);
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
