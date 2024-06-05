use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::schedules::CubeScheduleSet;

use super::{
    axis::Axis,
    cube::{Cube, Piece, PieceFace},
};

pub struct CubeRotationPlugin;

impl Plugin for CubeRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CubeRotationEvent>()
            .add_systems(
                Update,
                rotation_events_handler.in_set(CubeScheduleSet::HandleEvents),
            )
            .add_systems(
                Update,
                handle_rotation_animations.in_set(CubeScheduleSet::UpdateAnimations),
            );
    }
}

#[derive(Clone, Debug)]
pub struct RotationAnimation {
    pub duration_in_seconds: f32,
}

#[derive(Component)]
struct RotationAnimator {
    progress: f32,
    duration_in_seconds: f32,
    amount_to_rotate: f32, // in radians
    axis: Axis,
    pivot_point: Vec3,
}

impl RotationAnimator {
    fn new(
        animation: &RotationAnimation,
        amount_to_rotate: f32,
        axis: Axis,
        pivot_point: Vec3,
    ) -> Self {
        Self {
            progress: 0.0,
            duration_in_seconds: animation.duration_in_seconds,
            amount_to_rotate,
            axis,
            pivot_point,
        }
    }
}

#[derive(Event, Clone, Debug)]
pub struct CubeRotationEvent {
    pub rotation: Rotation,
    pub negative_direction: bool,
    pub twice: bool,
    pub animation: Option<RotationAnimation>,
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
            animation: None,
        }
    }

    /// check wether the given event negates (undoes) self
    pub fn negates(&self, event: &Self) -> bool {
        if self.rotation != event.rotation {
            return false;
        }

        if self.twice != event.twice {
            return false;
        }

        if self.negative_direction != event.negative_direction {
            return true;
        }

        return false;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rotation {
    Face(FaceRotation),
    Cube(CubeRotation),
}

/// Rotate the given faces (e.g. slices) of the cube on a given axis. This is relative to the current cube rotation.
#[derive(Clone, Debug, PartialEq)]
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

        let slice = if cube.size() % 2 == 1 {
            rng.gen_range(cube.lowest_piece_index()..=cube.highest_piece_index())
        } else {
            let mut result = if cube.size() == 2 {
                1
            } else {
                rng.gen_range(1..(cube.size() / 2))
            };
            let negative = rng.gen_range(0..=1);

            if negative == 0 {
                result *= -1
            }

            result
        };
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
#[derive(Clone, Debug, PartialEq)]
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
    mut commands: Commands,
    mut cube_query: Query<&mut Cube>,
    mut cube_pieces_query: Query<&mut Piece>,
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

    let mut cube_pieces: Vec<Mut<Piece>> = cube_pieces_query.iter_mut().collect();

    for cube_rotation_event in event_reader.read() {
        if cube.is_animating_rotation {
            continue;
        }

        let mut rotation_amount = TAU / 4.0;

        if cube_rotation_event.twice {
            rotation_amount *= 2.;
        }

        if cube_rotation_event.negative_direction {
            rotation_amount *= -1.;
        }

        match &cube_rotation_event.rotation {
            Rotation::Face(face_rotation) => {
                let cube_size = cube.size() as f32;
                let cube_piece_spread = cube.piece_spread;
                let pivot_coordinate = |slice: &i32| {
                    return *slice as f32 * (cube_size + cube_piece_spread);
                };

                match face_rotation {
                    FaceRotation::X(slices) => {
                        for slice in slices {
                            if cube.size() % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

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
                                        &mut commands,
                                        &mut faces_query,
                                        &mut cube,
                                        face,
                                        pivot_point,
                                        &cube_rotation_event.animation,
                                        Axis::X,
                                        rotation_amount,
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_x,
                                        cube_pieces[*cube_index_to_rotate].current_y * -1,
                                        cube_pieces[*cube_index_to_rotate].current_z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_x,
                                        cube_pieces[*cube_index_to_rotate].current_z,
                                        cube_pieces[*cube_index_to_rotate].current_y * -1,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_x,
                                        cube_pieces[*cube_index_to_rotate].current_z * -1,
                                        cube_pieces[*cube_index_to_rotate].current_y,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].current_z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Y(slices) => {
                        for slice in slices {
                            if cube.size() % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

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
                                        &mut commands,
                                        &mut faces_query,
                                        &mut cube,
                                        face,
                                        pivot_point,
                                        &cube_rotation_event.animation,
                                        Axis::Y,
                                        rotation_amount,
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_x * -1,
                                        cube_pieces[*cube_index_to_rotate].current_y,
                                        cube_pieces[*cube_index_to_rotate].current_z * -1,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_z * -1,
                                        cube_pieces[*cube_index_to_rotate].current_y,
                                        cube_pieces[*cube_index_to_rotate].current_x,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_z,
                                        cube_pieces[*cube_index_to_rotate].current_y,
                                        cube_pieces[*cube_index_to_rotate].current_x * -1,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].current_z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Z(slices) => {
                        for slice in slices {
                            if cube.size() % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

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
                                        &mut commands,
                                        &mut faces_query,
                                        &mut cube,
                                        face,
                                        pivot_point,
                                        &cube_rotation_event.animation,
                                        Axis::Z,
                                        rotation_amount,
                                    );
                                }
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(cubes_indices_to_rotate.len());

                            for cube_index_to_rotate in &cubes_indices_to_rotate {
                                if cube_rotation_event.twice {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_x * -1,
                                        cube_pieces[*cube_index_to_rotate].current_y * -1,
                                        cube_pieces[*cube_index_to_rotate].current_z,
                                    ));
                                } else if cube_rotation_event.negative_direction {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_y,
                                        cube_pieces[*cube_index_to_rotate].current_x * -1,
                                        cube_pieces[*cube_index_to_rotate].current_z,
                                    ));
                                } else {
                                    new_pieces_coordinates.push((
                                        cube_pieces[*cube_index_to_rotate].current_y * -1,
                                        cube_pieces[*cube_index_to_rotate].current_x,
                                        cube_pieces[*cube_index_to_rotate].current_z,
                                    ));
                                }
                            }

                            for (i, cube_index) in cubes_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*cube_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*cube_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*cube_index].current_z = new_pieces_coordinates[i].2;
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
                                &mut commands,
                                &mut faces_query,
                                &mut cube,
                                face,
                                cube_transform.translation,
                                &cube_rotation_event.animation,
                                Axis::X,
                                rotation_amount,
                            );
                        }

                        // Update piece indices
                        let new_y: i32;
                        let new_z: i32;

                        if cube_rotation_event.twice {
                            new_y = piece.current_y * -1;
                            new_z = piece.current_z * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_y = piece.current_y * -1;
                            new_z = piece.current_z;
                        } else {
                            new_y = piece.current_z;
                            new_z = piece.current_y * -1;
                        }

                        piece.current_y = new_y;
                        piece.current_z = new_z;
                    }
                }
                CubeRotation::Y => {
                    for piece in &mut cube_pieces {
                        // Rotate faces
                        for face in piece.faces {
                            rotate_face(
                                &mut commands,
                                &mut faces_query,
                                &mut cube,
                                face,
                                cube_transform.translation,
                                &cube_rotation_event.animation,
                                Axis::Y,
                                rotation_amount,
                            );
                        }

                        // Update piece indices
                        let new_x: i32;
                        let new_z: i32;

                        if cube_rotation_event.twice {
                            new_x = piece.current_x * -1;
                            new_z = piece.current_z * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_x = piece.current_z * -1;
                            new_z = piece.current_x;
                        } else {
                            new_x = piece.current_z;
                            new_z = piece.current_x * -1;
                        }

                        piece.current_x = new_x;
                        piece.current_z = new_z;
                    }
                }
                CubeRotation::Z => {
                    for piece in &mut cube_pieces {
                        // Rotate faces
                        for face in piece.faces {
                            rotate_face(
                                &mut commands,
                                &mut faces_query,
                                &mut cube,
                                face,
                                cube_transform.translation,
                                &cube_rotation_event.animation,
                                Axis::Z,
                                rotation_amount,
                            );
                        }

                        // Update piece indices
                        let new_x: i32;
                        let new_y: i32;

                        if cube_rotation_event.twice {
                            new_x = piece.current_x * -1;
                            new_y = piece.current_y * -1;
                        } else if cube_rotation_event.negative_direction {
                            new_x = piece.current_y;
                            new_y = piece.current_x * -1;
                        } else {
                            new_x = piece.current_y * -1;
                            new_y = piece.current_x;
                        }

                        piece.current_x = new_x;
                        piece.current_y = new_y;
                    }
                }
            },
        }
    }
}

