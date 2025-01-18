use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::schedules::CubeScheduleSet;

use super::{
    axis::Axis,
    cube::{Cube, CubeSize, Piece},
    cube_state::CubeState,
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
    pub ease_function: Option<EaseFunction>,
}

#[derive(Component)]
struct RotationAnimator {
    start: Transform,
    progress: f32, // between 0.0 and 1.0
    duration_in_seconds: f32,
    ease_function: EaseFunction,
    amount_to_rotate: f32, // in radians
    axis: Axis,
    pivot_point: Vec3,
}

impl RotationAnimator {
    fn new(
        start: Transform,
        animation: &RotationAnimation,
        amount_to_rotate: f32,
        axis: Axis,
        pivot_point: Vec3,
    ) -> Self {
        Self {
            start,
            progress: 0.0,
            duration_in_seconds: animation.duration_in_seconds,
            ease_function: animation.ease_function.unwrap_or(EaseFunction::Linear),
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
    pub fn random_face_rotation(cube_size: &CubeSize) -> Self {
        let face_rotation = FaceRotation::random(cube_size);

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

    pub fn equals(&self, comparison: &Self) -> bool {
        return self.rotation == comparison.rotation
            && self.negative_direction == comparison.negative_direction
            && self.twice == comparison.twice;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rotation {
    Face(FaceRotation),
    Cube(CubeRotation),
}

impl Rotation {
    pub fn face_x(slice: i32) -> Self {
        Self::Face(FaceRotation::x(slice))
    }
    pub fn face_y(slice: i32) -> Self {
        Self::Face(FaceRotation::y(slice))
    }
    pub fn face_z(slice: i32) -> Self {
        Self::Face(FaceRotation::z(slice))
    }
    pub fn cube_x() -> Self {
        Self::Cube(CubeRotation::X)
    }
    pub fn cube_y() -> Self {
        Self::Cube(CubeRotation::Y)
    }
    pub fn cube_z() -> Self {
        Self::Cube(CubeRotation::Z)
    }
}

/// Rotate the given faces (e.g. slices) of the cube on a given axis. This is relative to the current cube rotation.
/// For even sized cubes (2x2, 4x4) there is no slice 0.
#[derive(Clone, Debug, PartialEq)]
pub enum FaceRotation {
    /// Rotate the given slices on the x axis.
    /// For a rotation in the default direction, when looking at the front of the cube, the front row ends up at the bottom.
    /// The positive slice indices are on the right of the cube and the negative slice indices are at the left of the cube.
    X(Vec<i32>),
    /// Rotate the given slices on the y axis.
    /// For a rotation in the default direction, when looking at the front of the cube, the front row ends up at the right side.
    /// The positive slice indices are on the top of the cube and the negative slice indices are at the bottom of the cube.
    Y(Vec<i32>),
    /// Rotate the given slices on the z axis.
    /// For a rotation in the default direction, when looking at the front of the cube, the top row ends up at the left side.
    /// The positive slice indices are on the front of the cube and the negative slice indices are on the back of the cube.
    Z(Vec<i32>),
}

impl FaceRotation {
    pub fn random(cube_size: &CubeSize) -> Self {
        let mut rng = rand::thread_rng();

        let slice = if cube_size.0 % 2 == 1 {
            rng.gen_range(cube_size.lowest_piece_index()..=cube_size.highest_piece_index())
        } else {
            let mut result = if cube_size.0 == 2 {
                1
            } else {
                rng.gen_range(1..(cube_size.0 / 2))
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

    pub fn x(slice: i32) -> Self {
        Self::X(vec![slice])
    }
    pub fn y(slice: i32) -> Self {
        Self::Y(vec![slice])
    }
    pub fn z(slice: i32) -> Self {
        Self::Z(vec![slice])
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
    mut cube_state_query: Query<&mut CubeState>,
    mut cube_pieces_query: Query<(Entity, &mut Piece, &mut Transform)>,
    mut event_reader: EventReader<CubeRotationEvent>,
) {
    let Ok(mut cube) = cube_query.get_single_mut() else {
        error!("expected exactly 1 Cube entity");
        return;
    };
    let Ok(mut cube_state) = cube_state_query.get_single_mut() else {
        error!("expected exactly 1 Cube Transform entity");
        return;
    };

    let mut cube_piece_entities = Vec::new();
    let mut cube_pieces = Vec::new();
    let mut cube_piece_transforms = Vec::new();
    let mut number_of_pieces = 0;

    for (entity, piece, transform) in cube_pieces_query.iter_mut() {
        cube_piece_entities.push(entity);
        cube_pieces.push(piece.into_inner());
        cube_piece_transforms.push(transform.into_inner());
        number_of_pieces += 1;
    }

    for cube_rotation_event in event_reader.read() {
        if cube.is_animating_rotation {
            warn!("Skipping cube rotation event because cube is already rotating");
            continue;
        }

        cube_state.handle_rotate_event(cube_rotation_event);

        let mut rotation_amount = TAU / 4.0;

        if cube_rotation_event.twice {
            rotation_amount *= 2.;
        }

        if cube_rotation_event.negative_direction {
            rotation_amount *= -1.;
        }

        match &cube_rotation_event.rotation {
            Rotation::Face(face_rotation) => {
                let cube_size: f32 = cube.size().0 as f32;
                let cube_piece_spread = cube.piece_spread;
                let pivot_coordinate = |slice: &i32| {
                    return *slice as f32 * (cube_size + cube_piece_spread);
                };

                match face_rotation {
                    FaceRotation::X(slices) => {
                        for slice in slices {
                            if cube.size().0 % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

                            let pivot_point = Vec3::new(pivot_coordinate(slice), 0.0, 0.0);
                            let piece_indices_to_rotate =
                                Piece::get_piece_indices(&cube_pieces, Axis::X, *slice);

                            // Rotate pieces
                            for piece_index_to_rotate in &piece_indices_to_rotate {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[*piece_index_to_rotate],
                                    cube_piece_transforms[*piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
                                    &cube_rotation_event.animation,
                                    Axis::X,
                                    rotation_amount,
                                );
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(piece_indices_to_rotate.len());

                            for cube_index_to_rotate in &piece_indices_to_rotate {
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

                            for (i, piece_index) in piece_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*piece_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*piece_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*piece_index].current_z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Y(slices) => {
                        for slice in slices {
                            if cube.size().0 % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

                            let pivot_point = Vec3::new(0.0, pivot_coordinate(slice), 0.0);
                            let piece_indices_to_rotate =
                                Piece::get_piece_indices(&cube_pieces, Axis::Y, *slice);

                            // Rotate pieces
                            for piece_index_to_rotate in &piece_indices_to_rotate {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[*piece_index_to_rotate],
                                    &mut cube_piece_transforms[*piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
                                    &cube_rotation_event.animation,
                                    Axis::Y,
                                    rotation_amount,
                                );
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(piece_indices_to_rotate.len());

                            for cube_index_to_rotate in &piece_indices_to_rotate {
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

                            for (i, piece_index) in piece_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*piece_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*piece_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*piece_index].current_z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                    FaceRotation::Z(slices) => {
                        for slice in slices {
                            if cube.size().0 % 2 == 0 && *slice == 0 {
                                warn!("Cube can not rotate slice with index 0 for even cubes");
                                continue;
                            }

                            let pivot_point = Vec3::new(0.0, 0.0, pivot_coordinate(slice));
                            let piece_indices_to_rotate =
                                Piece::get_piece_indices(&cube_pieces, Axis::Z, *slice);

                            // Rotate pieces
                            for piece_index_to_rotate in &piece_indices_to_rotate {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[*piece_index_to_rotate],
                                    &mut cube_piece_transforms[*piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
                                    &cube_rotation_event.animation,
                                    Axis::Z,
                                    rotation_amount,
                                );
                            }

                            // Update piece indices
                            let mut new_pieces_coordinates: Vec<(i32, i32, i32)> =
                                Vec::with_capacity(piece_indices_to_rotate.len());

                            for cube_index_to_rotate in &piece_indices_to_rotate {
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

                            for (i, piece_index) in piece_indices_to_rotate.iter().enumerate() {
                                cube_pieces[*piece_index].current_x = new_pieces_coordinates[i].0;
                                cube_pieces[*piece_index].current_y = new_pieces_coordinates[i].1;
                                cube_pieces[*piece_index].current_z = new_pieces_coordinates[i].2;
                            }
                        }
                    }
                }
            }
            Rotation::Cube(cube_rotation) => {
                let pivot_point = Vec3::ZERO;

                match cube_rotation {
                    CubeRotation::X => {
                        for piece in &mut cube_pieces {
                            // Rotate pieces
                            for piece_index_to_rotate in 0..number_of_pieces {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[piece_index_to_rotate],
                                    &mut cube_piece_transforms[piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
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
                                new_y = piece.current_z;
                                new_z = piece.current_y * -1;
                            } else {
                                new_y = piece.current_z * -1;
                                new_z = piece.current_y;
                            }

                            piece.current_y = new_y;
                            piece.current_z = new_z;
                        }
                    }
                    CubeRotation::Y => {
                        for piece in &mut cube_pieces {
                            // Rotate pieces
                            for piece_index_to_rotate in 0..number_of_pieces {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[piece_index_to_rotate],
                                    &mut cube_piece_transforms[piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
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
                            // Rotate pieces
                            for piece_index_to_rotate in 0..number_of_pieces {
                                rotate_piece(
                                    &mut commands,
                                    cube_piece_entities[piece_index_to_rotate],
                                    &mut cube_piece_transforms[piece_index_to_rotate],
                                    &mut cube,
                                    pivot_point,
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
                }
            }
        }
    }
}

fn rotate_piece(
    commands: &mut Commands,
    piece_entity: Entity,
    piece_transform: &mut Transform,
    cube: &mut Cube,
    pivot_point: Vec3,
    animation: &Option<RotationAnimation>,
    axis: Axis,
    rotation_amount: f32,
) {
    match animation {
        Some(animation_properties) => {
            let animator = RotationAnimator::new(
                piece_transform.clone(),
                animation_properties,
                rotation_amount,
                axis,
                pivot_point,
            );
            commands.entity(piece_entity).insert(animator);

            cube.is_animating_rotation = true;
        }
        None => {
            let rotation = match axis {
                Axis::X => Quat::from_rotation_x(rotation_amount),
                Axis::Y => Quat::from_rotation_y(rotation_amount),
                Axis::Z => Quat::from_rotation_z(rotation_amount),
            };
            piece_transform.rotate_around(pivot_point, rotation);
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
        animation.progress += time.delta_secs() / animation.duration_in_seconds;
        animation.progress = animation.progress.clamp(0.0, 1.0);

        let eased_progress = EasingCurve::new(0.0, 1.0, animation.ease_function)
            .sample(animation.progress)
            .unwrap();

        let angle = eased_progress * animation.amount_to_rotate;
        let rotation = match animation.axis {
            Axis::X => Quat::from_rotation_x(angle),
            Axis::Y => Quat::from_rotation_y(angle),
            Axis::Z => Quat::from_rotation_z(angle),
        };
        let mut new_transform = animation.start.clone();
        new_transform.rotate_around(animation.pivot_point, rotation);

        transform.rotation = new_transform.rotation;
        transform.translation = new_transform.translation;

        // cleanup if animation is done
        if animation.progress >= 1.0 {
            commands.entity(entity).remove::<RotationAnimator>();
            cube_query.get_single_mut().unwrap().is_animating_rotation = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CubeRotationEvent, Rotation};

    #[test]
    fn cube_rotation_event_negates() {
        let rotation_event = CubeRotationEvent {
            rotation: Rotation::face_x(-1),
            negative_direction: true,
            twice: false,
            animation: None,
        };

        assert_eq!(rotation_event.negates(&rotation_event), false);

        let negating_event = CubeRotationEvent {
            rotation: Rotation::face_x(-1),
            negative_direction: false,
            twice: false,
            animation: None,
        };

        assert!(negating_event.negates(&rotation_event));

        // it does not negate because its on a different axis
        let non_negating_event = CubeRotationEvent {
            rotation: Rotation::face_y(-1),
            negative_direction: false,
            twice: false,
            animation: None,
        };

        assert_eq!(non_negating_event.negates(&rotation_event), false);
    }
}
