use bevy::prelude::*;

use super::{cube::Face, CubeRotationEvent};

#[derive(Component)]
pub struct CubeState {
    cube_size: usize,
    face_states: FaceStates,
}

/// The indices of a folded out 3x3 cube would be as follows:
///
///             0 1 2
///             3 4 5                       --> top
///             6 7 8
///
///     0 1 2   0 1 2   0 1 2   0 1 2
///     3 4 5   3 4 5   3 4 5   3 4 5       --> left, front, right, back
///     6 7 8   6 7 8   6 7 8   6 7 8
///
///             0 1 2
///             3 4 5                       --> bottom
///             6 7 8
#[derive(Clone)]
struct FaceStates {
    left: FaceState,
    right: FaceState,
    top: FaceState,
    bottom: FaceState,
    front: FaceState,
    back: FaceState,
}

#[derive(Clone)]
struct FaceState(Vec<Face>);

impl FaceState {
    fn is_solved(&self) -> bool {
        let face_to_match = self.0.first().unwrap().clone();
        for i in 1..self.0.len() {
            if self.0[i] != face_to_match {
                return false;
            }
        }

        return true;
    }
}

impl CubeState {
    pub fn new(cube_size: usize) -> Self {
        Self {
            cube_size,
            face_states: FaceStates::new(cube_size),
        }
    }

    pub fn is_solved(&self) -> bool {
        return self.face_states.left.is_solved()
            && self.face_states.right.is_solved()
            && self.face_states.top.is_solved()
            && self.face_states.bottom.is_solved()
            && self.face_states.front.is_solved()
            && self.face_states.back.is_solved();
    }

    pub fn rotate(&mut self, event: &CubeRotationEvent) {
        let iterations = if event.twice { 2 } else { 1 };

        for _ in 0..iterations {
            match &event.rotation {
                super::rotation::Rotation::Face(face_rotation) => match face_rotation {
                    super::rotation::FaceRotation::X(slices) => {
                        for slice in slices {
                            if event.negative_direction {
                                todo!()
                            } else {
                                todo!()
                            }
                        }
                    }
                    super::rotation::FaceRotation::Y(slices) => {
                        for slice in slices {
                            if event.negative_direction {
                                todo!()
                            } else {
                                todo!()
                            }
                        }
                    }
                    super::rotation::FaceRotation::Z(slices) => {
                        for slice in slices {
                            if event.negative_direction {
                                todo!()
                            } else {
                                todo!()
                            }
                        }
                    }
                },
                super::rotation::Rotation::Cube(cube_rotation) => {
                    let mut new_face_states = self.face_states.clone();

                    match cube_rotation {
                        super::rotation::CubeRotation::X => {
                            if event.negative_direction {
                                new_face_states.top = self.face_states.front.clone();
                                new_face_states.bottom = self.face_states.back.clone();
                                new_face_states.front = self.face_states.bottom.clone();
                                new_face_states.back = self.face_states.top.clone();
                            } else {
                                new_face_states.top = self.face_states.back.clone();
                                new_face_states.bottom = self.face_states.front.clone();
                                new_face_states.front = self.face_states.top.clone();
                                new_face_states.back = self.face_states.bottom.clone();
                            }
                        }
                        super::rotation::CubeRotation::Y => {
                            if event.negative_direction {
                                new_face_states.front = self.face_states.left.clone();
                                new_face_states.back = self.face_states.right.clone();
                                new_face_states.left = self.face_states.back.clone();
                                new_face_states.right = self.face_states.front.clone();
                            } else {
                                new_face_states.front = self.face_states.right.clone();
                                new_face_states.back = self.face_states.left.clone();
                                new_face_states.left = self.face_states.front.clone();
                                new_face_states.right = self.face_states.back.clone();
                            }
                        }
                        super::rotation::CubeRotation::Z => {
                            if event.negative_direction {
                                new_face_states.top = self.face_states.left.clone();
                                new_face_states.bottom = self.face_states.right.clone();
                                new_face_states.left = self.face_states.bottom.clone();
                                new_face_states.right = self.face_states.top.clone();
                            } else {
                                new_face_states.top = self.face_states.right.clone();
                                new_face_states.bottom = self.face_states.left.clone();
                                new_face_states.left = self.face_states.top.clone();
                                new_face_states.right = self.face_states.bottom.clone();
                            }
                        }
                    }

                    self.face_states = new_face_states;
                }
            }
        }
    }
}

impl FaceStates {
    pub fn new(cube_size: usize) -> Self {
        let faces_per_side = cube_size * cube_size;

        Self {
            left: FaceState(vec![Face::Left; faces_per_side]),
            right: FaceState(vec![Face::Right; faces_per_side]),
            top: FaceState(vec![Face::Top; faces_per_side]),
            bottom: FaceState(vec![Face::Bottom; faces_per_side]),
            front: FaceState(vec![Face::Front; faces_per_side]),
            back: FaceState(vec![Face::Back; faces_per_side]),
        }
    }
}
