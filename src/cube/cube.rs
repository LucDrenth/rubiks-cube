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
    pub is_animating_rotation: bool,
}

impl Cube {
    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn lowest_piece_index(&self) -> i32 {
        if self.size % 2 == 1 {
            -(self.size as i32 - 1) / 2
        } else {
            -self.size as i32 / 2
        }
    }

    pub fn highest_piece_index(&self) -> i32 {
        if self.size % 2 == 1 {
            (self.size as i32 - 1) / 2
        } else {
            self.size as i32 / 2
        }
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
        is_animating_rotation: false,
    };

    let piece_face_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * cube.block_size / 2.0,
    });

    if cube.size < 2 {
        panic!("Invalid cube size {}", cube.size)
    }

    let range = cube.lowest_piece_index()..=cube.highest_piece_index();

    let spread_factor = 1.0 + cube.piece_spread;
    let face_offset = cube.block_size / 2.0;

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                if cube.size() % 2 == 0 && (x == 0 || y == 0 || z == 0) {
                    continue;
                }

                // The middle point of the cube piece
                let mut middle_point = if cube.size() % 2 == 0 {
                    let mut result = Vec3::new(
                        x as f32 * cube.block_size,
                        y as f32 * cube.block_size,
                        z as f32 * cube.block_size,
                    );

                    if x < 0 {
                        result.x += cube.block_size / 2.0;
                    } else {
                        result.x -= cube.block_size / 2.0;
                    }

                    if y < 0 {
                        result.y += cube.block_size / 2.0;
                    } else {
                        result.y -= cube.block_size / 2.0;
                    }

                    if z < 0 {
                        result.z += cube.block_size / 2.0;
                    } else {
                        result.z -= cube.block_size / 2.0;
                    }

                    result
                } else {
                    Vec3::new(
                        x as f32 * cube.block_size,
                        y as f32 * cube.block_size,
                        z as f32 * cube.block_size,
                    )
                };
                middle_point *= spread_factor;

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
                let material = if z == cube.highest_piece_index() {
                    materials.add(Color::rgb(0.027, 0.89, 0.215)) // green
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