fn rotate_face(
    commands: &mut Commands,
    faces_query: &mut Query<&mut Transform, With<PieceFace>>,
    cube: &mut Cube,
    face: Entity,
    pivot_point: Vec3,
    animation: &Option<RotationAnimation>,
    axis: Axis,
    rotation_amount: f32,
) {
    let Ok(mut transform) = faces_query.get_mut(face) else {
        error!("failed to get cube face transform for rotating");
        return;
    };

    match animation {
        Some(animation_properties) => {
            let animator =
                RotationAnimator::new(animation_properties, rotation_amount, axis, pivot_point);
            commands.entity(face).insert(animator);

            cube.is_animating_rotation = true;
        }
        None => {
            let rotation = match axis {
                Axis::X => Quat::from_rotation_x(rotation_amount),
                Axis::Y => Quat::from_rotation_y(rotation_amount),
                Axis::Z => Quat::from_rotation_z(rotation_amount),
            };
            transform.rotate_around(pivot_point, rotation);
        }
    };
}

fn handle_rotation_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut RotationAnimator)>,
    mut cube_query: Query<&mut Cube>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animation) in query.iter_mut() {
        let mut progress = time.delta_seconds() / animation.duration_in_seconds;
        animation.progress += progress;

        // prevent overshooting the target
        if animation.progress >= 1.0 {
            progress -= animation.progress - 1.0;
            animation.progress = 1.0;
        }

        let angle = animation.amount_to_rotate * progress;
        let rotation = match animation.axis {
            Axis::X => Quat::from_rotation_x(angle),
            Axis::Y => Quat::from_rotation_y(angle),
            Axis::Z => Quat::from_rotation_z(angle),
        };

        transform.rotate_around(animation.pivot_point, rotation);

        if animation.progress >= 1.0 {
            commands.entity(entity).remove::<RotationAnimator>();
            cube_query.get_single_mut().unwrap().is_animating_rotation = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CubeRotationEvent, FaceRotation, Rotation};

    #[test]
    fn cube_rotation_event_negates() {
        let rotation_event = CubeRotationEvent {
            rotation: Rotation::Face(FaceRotation::X(vec![-1])),
            negative_direction: true,
            twice: false,
            animation: None,
        };

        assert_eq!(rotation_event.negates(&rotation_event), false);

        let negating_event = CubeRotationEvent {
            rotation: Rotation::Face(FaceRotation::X(vec![-1])),
            negative_direction: false,
            twice: false,
            animation: None,
        };

        assert!(negating_event.negates(&rotation_event));

        // it does not negate because its on a different axis
        let non_negating_event = CubeRotationEvent {
            rotation: Rotation::Face(FaceRotation::Y(vec![-1])),
            negative_direction: false,
            twice: false,
            animation: None,
        };

        assert_eq!(non_negating_event.negates(&rotation_event), false);
    }
}
