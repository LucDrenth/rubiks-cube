use std::f32::consts::TAU;

use bevy::prelude::*;

use super::{controller::ControllerPlugin, rotation::CubeRotationPlugin};

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControllerPlugin)
            .add_plugins(CubeRotationPlugin)
            .add_systems(Startup, spawn_cube);
    }
}

#[derive(Component, Debug)]
pub struct Cube {
    size: i32,             // For example 3 for 3x3
    pub piece_spread: f32, // The size of the gap between the pieces
    block_size: f32,
    inner_material: Handle<StandardMaterial>,
}

impl Cube {
    pub fn size(&self) -> i32 {
        self.size
    }

    fn lowest_piece_index(&self) -> i32 {
        // TODO this only holds for 3x3
        -1
    }

    fn highest_piece_index(&self) -> i32 {
        // TODO this only holds for 3x3
        1
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
    let cube = Cube {
        size: 3,
        piece_spread: 0.05,
        block_size: 1.0,
        inner_material: materials.add(Color::rgb(0.1, 0.1, 0.1)),
    };

    let piece_face_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * cube.block_size / 2.0,
    });

    if cube.size < 2 {
        panic!("Invalid cube size {}", cube.size)
    }

    let mut offset = 0.0;

    let range = if cube.size % 2 == 1 {
        -(cube.size as i32 - 1) / 2..=(cube.size as i32 - 1) / 2
    } else {
        offset = cube.block_size / 2.0;
        -(cube.size as i32 / 2) + 1..=cube.size as i32 / 2
    };

    let spread_factor = 1.0 + cube.piece_spread;
    let face_offset = cube.block_size / 2.0;

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                // The middle point of the cube piece
                let middle_point = Vec3::new(
                    x as f32 * cube.block_size - offset,
                    y as f32 * cube.block_size - offset,
                    z as f32 * cube.block_size - offset,
                ) * spread_factor;

                // left
                let mut transform =
                    Transform::from_translation(middle_point - Vec3::new(face_offset, 0.0, 0.0));
                transform.rotate_local_y(-TAU / 4.0);

                let material = if x == cube.lowest_piece_index() {
                    materials.add(Color::rgb(0.99, 0.49, 0.05)) // orange
                } else {
                    cube.inner_material.clone()
                };

                let face_left = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: material,
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // right
                let mut transform =
                    Transform::from_translation(middle_point + Vec3::new(face_offset, 0.0, 0.0));

                // TODO this will only hold for 3x3
                let material = if x == cube.highest_piece_index() {
                    materials.add(Color::rgb(0.99, 0.0, 0.0)) // red
                } else {
                    cube.inner_material.clone()
                };

                transform.rotate_local_y(TAU / 4.0);
                let face_right = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: material,
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
                let material = if y == cube.highest_piece_index() {
                    materials.add(Color::rgb(0.99, 0.99, 0.99)) // white
                } else {
                    cube.inner_material.clone()
                };

                let face_top = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: material,
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
                let material = if y == cube.lowest_piece_index() {
                    materials.add(Color::rgb(0.99, 0.99, 0.0)) // yellow
                } else {
                    cube.inner_material.clone()
                };

                let face_bottom = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: material,
                            ..default()
                        },
                        PieceFace,
                    ))
                    .id();

                // front
                // TODO this will only hold for 3x3
                let material = if z == cube.highest_piece_index() {
                    materials.add(Color::rgb(7.0 / 255.0, 227.0 / 255.0, 55.0 / 255.0))
                // green
                } else {
                    cube.inner_material.clone()
                };

                let face_front = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: Transform::from_translation(
                                middle_point + Vec3::new(0.0, 0.0, face_offset),
                            ),
                            material: material,
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
                let material = if z == cube.lowest_piece_index() {
                    materials.add(Color::rgb(0.0, 0.0, 0.99)) // blue
                } else {
                    cube.inner_material.clone()
                };

                let face_back = commands
                    .spawn((
                        PbrBundle {
                            mesh: piece_face_mesh.clone(),
                            transform: transform,
                            material: material,
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
