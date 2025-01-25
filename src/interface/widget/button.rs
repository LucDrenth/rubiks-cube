pub use bevy::prelude::*;

use crate::{
    interface::interface::{COLOR_DARK_GREY, COLOR_MAIN},
    schedules::CubeScheduleSet,
};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            buttons_disable_timer_handler.in_set(CubeScheduleSet::InterfaceTimer),
        )
        .add_systems(
            Update,
            (buttons_hover_effect, buttons_disable_handler)
                .chain()
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component)]
#[require(Button, ButtonDisabledHandler)]
pub struct UiButton;

#[derive(Component)]
#[require(ButtonDisabledHandlerTimer)]
pub struct ButtonDisabledHandler {
    pub disabled: bool,
}

#[derive(Component)]
pub struct ButtonDisabledHandlerTimer(Option<(Timer, bool)>);

impl ButtonDisabledHandlerTimer {
    pub fn enable_after(&mut self, seconds: f32) {
        self.0 = Some((Timer::from_seconds(seconds, TimerMode::Once), false));
    }

    pub fn disable_after(&mut self, seconds: f32) {
        self.0 = Some((Timer::from_seconds(seconds, TimerMode::Once), true));
    }
}

impl Default for ButtonDisabledHandler {
    fn default() -> Self {
        Self { disabled: false }
    }
}

impl Default for ButtonDisabledHandlerTimer {
    fn default() -> Self {
        Self(None)
    }
}

fn buttons_hover_effect(
    mut query: Query<
        (&Interaction, &ButtonDisabledHandler, &mut BorderColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, is_disabled, mut border_color) in query.iter_mut() {
        if is_disabled.disabled {
            continue;
        }

        handle_button_interaction_state(interaction, &mut border_color);
    }
}

fn handle_button_interaction_state(interaction: &Interaction, border_color: &mut BorderColor) {
    match interaction {
        Interaction::Pressed => (),
        Interaction::Hovered => {
            border_color.0 = COLOR_MAIN;
        }
        Interaction::None => {
            border_color.0 = Color::BLACK;
        }
    };
}

fn buttons_disable_timer_handler(
    mut query: Query<(&mut ButtonDisabledHandler, &mut ButtonDisabledHandlerTimer)>,
    time: Res<Time>,
) {
    for (mut button_disabled_handler, mut button_disabled_handler_timer) in query.iter_mut() {
        match &mut button_disabled_handler_timer.0 {
            Some((timer, state_after_timer)) => {
                timer.tick(time.delta());

                if timer.finished() {
                    button_disabled_handler.disabled = *state_after_timer;
                    button_disabled_handler_timer.0 = None;
                }
            }
            None => (),
        }
    }
}

fn buttons_disable_handler(
    mut query: Query<
        (
            &mut BackgroundColor,
            &mut BorderColor,
            &Interaction,
            &ButtonDisabledHandler,
            Option<&mut BoxShadow>,
        ),
        Changed<ButtonDisabledHandler>,
    >,
) {
    for (mut background_color, mut border_color, interaction, is_disabled, box_shadow) in
        query.iter_mut()
    {
        if is_disabled.disabled {
            background_color.0 = Color::NONE;

            match box_shadow {
                Some(mut box_shadow) => {
                    box_shadow.color = Color::NONE;
                }
                None => (),
            }

            handle_button_interaction_state(&Interaction::None, &mut border_color);
        } else {
            // TODO get this color from element
            background_color.0 = COLOR_DARK_GREY;

            match box_shadow {
                Some(mut box_shadow) => {
                    // TODO get this color from element
                    box_shadow.color = Color::BLACK;
                }
                None => (),
            }

            handle_button_interaction_state(interaction, &mut border_color);
        }
    }
}
