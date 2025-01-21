use std::f32::consts::TAU;

use bevy::{log, prelude::*};

use crate::schedules::CubeStartupSet;

use super::{
    axis::Axis, controller::ControllerPlugin, cube_state::CubeState, rotation::CubeRotationPlugin,
};

pub const DEFAULT_CUBE_SIZE: usize = 3;

const COLOR_LEFT: Color = Color::srgb(0.99, 0.49, 0.05); // orange
const COLOR_RIGHT: Color = Color::srgb(0.99, 0.0, 0.0); // red
const COLOR_TOP: Color = Color::srgb(0.99, 0.99, 0.99); // white
const COLOR_BOTTOM: Color = Color::srgb(0.99, 0.99, 0.0); // yellow
const COLOR_FRONT: Color = Color::srgb(0.027, 0.89, 0.215); // green
const COLOR_BACK: Color = Color::srgb(0.0, 0.0, 0.99); // blue

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ControllerPlugin)
            .add_plugins(CubeRotationPlugin)
            .add_systems(
                Startup,
                spawn_default_cube.in_set(CubeStartupSet::SpawnCube),
            );
    }
}

/// A 3D representation of a cube
#[derive(Component, Debug, Clone)]
pub struct Cube {
    cube_size: CubeSize,   // For example 3 for 3x3
    pub piece_spread: f32, // The size of the gap between the pieces
    block_size: f32,
    inner_material: Handle<StandardMaterial>,
    pub is_animating_rotation: bool,
}

impl Cube {
    pub fn size(&self) -> &CubeSize {
        &self.cube_size
    }
}

#[derive(Clone, Debug)]
pub struct CubeSize(pub i32);

impl CubeSize {
    pub fn lowest_piece_index(&self) -> i32 {
        if self.0 % 2 == 1 {
            -(self.0 as i32 - 1) / 2
        } else {
            -self.0 as i32 / 2
        }
    }

    pub fn highest_piece_index(&self) -> i32 {
        if self.0 % 2 == 1 {
            (self.0 as i32 - 1) / 2
        } else {
            self.0 as i32 / 2
        }
    }
}

/// TODO add orientation (matrix?) to easily check the current state (correct spot or not). The original_x etc. do not take the rotation in to account.
/// Each piece has 24 possible states (you can look at each from 6 sides, rotating each side 4 times around the y axis).
#[derive(Component, Clone, Debug)]
pub struct Piece {
    pub current_x: i32,
    pub current_y: i32,
    pub current_z: i32,
}

impl Piece {
    /// Get all piece indicies of a slice
    pub fn get_piece_indices(pieces: &Vec<&mut Piece>, axis: Axis, slice_index: i32) -> Vec<usize> {
        let mut result = vec![];

        for (i, piece) in pieces.iter().enumerate() {
            let current_piece_index = match axis {
                Axis::X => piece.current_x,
                Axis::Y => piece.current_y,
                Axis::Z => piece.current_z,
            };

            if current_piece_index != slice_index {
                continue;
            }

            result.push(i);
        }

        result
    }
}

#[derive(Component)]
pub struct PieceFace;

fn spawn_default_cube(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn(commands, meshes, materials, DEFAULT_CUBE_SIZE);
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_size: usize,
) {
    if cube_size <= 1 {
        log::error!("can not spawn cube with invalid size: {cube_size}");
        return;
    }

    let cube = Cube {
        cube_size: CubeSize(cube_size as i32),
        piece_spread: 0.05,
        block_size: 1.0,
        inner_material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
        is_animating_rotation: false,
    };

    let piece_face_mesh = meshes.add(Rectangle {
        half_size: Vec2::ONE * cube.block_size / 2.0,
    });

    let range = cube.size().lowest_piece_index()..=cube.size().highest_piece_index();

    let spread_factor = 1.0 + cube.piece_spread;
    let face_offset = cube.block_size / 2.0;

    let mut cube_entity = commands.spawn((
        cube.clone(),
        CubeState::new(cube_size as usize),
        Transform::default(),
        Visibility::Visible,
    ));

    for x in range.clone() {
        for y in range.clone() {
            for z in range.clone() {
                if cube.size().0 % 2 == 0 && (x == 0 || y == 0 || z == 0) {
                    continue;
                }

                // The middle point of the cube piece
                let mut middle_point = if cube.size().0 % 2 == 0 {
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

                cube_entity.with_children(|parent| {
                    let mut piece_entity = parent.spawn((
                        Piece {
                            current_x: x,
                            current_y: y,
                            current_z: z,
                        },
                        Transform::from_translation(middle_point),
                    ));

                    piece_entity.with_children(|parent| {
                        // left face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(face_offset, 0.0, 0.0));
                        transform.rotate_local_y(-TAU / 4.0);

                        let material = if x == cube.size().lowest_piece_index() {
                            materials.add(COLOR_LEFT)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));

                        // right face
                        let mut transform =
                            Transform::from_translation(Vec3::new(face_offset, 0.0, 0.0));

                        let material = if x == cube.size().highest_piece_index() {
                            materials.add(COLOR_RIGHT)
                        } else {
                            cube.inner_material.clone()
                        };

                        transform.rotate_local_y(TAU / 4.0);

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));

                        // top face
                        let mut transform =
                            Transform::from_translation(Vec3::new(0.0, face_offset, 0.0));
                        transform.rotate_x(-TAU / 4.0);

                        let material = if y == cube.size().highest_piece_index() {
                            materials.add(COLOR_TOP)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));

                        // bottom face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(0.0, face_offset, 0.0));
                        transform.rotate_x(TAU / 4.0);

                        let material = if y == cube.size().lowest_piece_index() {
                            materials.add(COLOR_BOTTOM)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));

                        // front face
                        let transform =
                            Transform::from_translation(Vec3::new(0.0, 0.0, face_offset));

                        let material = if z == cube.size().highest_piece_index() {
                            materials.add(COLOR_FRONT)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));

                        // back face
                        let mut transform =
                            Transform::from_translation(-Vec3::new(0.0, 0.0, face_offset));
                        transform.rotate_local_y(-TAU / 2.0);

                        let material = if z == cube.size().lowest_piece_index() {
                            materials.add(COLOR_BACK)
                        } else {
                            cube.inner_material.clone()
                        };

                        parent.spawn((
                            Mesh3d(piece_face_mesh.clone()),
                            transform,
                            MeshMaterial3d(material),
                            PieceFace,
                        ));
                    });
                });
            }
        }
    }
}
