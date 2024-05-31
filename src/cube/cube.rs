use std::f32::consts::TAU;

use bevy::prelude::*;

use super::{controller::ControllerPlugin, rotation::CubeRotationPlugin};

pub const BLOCKS_SPREAD: f32 = 0.05;
pub const BLOCKS_SIZE: f32 = 1.0;
pub const CUBE_SIZE: u32 = 3; // 3 for 3x3, 6 for 6x6 etc

const COLOR_INSIDE_R: f32 = 0.1;
const COLOR_INSIDE_G: f32 = 0.1;
const COLOR_INSIDE_B: f32 = 0.1;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControllerPlugin)
            .add_plugins(CubeRotationPlugin)
            .add_systems(Startup, spawn_cube);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Piece {
    pub faces: [Entity; 6],
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Piece {
    pub fn get_piece_indices_with_coords(
        pieces: &Vec<Mut<Piece>>,
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
    ) -> Vec<usize> {
        let mut result = vec![];

        for (i, piece) in pieces.iter().enumerate() {
            if let Some(x) = x {
                if piece.x != x {
                    continue;
                }
            }

            if let Some(y) = y {
                if piece.y != y {
                    continue;
                }
            }

            if let Some(z) = z {
                if piece.z != z {
                    continue;
                }
            }

            result.push(i);
        }

        result
    }
}

#[derive(Component, Debug)]
pub struct Cube {}

#[derive(Component)]
pub struct PieceFace;

pub enum Face {
    Left = 0,
    Right = 2,
    Top = 3,
    Bottom = 4,
    Front = 5,
    Back = 6,
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = Cube {};

    let piece_face_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * BLOCKS_SIZE / 2.0,
    });

    if CUBE_SIZE < 2 {
        panic!("Invalid cube size {}", CUBE_SIZE)
    }

    let mut offset = 0.0;

    let range = if CUBE_SIZE % 2 == 1 {
        -(CUBE_SIZE as i32 - 1) / 2..=(CUBE_SIZE as i32 - 1) / 2
    } else {
        offset = BLOCKS_SIZE / 2.0;
        -(CUBE_SIZE as i32 / 2) + 1..=CUBE_SIZE as i32 / 2
    };

    let spread_factor = 1.0 + BLOCKS_SPREAD;
    let face_offset = BLOCKS_SIZE / 2.0;

    let color_inside = Color::rgb(COLOR_INSIDE_R, COLOR_INSIDE_G, COLOR_INSIDE_B);

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                // The middle point of the cube piece
                let middle_point = Vec3::new(
                    x as f32 * BLOCKS_SIZE - offset,
                    y as f32 * BLOCKS_SIZE - offset,
                    z as f32 * BLOCKS_SIZE - offset,
                ) * spread_factor;

                // left
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(face_offset, 0.0, 0.0));
                transform.rotate_local_y(-TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if x == -1 {
                    Color::rgb(0.99, 0.49, 0.05) // orange
                } else {
                    color_inside
                };

                let face_left = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // right
                let mut transform =
                    Transform::from_translation(middle_point + Vec3::new(face_offset, 0.0, 0.0));

                // TODO this will only hold for 3x3
                let color = if x == 1 {
                    Color::rgb(0.99, 0.0, 0.0) // red
                } else {
                    color_inside
                };

                transform.rotate_local_y(TAU / 4.0);
                let face_right = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // top
                let mut transform =
                    Transform::from_translation(middle_point + Vec3::new(0.0, face_offset, 0.0));
                transform.rotate_x(-TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if y == 1 {
                    Color::rgb(0.99, 0.99, 0.99) // white
                } else {
                    color_inside
                };

                let face_top = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // bottom
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(0.0, face_offset, 0.0));
                transform.rotate_x(TAU / 4.0);

                // TODO this will only hold for 3x3
                let color = if y == -1 {
                    Color::rgb(0.99, 0.99, 0.0) // yellow
                } else {
                    color_inside
                };

                let face_bottom = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // front
                // TODO this will only hold for 3x3
                let color = if z == 1 {
                    Color::rgb(7.0 / 255.0, 227.0 / 255.0, 55.0 / 255.0) // green
                } else {
                    color_inside
                };

                let face_front = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: Transform::from_translation(
                                middle_point + Vec3::new(0.0, 0.0, face_offset),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // back
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(0.0, 0.0, face_offset));
                transform.rotate_local_y(-TAU / 2.0);

                // TODO this will only hold for 3x3
                let color = if z == -1 {
                    Color::rgb(0.0, 0.0, 0.99) // blue
                } else {
                    color_inside
                };

                let face_back = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                commands.spawn(Piece {
                    faces: [
                        face_left,
                        face_right,
                        face_top,
                        face_bottom,
                        face_front,
                        face_back,
                    ],
                    x,
                    y,
                    z,
                });
            }
        }
    }

    commands.spawn((cube, Transform::default()));
}
