pub use bevy::prelude::*;

use crate::{
    interface::interface::{BUTTON_BACKGROUND_COLOR, BUTTON_TEXT_COLOR, COLOR_BLUE, COLOR_MAIN},
    schedules::CubeScheduleSet,
};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisableButtonEvent>()
            .add_event::<EnableButtonEvent>()
            .add_systems(
                Update,
                buttons_disable_timer_handler.in_set(CubeScheduleSet::Timers),
            )
            .add_systems(
                Update,
                (
                    handle_disable_button_event,
                    handle_enable_button_event,
                    buttons_disable_handler,
                )
                    .chain()
                    .in_set(CubeScheduleSet::HandleEvents),
            )
            .add_systems(
                Update,
                buttons_hover_effect.in_set(CubeScheduleSet::HandleUserInput),
            );
    }
}

#[derive(Event)]
pub struct DisableButtonEvent {
    pub entity: Entity,
    /// in seconds
    pub enable_after: Option<f32>,
}

impl DisableButtonEvent {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            enable_after: None,
        }
    }
}

#[derive(Event)]
pub struct EnableButtonEvent {
    pub entity: Entity,
    /// in seconds
    pub disable_after: Option<f32>,
}

impl EnableButtonEvent {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            disable_after: None,
        }
    }
}

#[derive(Component)]
#[require(Button, ButtonDisabledHandler)]
pub struct UiButton;

#[derive(Component)]
#[require(ButtonDisabledHandlerTimer)]
/// Button can be enabled and disabled through `EnableButtonEvent` and `DisableButtonEvent`
pub struct ButtonDisabledHandler {
    disabled: bool,
    disabled_last_frame: bool,
}

impl ButtonDisabledHandler {
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
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
        Self {
            disabled: false,
            disabled_last_frame: false,
        }
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
    mut button_query: Query<
        (
            Entity,
            &mut BackgroundColor,
            &mut BorderColor,
            &Interaction,
            &mut ButtonDisabledHandler,
            Option<&mut BoxShadow>,
        ),
        Changed<ButtonDisabledHandler>,
    >,
    mut text_query: Query<(&Parent, &mut TextColor)>,
    mut icon_query: Query<(&Parent, &mut ImageNode)>,
) {
    for (
        entity,
        mut background_color,
        mut border_color,
        interaction,
        mut is_disabled,
        box_shadow,
    ) in button_query.iter_mut()
    {
        let maybe_text = text_query
            .iter_mut()
            .find(|(parent, _)| parent.get() == entity);
        let maybe_icon = icon_query
            .iter_mut()
            .find(|(parent, _)| parent.get() == entity);

        if is_disabled.disabled && !is_disabled.disabled_last_frame {
            // set disabled state

            background_color.0 = Color::NONE;

            if let Some((_, mut text_color)) = maybe_text {
                text_color.0 = Color::BLACK;
            }
            if let Some((_, mut icon)) = maybe_icon {
                icon.color = Color::BLACK;
            }

            match box_shadow {
                Some(mut box_shadow) => {
                    box_shadow.color = Color::NONE;
                }
                None => (),
            }

            handle_button_interaction_state(&Interaction::None, &mut border_color);
        } else if !is_disabled.disabled && is_disabled.disabled_last_frame {
            // set enabled state

            // TODO get this color from element
            background_color.0 = BUTTON_BACKGROUND_COLOR;

            if let Some((_, mut text_color)) = maybe_text {
                text_color.0 = BUTTON_TEXT_COLOR;
            }
            if let Some((_, mut icon)) = maybe_icon {
                icon.color = COLOR_BLUE;
            }

            match box_shadow {
                Some(mut box_shadow) => {
                    // TODO get this color from element
                    box_shadow.color = Color::BLACK;
                }
                None => (),
            }

            handle_button_interaction_state(interaction, &mut border_color);
        }

        is_disabled.disabled_last_frame = is_disabled.disabled;
    }
}

fn handle_disable_button_event(
    mut event_reader: EventReader<DisableButtonEvent>,
    mut button_query: Query<(&mut ButtonDisabledHandler, &mut ButtonDisabledHandlerTimer)>,
) {
    for event in event_reader.read() {
        let Ok((mut disabled_handler, mut disabled_handler_timer)) =
            button_query.get_mut(event.entity)
        else {
            error!(
                "received handle_disable_button_event with invalid entity: {}",
                event.entity
            );
            continue;
        };

        disabled_handler.disabled = true;
        if let Some(seconds) = event.enable_after {
            disabled_handler_timer.enable_after(seconds);
        }
    }
}

fn handle_enable_button_event(
    mut event_reader: EventReader<EnableButtonEvent>,
    mut button_query: Query<(&mut ButtonDisabledHandler, &mut ButtonDisabledHandlerTimer)>,
) {
    for event in event_reader.read() {
        let Ok((mut disabled_handler, mut disabled_handler_timer)) =
            button_query.get_mut(event.entity)
        else {
            error!(
                "received handle_enable_button_event with invalid entity: {}",
                event.entity
            );
            continue;
        };

        disabled_handler.disabled = false;
        if let Some(seconds) = event.disable_after {
            disabled_handler_timer.disable_after(seconds);
        }
    }
}
