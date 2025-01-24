pub use bevy::prelude::*;

use crate::{
    interface::interface::{COLOR_DARK_GREY, COLOR_YELLOW},
    schedules::CubeScheduleSet,
};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (buttons_hover_effect, buttons_disable_handler)
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
#[require(Button, ButtonDisabledHandler)]
pub struct UiButton;

#[derive(Component)]
pub struct ButtonDisabledHandler(pub bool);

impl Default for ButtonDisabledHandler {
    fn default() -> Self {
        Self(false)
    }
}

fn buttons_hover_effect(
    mut query: Query<
        (&Interaction, &ButtonDisabledHandler, &mut BorderColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, is_disabled, mut border_color) in query.iter_mut() {
        if is_disabled.0 {
            continue;
        }

        handle_button_interaction_state(interaction, &mut border_color);
    }
}

fn handle_button_interaction_state(interaction: &Interaction, border_color: &mut BorderColor) {
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

fn buttons_disable_handler(
    mut query: Query<
        (
            &mut BackgroundColor,
            &mut BorderColor,
            &Interaction,
            &ButtonDisabledHandler,
        ),
        Changed<ButtonDisabledHandler>,
    >,
) {
    for (mut background_color, mut border_color, interaction, is_disabled) in query.iter_mut() {
        if is_disabled.0 {
            background_color.0 = Color::NONE;
            handle_button_interaction_state(&Interaction::None, &mut border_color);
        } else {
            background_color.0 = COLOR_DARK_GREY;
            handle_button_interaction_state(interaction, &mut border_color);
        }
    }
}
