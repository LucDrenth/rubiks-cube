use bevy::prelude::*;

use crate::{
    cube::{self, Cube, CubeRotationAnimation, CubeRotationEvent},
    schedules::CubeScheduleSet,
};

use super::{
    interface::{
        CaptureClick, BUTTON_BACKGROUND_COLOR, BUTTON_BORDER, BUTTON_BORDER_RADIUS, COLOR_BLUE,
        COLOR_MAIN,
    },
    widget::{
        self,
        button::UiButton,
        dropdown::{Dropdown, DropdownOption},
    },
};

pub struct CubeRotationsPlugin;

impl Plugin for CubeRotationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_cube_rotation_dropdown).in_set(CubeScheduleSet::HandleEvents),
        );
    }
}

#[derive(Component)]
struct CubeRotationDropdownButton;

#[derive(Component, Clone, Debug)]
enum CubeRotation {
    X,
    XPrime,
    Y,
    YPrime,
    Z,
    ZPrime,
}

pub fn spawn(parent: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
    // let cube_rotation_image = asset_server.load("icons/cube-rotation.png");

    parent.spawn(Node { ..default() }).with_children(|parent| {
        widget::dropdown::spawn(
            Dropdown::new(
                vec![
                    DropdownOption {
                        label: "X".to_string(),
                        value: CubeRotation::X,
                    },
                    DropdownOption {
                        label: "X'".to_string(),
                        value: CubeRotation::XPrime,
                    },
                    DropdownOption {
                        label: "Y".to_string(),
                        value: CubeRotation::Y,
                    },
                    DropdownOption {
                        label: "Y'".to_string(),
                        value: CubeRotation::YPrime,
                    },
                    DropdownOption {
                        label: "Z".to_string(),
                        value: CubeRotation::Z,
                    },
                    DropdownOption {
                        label: "Z'".to_string(),
                        value: CubeRotation::ZPrime,
                    },
                ],
                widget::dropdown::DropdownType::Menu("rotate".to_string()),
            )
            .without_close_on_button_click(),
            CubeRotationDropdownButton,
            parent,
            asset_server,
        );

        // TODO change use this icon in dropdown
        // parent
        //     .spawn((
        //         CubeRotationDropdownButton,
        //         CaptureClick,
        //         UiButton,
        //         Node {
        //             justify_content: JustifyContent::Center,
        //             align_items: AlignItems::Center,
        //             padding: UiRect {
        //                 left: Val::Px(12.0),
        //                 right: Val::Px(12.),
        //                 top: Val::Px(4.),
        //                 bottom: Val::Px(4.),
        //             },
        //             border: BUTTON_BORDER,
        //             ..default()
        //         },
        //         BorderColor(COLOR_MAIN),
        //         BUTTON_BORDER_RADIUS,
        //         BackgroundColor(BUTTON_BACKGROUND_COLOR),
        //     ))
        //     .with_child((
        //         ImageNode {
        //             image: cube_rotation_image.clone(),
        //             color: COLOR_BLUE,
        //             ..default()
        //         },
        //         Node {
        //             width: Val::Px(24.0),
        //             height: Val::Px(24.0),
        //             ..default()
        //         },
        //     ));
    });
}

fn handle_cube_rotation_dropdown(
    event_query: Query<(&CubeRotation, &Interaction), Changed<Interaction>>,
    cube_query: Query<&Cube>,
    mut event_writer: EventWriter<CubeRotationEvent>,
) {
    let Ok((cube_rotation, interaction)) = event_query.get_single() else {
        return;
    };
    if *interaction != Interaction::Pressed {
        return;
    }

    let Ok(cube) = cube_query.get_single() else {
        error!("failed to get cube");
        return;
    };
    if cube.is_animating_rotation {
        return;
    }

    let (rotation, negative_direction) = match cube_rotation {
        CubeRotation::X => (cube::Rotation::cube_x(), true),
        CubeRotation::XPrime => (cube::Rotation::cube_x(), false),
        CubeRotation::Y => (cube::Rotation::cube_y(), true),
        CubeRotation::YPrime => (cube::Rotation::cube_y(), false),
        CubeRotation::Z => (cube::Rotation::cube_z(), false),
        CubeRotation::ZPrime => (cube::Rotation::cube_z(), true),
    };

    event_writer.send(CubeRotationEvent {
        rotation,
        negative_direction,
        twice: false,
        animation: Some(CubeRotationAnimation {
            duration_in_seconds: 0.4,
            ease_function: Some(EaseFunction::CubicOut),
        }),
    });
}
