use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    interface::interface::{
        CaptureClick, BUTTON_BACKGROUND_COLOR, BUTTON_BORDER, BUTTON_BORDER_RADIUS,
        BUTTON_TEXT_COLOR, COLOR_GREY, DEFAULT_FONT, DEFAULT_FONT_BOLD,
    },
    schedules::CubeScheduleSet,
};

use super::button::UiButton;

pub struct DropdownPlugin;

impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (dropdown_state_handler, handle_dropdown_options_button_click)
                .chain()
                .in_set(CubeScheduleSet::HandleUserInput),
        );
    }
}

#[derive(Component, Clone)]
pub struct DropdownOption<T> {
    pub label: String,
    pub value: T,
}

#[derive(Component)]
struct DropdownMainButton;
#[derive(Component)]
struct DropdownMainButtonLabel;
#[derive(Component)]
struct DropdownMainButtonContent;
#[derive(Component)]
struct DropdownOptionsContainer {
    close_on_button_click: bool,
    /// If true, set the text of the main button to be the label of the selected option on clicking an option
    set_option_label_on_button_click: bool,
}
#[derive(Component)]
pub struct DropdownOptionButton;
#[derive(Component)]
struct DropdownOptionButtonLabel;

/// Selected options will be displayed as the main button label.
pub fn spawn_type_select<T: Component + Clone>(
    options: Vec<DropdownOption<T>>,
    mut selected_option_index: usize,
    close_on_button_click: bool,
    main_button_marker: impl Bundle,
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
) {
    if selected_option_index >= options.len() {
        warn!(
            "spawn_type_select: selected_option_index {} is invalid. Defaulting to 0 (\"{}\")",
            selected_option_index,
            options[0].label.clone()
        );
        selected_option_index = 0;
    }

    // main button
    parent
        .spawn((
            main_button_marker,
            Button,
            CaptureClick,
            DropdownMainButton,
            Node {
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(16.0),
                    right: Val::Px(16.),
                    top: Val::Px(9.5),
                    bottom: Val::Px(9.5),
                },
                min_width: Val::Px(80.), // account for longest label, "instant"
                border: BUTTON_BORDER,
                ..default()
            },
            BorderColor(Color::BLACK),
            BUTTON_BORDER_RADIUS,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            // label
            let text = options[selected_option_index].label.clone();

            parent.spawn((
                DropdownMainButtonLabel,
                Text::new(text),
                TextFont {
                    font: asset_server.load(DEFAULT_FONT_BOLD),
                    font_size: 14.0,
                    ..default()
                },
                Node { ..default() },
                TextColor(BUTTON_TEXT_COLOR),
            ));

            // options container
            parent
                .spawn((
                    DropdownOptionsContainer {
                        close_on_button_click: close_on_button_click,
                        set_option_label_on_button_click: true,
                    },
                    CaptureClick,
                    Node {
                        width: Val::Percent(100.),
                        position_type: PositionType::Absolute,
                        top: Val::Percent(93.),
                        border: BUTTON_BORDER,
                        padding: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(4.0),
                            bottom: Val::ZERO,
                        },
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(COLOR_GREY),
                    BorderColor(Color::BLACK),
                    BUTTON_BORDER_RADIUS,
                ))
                .with_children(|parent| {
                    // option buttons
                    for dropdown_option in options {
                        parent
                            .spawn((
                                dropdown_option.value,
                                DropdownOptionButton,
                                UiButton,
                                FocusPolicy::Pass,
                                Node {
                                    margin: UiRect::bottom(Val::Px(4.0)),
                                    padding: UiRect {
                                        left: Val::Px(16.0),
                                        right: Val::Px(16.),
                                        top: Val::Px(8.),
                                        bottom: Val::Px(8.),
                                    },
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Percent(100.),
                                    border: BUTTON_BORDER,
                                    ..default()
                                },
                                BackgroundColor(BUTTON_BACKGROUND_COLOR),
                                BorderColor(Color::BLACK),
                                BUTTON_BORDER_RADIUS,
                            ))
                            .with_child((
                                DropdownOptionButtonLabel,
                                Text::new(dropdown_option.label.clone()),
                                TextFont {
                                    font: asset_server.load(DEFAULT_FONT),
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(BUTTON_TEXT_COLOR),
                            ));
                    }
                });
        });
}

