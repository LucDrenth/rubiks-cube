use bevy::prelude::*;

use crate::{
    cube::{self},
    schedules::CubeScheduleSet,
};

use super::{
    cube_actions::{self, CubeActionsPlugin},
    cube_rotations::{self, CubeRotationsPlugin},
    cube_size::{self, CubeSizePlugin},
    gradient_shader::{
        BackgroundGradientMaterial, BackgroundGradientMaterialBuilder, ColorSize,
        GradientShaderPlugin, GradientType,
    },
    widget,
};

/// yellow
pub const COLOR_MAIN: Color = Color::srgb(0.952, 0.784, 0.007);
pub const COLOR_RED: Color = Color::srgb(99.0 / 255.0, 28.0 / 255.0, 35.0 / 255.0);
pub const COLOR_BLUE: Color = Color::srgb(10.0 / 255.0, 30.0 / 255.0, 74.0 / 255.0);
pub const COLOR_GREY: Color = Color::srgb(0.55, 0.55, 0.55);
pub const BUTTON_TEXT_COLOR: Color = COLOR_RED;
pub const BUTTON_BORDER: UiRect = UiRect::all(Val::Px(2.));
pub const BUTTON_BORDER_RADIUS: BorderRadius = BorderRadius::all(Val::Px(4.));
pub const BUTTON_BACKGROUND_COLOR: Color = Color::srgb(240.0 / 255.0, 235.0 / 255.0, 220.0 / 255.0);

/// roboto
pub const DEFAULT_FONT: &str = "fonts/roboto.ttf";
/// roboto
pub const DEFAULT_FONT_BOLD: &str = "fonts/roboto-bold.ttf";

#[derive(Resource)]
pub struct UiResource {
    pub did_handle_click: bool,
}

/// Add this component to a ui element to not let a click event bubble up to the world
#[derive(Component)]
#[require(Button)]
pub struct CaptureClick;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiResource {
            did_handle_click: false,
        })
        .add_plugins(GradientShaderPlugin)
        .add_plugins(widget::WidgetPlugin)
        .add_plugins(CubeSizePlugin)
        .add_plugins(CubeActionsPlugin)
        .add_plugins(CubeRotationsPlugin)
        .add_systems(Startup, init)
        .add_systems(
            Update,
            update_ui_resource.in_set(CubeScheduleSet::HandleUserInput),
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

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ui_materials: ResMut<Assets<BackgroundGradientMaterial>>,
) {
    let colored_border_alpha = 0.7;

    // container element
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            MaterialNode(
                ui_materials.add(
                    BackgroundGradientMaterialBuilder::default()
                        .with_colors(vec![COLOR_BLUE, COLOR_RED, COLOR_BLUE])
                        .unwrap(),
                ),
            ),
            CaptureClick,
        ))
        .with_children(|parent| {
            // colored border bottom
            parent.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(2.),
                    ..default()
                },
                MaterialNode(
                    ui_materials.add(
                        BackgroundGradientMaterialBuilder::default()
                            .with_colors(vec![
                                cube::COLOR_LEFT.with_alpha(colored_border_alpha),
                                cube::COLOR_TOP.with_alpha(colored_border_alpha),
                                cube::COLOR_BACK.with_alpha(colored_border_alpha),
                                cube::COLOR_RIGHT.with_alpha(colored_border_alpha),
                                cube::COLOR_FRONT.with_alpha(colored_border_alpha),
                                cube::COLOR_BOTTOM.with_alpha(colored_border_alpha),
                            ])
                            .unwrap()
                            .with_gradient_type(GradientType::Block)
                            .with_color_size(ColorSize::Repeat(10.0)),
                    ),
                ),
            ));

            // actual UI
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::px(12.0, 12.0, 8.0, 8.0),
                    column_gap: Val::Px(8.),
                    ..default()
                })
                .with_children(|parent| {
                    cube_rotations::spawn(parent, &asset_server);
                    cube_actions::spawn(parent, &asset_server);
                    cube_size::spawn(parent, &asset_server);
                });
        });
}
