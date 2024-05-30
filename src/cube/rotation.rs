use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

use super::cube::{Cube, PieceFace, BLOCKS_SPREAD, CUBE_SIZE};

pub struct CubeRotationPlugin;

impl Plugin for CubeRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CubeRotationEvent>().add_systems(Update, rotation_events_handler.in_set(CubeScheduleSet::HandleEvents))
        // .add_systems(Update, smoothly_rotate_whole_cube.in_set(CubeScheduleSet::UpdateAnimations))
        ;
    }
}

#[derive(Event, Clone, Debug)]
pub struct CubeRotationEvent {
    pub rotation: Rotation,
    pub negative_direction: bool,
    pub twice: bool,
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

/// TODO this does not work together with the rotation events
fn smoothly_rotate_whole_cube(
    mut query: Query<&mut Transform, Or<(With<PieceFace>, With<Cube>)>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() / 1.5),
        );
    }
}

fn rotation_events_handler(
    mut cube_query: Query<&mut Cube>,
    cube_transform_query: Query<&Transform, (With<Cube>, Without<PieceFace>)>,
    mut faces_query: Query<&mut Transform, With<PieceFace>>,
    mut event_reader: EventReader<CubeRotationEvent>,
) {
    let Ok(mut cube) = cube_query.get_single_mut() else {
        error!("expected exactly 1 Cube entity");
        return;
    };
    let Ok(cube_transform) = cube_transform_query.get_single() else {
        error!("expected exactly 1 Cube Transform entity");
        return;
    };

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
                    return *slice as f32 * (CUBE_SIZE as f32 + BLOCKS_SPREAD);
                };

                match face_rotation {
                    FaceRotation::X(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(pivot_coordinate(slice), 0.0, 0.0);
                            let cubes_indices_to_rotate =
                                cube.get_piece_indices_with_coords(Some(*slice), None, None);

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube.pieces[*cube_index_to_rotate].faces {
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
                                        cube.pieces[*cube_index_to_rotate].x,
                                        cube.pieces[*cube_index_to_rotate].y * -1,
                                        cube.pieces[*cube_index_to_rotate].z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].x,
                                        cube.pieces[*cube_index_to_rotate].z,
                                        cube.pieces[*cube_index_to_rotate].y * -1,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].x,
                                        cube.pieces[*cube_index_to_rotate].z * -1,
                                        cube.pieces[*cube_index_to_rotate].y,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube.pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube.pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube.pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Y(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(0.0, pivot_coordinate(slice), 0.0);
                            let cubes_indices_to_rotate =
                                cube.get_piece_indices_with_coords(None, Some(*slice), None);

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube.pieces[*cube_index_to_rotate].faces {
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
                                        cube.pieces[*cube_index_to_rotate].x * -1,
                                        cube.pieces[*cube_index_to_rotate].y,
                                        cube.pieces[*cube_index_to_rotate].z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].z * -1,
                                        cube.pieces[*cube_index_to_rotate].y,
                                        cube.pieces[*cube_index_to_rotate].x,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].z,
                                        cube.pieces[*cube_index_to_rotate].y,
                                        cube.pieces[*cube_index_to_rotate].x * -1,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube.pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube.pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube.pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Z(slices) => {
                        for slice in slices {
                            let pivot_point = Vec3::new(0.0, 0.0, pivot_coordinate(slice));
                            let cubes_indices_to_rotate =
                                cube.get_piece_indices_with_coords(None, None, Some(*slice));

                            // Rotate pieces
                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                for face in cube.pieces[*cube_index_to_rotate].faces {
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
                                        cube.pieces[*cube_index_to_rotate].x * -1,
                                        cube.pieces[*cube_index_to_rotate].y * -1,
                                        cube.pieces[*cube_index_to_rotate].z,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].y,
                                        cube.pieces[*cube_index_to_rotate].x * -1,
                                        cube.pieces[*cube_index_to_rotate].z,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube.pieces[*cube_index_to_rotate].y * -1,
                                        cube.pieces[*cube_index_to_rotate].x,
                                        cube.pieces[*cube_index_to_rotate].z,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube.pieces[*cube_index].x = new_pieces_coordinates[i].0;
                                cube.pieces[*cube_index].y = new_pieces_coordinates[i].1;
                                cube.pieces[*cube_index].z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                }
            }
            Rotation::Cube(cube_rotation) => match cube_rotation {
                CubeRotation::X => {
                    for piece in &mut cube.pieces {
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
                    for piece in &mut cube.pieces {
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
                    for piece in &mut cube.pieces {
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
