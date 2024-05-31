use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::schedules::CubeScheduleSet;

use super::cube::{Cube, Piece, PieceFace};

pub struct CubeRotationPlugin;

impl Plugin for CubeRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CubeRotationEvent>().add_systems(
            Update,
            rotation_events_handler.in_set(CubeScheduleSet::HandleEvents),
        );
    }
}

// TODO add Option<Animation> property
#[derive(Event, Clone, Debug)]
pub struct CubeRotationEvent {
    pub rotation: Rotation,
    pub negative_direction: bool,
    pub twice: bool,
}

impl CubeRotationEvent {
    pub fn random_face_rotation(cube: &Cube) -> Self {
        let face_rotation = FaceRotation::random(cube);

        let mut rng = rand::thread_rng();
        let direction = if rng.gen_range(0..=1) == 0 {
            true
        } else {
            false
        };

        CubeRotationEvent {
            rotation: Rotation::Face(face_rotation),
            negative_direction: direction,
            twice: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Rotation {
    Face(FaceRotation),
    Cube(CubeRotation),
}

/// Rotate the given faces (e.g. slices) of the cube on a given axis. This is relative to the current cube rotation.
#[derive(Clone, Debug)]
pub enum FaceRotation {
    /// Rotate the given slices of the x axis.
    /// For a rotation in the default direction, when looking at the front of the cube, the front row ends up at the bottom.
    X(Vec<i32>),
    /// Rotate the given slices of the y axis.
    /// For a rotation in the default direction, when looking at the front of the cube, the front row ends up at the right side.
    Y(Vec<i32>),
    /// Rotate the given slices of the z axis
    /// For a rotation in the default direction, when looking at the front of the cube, the top row ends up at the left side.
    Z(Vec<i32>),
}

impl FaceRotation {
    pub fn random(cube: &Cube) -> Self {
        let mut rng = rand::thread_rng();

        let slice = rng.gen_range(cube.lowest_piece_index()..=cube.highest_piece_index());
        let axis = rng.gen_range(0..=2);

        if axis == 0 {
            Self::X(vec![slice])
        } else if axis == 1 {
            Self::Y(vec![slice])
        } else {
            Self::Z(vec![slice])
        }
    }
}

/// Rotate the whole cube on a given axis. This also changes which faces gets rotated for FaceRotation events.
#[derive(Clone, Debug)]
pub enum CubeRotation {
    /// Move the whole cube on the x axis.
    /// For the default direction, when looking at the front of the cube, the front face ends up at the bottom.
    X,
    /// Move the whole cube on the y axis
    /// For the default direction, when looking at the front of the cube, the front face ends up at the left side.
    Y,
    /// Move the whole cube on the z axis.
    /// For the default direction, when looking at the front of the cube, the top row ends up at the left side.
    Z,
}

fn rotation_events_handler(
    cube_query: Query<&Cube>,
    mut cube_pieces_query: Query<&mut Piece>,
    cube_transform_query: Query<&Transform, (With<Cube>, Without<PieceFace>)>,
    mut faces_query: Query<&mut Transform, With<PieceFace>>,
    mut event_reader: EventReader<CubeRotationEvent>,
) {
    let Ok(cube) = cube_query.get_single() else {
        error!("expected exactly 1 Cube entity");
        return;
    };
    let Ok(cube_transform) = cube_transform_query.get_single() else {
        error!("expected exactly 1 Cube Transform entity");
        return;
    };

    let mut cube_pieces: Vec<Mut<Piece>> = cube_pieces_query.iter_mut().collect();

    let mut rotation_amount = TAU / 4.0;

    let mut rotate_face =
        |face: Entity, pivot_point: Vec3, rotation: Quat| match faces_query.get_mut(face) {
            Ok(mut transform) => {
                transform.rotate_around(pivot_point, rotation);
            }
            Err(err) => {
                error!("failed to get cube face: {}", err);
            }
        };

    for cube_rotation_event in event_reader.read() {
        if cube_rotation_event.twice {
            rotation_amount *= 2.;
        }

        if cube_rotation_event.negative_direction {
            rotation_amount *= -1.;
        }

        match &cube_rotation_event.rotation {
            Rotation::Face(face_rotation) => {
                let pivot_coordinate = |slice: &i32| {
                    return *slice as f32 * (cube.size() as f32 + cube.piece_spread);
                };

                match face_rotation {
                    FaceRotation::X(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(pivot_coordinate(slice), 0.0, 0.0);
                            let cubes_indices_to_rotate = Piece::get_piece_indices_with_coords(
                                &cube_pieces,
                                Some(*slice),
                                None,
                                None,
                            );

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube_pieces[*cube_index_to_rotate].faces {
                                    rotate_face(
                                        face,
                                        pivot_point,
                                        Quat::from_rotation_x(rotation_amount),
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].x,
                                        cube_pieces[*cube_index_to_rotate].y * -1,
                                        cube_pieces[*cube_index_to_rotate].z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].x,
                                        cube_pieces[*cube_index_to_rotate].z,
                                        cube_pieces[*cube_index_to_rotate].y * -1,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].x,
                                        cube_pieces[*cube_index_to_rotate].z * -1,
                                        cube_pieces[*cube_index_to_rotate].y,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Y(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(0.0, pivot_coordinate(slice), 0.0);
                            let cubes_indices_to_rotate = Piece::get_piece_indices_with_coords(
                                &cube_pieces,
                                None,
                                Some(*slice),
                                None,
                            );

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube_pieces[*cube_index_to_rotate].faces {
                                    rotate_face(
                                        face,
                                        pivot_point,
                                        Quat::from_rotation_y(rotation_amount),
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].x * -1,
                                        cube_pieces[*cube_index_to_rotate].y,
                                        cube_pieces[*cube_index_to_rotate].z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].z * -1,
                                        cube_pieces[*cube_index_to_rotate].y,
                                        cube_pieces[*cube_index_to_rotate].x,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].z,
                                        cube_pieces[*cube_index_to_rotate].y,
                                        cube_pieces[*cube_index_to_rotate].x * -1,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Z(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(0.0, 0.0, pivot_coordinate(slice));
                            let cubes_indices_to_rotate = Piece::get_piece_indices_with_coords(
                                &cube_pieces,
                                None,
                                None,
                                Some(*slice),
                            );

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube_pieces[*cube_index_to_rotate].faces {
                                    rotate_face(
                                        face,
                                        pivot_point,
                                        Quat::from_rotation_z(rotation_amount),
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].x * -1,
                                        cube_pieces[*cube_index_to_rotate].y * -1,
                                        cube_pieces[*cube_index_to_rotate].z,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].y,
                                        cube_pieces[*cube_index_to_rotate].x * -1,
                                        cube_pieces[*cube_index_to_rotate].z,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].y * -1,
                                        cube_pieces[*cube_index_to_rotate].x,
                                        cube_pieces[*cube_index_to_rotate].z,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                }
            }
            Rotation::Cube(cube_rotation) => match cube_rotation {
                CubeRotation::X => {
                    for piece in &mut cube_pieces {
                        // Rotate faces
                        for face in piece.faces {
                            rotate_face(
                                face,
                                cube_transform.translation,
                                Quat::from_rotation_x(rotation_amount),
                            );
                        }

                        // Update piece indices
                        let new_y: i32;
                        let new_z: i32;

                        if cube_rotation_event.twice {
                            new_y = piece.y * -1;
                            new_z = piece.z * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_y = piece.y * -1;
                            new_z = piece.z;
                        } else {
                            new_y = piece.z;
                            new_z = piece.y * -1;
                        }

                        piece.y = new_y;
                        piece.z = new_z;
                    }
                }
                CubeRotation::Y => {
                    for piece in &mut cube_pieces {
                        // Rotate faces
                        for face in piece.faces {
                            rotate_face(
                                face,
                                cube_transform.translation,
                                Quat::from_rotation_y(rotation_amount),
                            );
                        }

                        // Update piece indices
                        let new_x: i32;
                        let new_z: i32;

                        if cube_rotation_event.twice {
                            new_x = piece.x * -1;
                            new_z = piece.z * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_x = piece.z * -1;
                            new_z = piece.x;
                        } else {
                            new_x = piece.z;
                            new_z = piece.x * -1;
                        }

                        piece.x = new_x;
                        piece.z = new_z;
                    }
                }
                CubeRotation::Z => {
                    for piece in &mut cube_pieces {
                        // Rotate faces
                        for face in piece.faces {
                            rotate_face(
                                face,
                                cube_transform.translation,
                                Quat::from_rotation_z(rotation_amount),
                            );
                        }

                        // Update piece indices
                        let new_x: i32;
                        let new_y: i32;

                        if cube_rotation_event.twice {
                            new_x = piece.x * -1;
                            new_y = piece.y * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_x = piece.y;
                            new_y = piece.x * -1;
                        } else {
                            new_x = piece.y * -1;
                            new_y = piece.x;
                        }

                        piece.x = new_x;
                        piece.y = new_y;
                    }
                }
            },
        }
    }
}