/// Selecting options does not affect the main button.
pub fn spawn_type_menu<T: Component + Clone>(
    options: Vec<DropdownOption<T>>,
    close_on_button_click: bool,
    main_button_marker: impl Bundle,
    main_button_content: impl Bundle,
    parent: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
) {
    // main button
    parent
        .spawn((
            main_button_marker,
            Button,
            CaptureClick,
            DropdownMainButton,
            Node {
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Px(16.0),
                    right: Val::Px(16.),
                    top: Val::Px(9.5),
                    bottom: Val::Px(9.5),
                },
                min_width: Val::Px(80.), // account for longest label, "instant"
                border: BUTTON_BORDER,
                ..default()
            },
            BorderColor(Color::BLACK),
            BUTTON_BORDER_RADIUS,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            // label
            parent.spawn((main_button_content, DropdownMainButtonContent));

            // options container
            parent
                .spawn((
                    DropdownOptionsContainer {
                        close_on_button_click: close_on_button_click,
                        set_option_label_on_button_click: false,
                    },
                    CaptureClick,
                    Node {
                        width: Val::Percent(100.),
                        position_type: PositionType::Absolute,
                        top: Val::Percent(93.),
                        border: BUTTON_BORDER,
                        padding: UiRect {
                            left: Val::Px(2.0),
                            right: Val::Px(2.0),
                            top: Val::Px(4.0),
                            bottom: Val::ZERO,
                        },
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(COLOR_GREY),
                    BorderColor(Color::BLACK),
                    BUTTON_BORDER_RADIUS,
                ))
                .with_children(|parent| {
                    // option buttons
                    for dropdown_option in options {
                        parent
                            .spawn((
                                dropdown_option.value,
                                DropdownOptionButton,
                                UiButton,
                                FocusPolicy::Pass,
                                Node {
                                    margin: UiRect::bottom(Val::Px(4.0)),
                                    padding: UiRect {
                                        left: Val::Px(16.0),
                                        right: Val::Px(16.),
                                        top: Val::Px(8.),
                                        bottom: Val::Px(8.),
                                    },
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Percent(100.),
                                    border: BUTTON_BORDER,
                                    ..default()
                                },
                                BackgroundColor(BUTTON_BACKGROUND_COLOR),
                                BorderColor(Color::BLACK),
                                BUTTON_BORDER_RADIUS,
                            ))
                            .with_child((
                                DropdownOptionButtonLabel,
                                Text::new(dropdown_option.label.clone()),
                                TextFont {
                                    font: asset_server.load(DEFAULT_FONT),
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(BUTTON_TEXT_COLOR),
                            ));
                    }
                });
        });
}

/// TODO optimise. The check for if the options_container is a child of the main_dropdown_button is O(n^2) where n = the number of dropdowns.
/// It runs every tick, so it may cause performance issues when there are a lot of dropdowns.
///
/// handle open/closing of the options
fn dropdown_state_handler(
    main_dropdown_button_query: Query<(&Children, &Interaction), With<DropdownMainButton>>,
    mut options_container_query: Query<
        (Entity, &Interaction, &mut Visibility),
        With<DropdownOptionsContainer>,
    >,
) {
    for (main_button_children, main_button_interaction) in main_dropdown_button_query.iter() {
        for (
            options_container_entity,
            options_container_interaction,
            mut options_container_visibility,
        ) in options_container_query.iter_mut()
        {
            if !main_button_children.contains(&options_container_entity) {
                continue;
            }

            if *main_button_interaction == Interaction::Hovered
                || *options_container_interaction == Interaction::Hovered
                || *main_button_interaction == Interaction::Pressed
                || *options_container_interaction == Interaction::Pressed
            {
                *options_container_visibility = Visibility::Visible;
            } else {
                *options_container_visibility = Visibility::Hidden;
            }
        }
    }
}

fn handle_dropdown_options_button_click(
    option_buttons_query: Query<
        (&Parent, Entity, &Interaction),
        (With<DropdownOptionButton>, Changed<Interaction>),
    >,
    option_button_labels_query: Query<
        (&Parent, &Text),
        (
            With<DropdownOptionButtonLabel>,
            Without<DropdownMainButtonLabel>,
        ),
    >,
    mut option_button_containers_query: Query<(
        Entity,
        &Parent,
        &DropdownOptionsContainer,
        &mut Visibility,
    )>,
    mut main_dropdown_button_query: Query<
        (&mut Text, &Parent),
        (
            With<DropdownMainButtonLabel>,
            Without<DropdownOptionButtonLabel>,
        ),
    >,
) {
    for (option_button_parent, option_button_entity, option_button_interaction) in
        option_buttons_query.iter()
    {
        if *option_button_interaction != Interaction::Pressed {
            continue;
        }

        let mut handled = false;

        for (
            option_button_container_entity,
            option_button_container_parent,
            dropdown_options_container,
            mut option_button_container_visibility,
        ) in option_button_containers_query.iter_mut()
        {
            if option_button_container_entity != option_button_parent.get() {
                continue;
            }

            handled = true;

            if dropdown_options_container.close_on_button_click {
                // hide options
                *option_button_container_visibility = Visibility::Hidden;
            }

            if dropdown_options_container.set_option_label_on_button_click {
                // update dropdown label
                for (option_button_label_parent, option_button_label_text) in
                    option_button_labels_query.iter()
                {
                    if option_button_label_parent.get() != option_button_entity {
                        continue;
                    }

                    for (mut main_dropdown_button_label, main_dropdown_button_parent) in
                        main_dropdown_button_query.iter_mut()
                    {
                        if main_dropdown_button_parent.get() != option_button_container_parent.get()
                        {
                            continue;
                        }

                        main_dropdown_button_label.0 = option_button_label_text.0.clone();
                        break;
                    }

                    break;
                }
            }

            break;
        }

        if !handled {
            warn!("dropdown option button was pressed but parent was not found");
        }
    }
}
