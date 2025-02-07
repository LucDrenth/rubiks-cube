use bevy::{log, prelude::*};

use crate::utils::console;

use super::{slice::slice_to_column_index, CubeRotationEvent};

/// Holds an efficient and precise state of a cube.
///
/// Only the outwards facing faces are stored, resulting in `n^2 * 6` stored elements where `n = cube_size`.
#[derive(Component)]
pub struct CubeState {
    cube_size: usize,
    face_states: FaceStates,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Face {
    Left = 0,
    Right = 1,
    Top = 2,
    Bottom = 3,
    Front = 4,
    Back = 5,
}

impl Face {
    #[allow(dead_code)]
    pub fn as_colored_string(&self) -> String {
        return format!(
            "{}{:?}{}",
            self.to_console_color(),
            self,
            console::COLOR_CLEAR
        );
    }

    fn to_console_color(&self) -> &str {
        match &self {
            Face::Left => console::COLOR_ORANGE,
            Face::Right => console::COLOR_RED,
            Face::Top => console::COLOR_WHITE,
            Face::Bottom => console::COLOR_YELLOW,
            Face::Front => console::COLOR_GREEN,
            Face::Back => console::COLOR_BLUE,
        }
    }
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

#[derive(Clone, PartialEq, Debug)]
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

    fn rotate_left(&mut self, cube_size: usize) {
        let mut new_state = self.0.clone();

        for i in 0..self.0.len() {
            new_state[Self::rotate_face_index_left(i, cube_size)] = self.0[i].clone();
        }

        self.0 = new_state;
    }

    fn rotate_right(&mut self, cube_size: usize) {
        let mut new_state = self.0.clone();

        for i in 0..self.0.len() {
            new_state[Self::rotate_face_index_right(i, cube_size)] = self.0[i].clone();
        }

        self.0 = new_state;
    }

    fn rotate_face_index_left(i: usize, cube_size: usize) -> usize {
        let x = i % cube_size;
        let y = i / cube_size;
        y + (cube_size - x - 1) * cube_size
    }

    fn rotate_face_index_right(i: usize, cube_size: usize) -> usize {
        let x = i % cube_size;
        let y = i / cube_size;
        (cube_size - y - 1) + x * cube_size
    }
}

impl CubeState {
    pub fn new(cube_size: usize) -> Self {
        let result = Self {
            cube_size,
            face_states: FaceStates::new(cube_size),
        };
        result
    }

    pub fn is_solved(&self) -> bool {
        return self.face_states.left.is_solved()
            && self.face_states.right.is_solved()
            && self.face_states.top.is_solved()
            && self.face_states.bottom.is_solved()
            && self.face_states.front.is_solved()
            && self.face_states.back.is_solved();
    }

    fn can_rotate_slice(&self, slice: &i32) -> bool {
        return slice_to_column_index(*slice, self.cube_size) < self.cube_size as usize;
    }

    pub fn handle_rotate_event(&mut self, event: &CubeRotationEvent) {
        let iterations = if event.twice { 2 } else { 1 };

        for _ in 0..iterations {
            match &event.rotation {
                super::rotation::Rotation::Face(face_rotation) => match face_rotation {
                    super::rotation::FaceRotation::X(slices) => {
                        for slice in slices {
                            if !self.can_rotate_slice(slice) {
                                log::warn!("Can not rotate slice {} on cube size {}. Skipping this rotation event.", *slice, self.cube_size);
                                continue;
                            }

                            let mut new_face_states = self.face_states.clone();

                            for i in 0..self.cube_size {
                                let face_index_top = slice_to_column_index(*slice, self.cube_size)
                                    + i * self.cube_size;

                                let face_index_back = self.cube_size
                                    - slice_to_column_index(*slice, self.cube_size)
                                    - 1
                                    + (self.cube_size - i - 1) * self.cube_size;

                                let face_index_bottom =
                                    slice_to_column_index(*slice, self.cube_size)
                                        + i * self.cube_size;

                                let face_index_front =
                                    slice_to_column_index(*slice, self.cube_size)
                                        + i * self.cube_size;

                                if event.negative_direction {
                                    // front to top
                                    new_face_states.top.0[face_index_top] =
                                        self.face_states.front.0[face_index_front].clone();

                                    // bottom to front
                                    new_face_states.front.0[face_index_front] =
                                        self.face_states.bottom.0[face_index_bottom].clone();

                                    // back to bottom
                                    new_face_states.bottom.0[face_index_bottom] =
                                        self.face_states.back.0[face_index_back].clone();

                                    // top to back
                                    new_face_states.back.0[face_index_back] =
                                        self.face_states.top.0[face_index_top].clone();
                                } else {
                                    // top to front
                                    new_face_states.front.0[face_index_front] =
                                        self.face_states.top.0[face_index_top].clone();

                                    // front to bottom
                                    new_face_states.bottom.0[face_index_bottom] =
                                        self.face_states.front.0[face_index_front].clone();

                                    // bottom to back
                                    new_face_states.back.0[face_index_back] =
                                        self.face_states.bottom.0[face_index_bottom].clone();

                                    // back to top
                                    new_face_states.top.0[face_index_top] =
                                        self.face_states.back.0[face_index_back].clone();
                                }
                            }

                            if event.negative_direction {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.right.rotate_right(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.left.rotate_left(self.cube_size);
                                }
                            } else {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.right.rotate_left(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.left.rotate_right(self.cube_size);
                                }
                            }

                            self.face_states = new_face_states;
                        }
                    }
                    super::rotation::FaceRotation::Y(slices) => {
                        for slice in slices {
                            if !self.can_rotate_slice(slice) {
                                log::warn!("Can not rotate slice {} on cube size {}. Skipping this rotation event.", *slice, self.cube_size);
                                continue;
                            }

                            let mut new_face_states = self.face_states.clone();

                            for i in 0..self.cube_size {
                                // index is the same for all sides
                                let face_index = (self.cube_size
                                    - slice_to_column_index(*slice, self.cube_size)
                                    - 1)
                                    * self.cube_size
                                    + i;

                                if event.negative_direction {
                                    // front to left
                                    new_face_states.left.0[face_index] =
                                        self.face_states.front.0[face_index].clone();

                                    // left to back
                                    new_face_states.back.0[face_index] =
                                        self.face_states.left.0[face_index].clone();

                                    // back to right
                                    new_face_states.right.0[face_index] =
                                        self.face_states.back.0[face_index].clone();

                                    // right to front
                                    new_face_states.front.0[face_index] =
                                        self.face_states.right.0[face_index].clone();
                                } else {
                                    // front to right
                                    new_face_states.right.0[face_index] =
                                        self.face_states.front.0[face_index].clone();

                                    // right to back
                                    new_face_states.back.0[face_index] =
                                        self.face_states.right.0[face_index].clone();

                                    // back to left
                                    new_face_states.left.0[face_index] =
                                        self.face_states.back.0[face_index].clone();

                                    // left to front
                                    new_face_states.front.0[face_index] =
                                        self.face_states.left.0[face_index].clone();
                                }
                            }

                            if event.negative_direction {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.top.rotate_right(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.bottom.rotate_left(self.cube_size);
                                }
                            } else {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.top.rotate_left(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.bottom.rotate_right(self.cube_size);
                                }
                            }

                            self.face_states = new_face_states;
                        }
                    }
                    super::rotation::FaceRotation::Z(slices) => {
                        for slice in slices {
                            if !self.can_rotate_slice(slice) {
                                log::warn!("Can not rotate slice {} on cube size {}. Skipping this rotation event.", *slice, self.cube_size);
                                continue;
                            }

                            let mut new_face_states = self.face_states.clone();

                            for i in 0..self.cube_size {
                                let face_index_top = slice_to_column_index(*slice, self.cube_size)
                                    * self.cube_size
                                    + i;

                                let face_index_right = self.cube_size
                                    - slice_to_column_index(*slice, self.cube_size)
                                    - 1
                                    + i * self.cube_size;

                                let face_index_bottom = invert_face_index_x(
                                    (self.cube_size
                                        - slice_to_column_index(*slice, self.cube_size)
                                        - 1)
                                        * self.cube_size
                                        + i,
                                    self.cube_size,
                                );

                                let face_index_left = invert_face_index_y(
                                    slice_to_column_index(*slice, self.cube_size)
                                        + i * self.cube_size,
                                    self.cube_size,
                                );

                                if event.negative_direction {
                                    // top to right
                                    new_face_states.right.0[face_index_right] =
                                        self.face_states.top.0[face_index_top].clone();

                                    // right to bottom
                                    new_face_states.bottom.0[face_index_bottom] =
                                        self.face_states.right.0[face_index_right].clone();

                                    // bottom to left
                                    new_face_states.left.0[face_index_left] =
                                        self.face_states.bottom.0[face_index_bottom].clone();

                                    // left to top
                                    new_face_states.top.0[face_index_top] =
                                        self.face_states.left.0[face_index_left].clone();
                                } else {
                                    // top to left
                                    new_face_states.left.0[face_index_left] =
                                        self.face_states.top.0[face_index_top].clone();

                                    // left to bottom
                                    new_face_states.bottom.0[face_index_bottom] =
                                        self.face_states.left.0[face_index_left].clone();

                                    // bottom to right
                                    new_face_states.right.0[face_index_right] =
                                        self.face_states.bottom.0[face_index_bottom].clone();

                                    // right to top
                                    new_face_states.top.0[face_index_top] =
                                        self.face_states.right.0[face_index_right].clone();
                                }
                            }

                            if event.negative_direction {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.front.rotate_right(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.back.rotate_left(self.cube_size);
                                }
                            } else {
                                if has_edge_on_positive_side(slice, self.cube_size) {
                                    new_face_states.front.rotate_left(self.cube_size);
                                } else if has_edge_on_negative_side(slice, self.cube_size) {
                                    new_face_states.back.rotate_right(self.cube_size);
                                }
                            }

                            self.face_states = new_face_states;
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

    /// Prints the current state in an unfolded format for debugging. The formatting does not work well for cubes 4x4 cubes or bigger.
    #[allow(dead_code)]
    fn print_indices(&self) {
        // top
        for y in 0..self.cube_size {
            print!("\t");
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.top.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            println!();
        }

        println!();

        // left, front, right, bottom
        for y in 0..self.cube_size {
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.left.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            print!("\t");
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.front.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            print!("\t");
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.right.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            print!("\t");
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.back.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            println!();
        }

        println!();

        // bottom
        for y in 0..self.cube_size {
            print!("\t");
            for x in 0..self.cube_size {
                print!(
                    "{}{}{} ",
                    self.face_states.bottom.0[x + y * self.cube_size].to_console_color(),
                    x + y * self.cube_size,
                    console::COLOR_CLEAR
                );
            }
            println!();
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

fn has_edge_on_positive_side(slice: &i32, cube_size: usize) -> bool {
    if cube_size % 2 == 0 {
        return *slice == (cube_size as i32 / 2);
    } else {
        return *slice == ((cube_size as i32 - 1) / 2);
    }
}

fn has_edge_on_negative_side(slice: &i32, cube_size: usize) -> bool {
    if cube_size % 2 == 0 {
        return *slice == -(cube_size as i32 / 2);
    } else {
        return *slice == -((cube_size as i32 - 1) / 2);
    }
}

fn invert_face_index_x(face_index: usize, cube_size: usize) -> usize {
    let base = face_index % cube_size;
    return cube_size - 1 + face_index - base * 2;
}

fn invert_face_index_y(face_index: usize, cube_size: usize) -> usize {
    let row_index = face_index / cube_size;
    let inverted_base = invert_face_index_x(row_index, cube_size);
    return inverted_base * cube_size + face_index % cube_size;
}

#[cfg(test)]
mod tests {
    use crate::cube::{
        algorithms, create_scramble_sequence_from_algorithm,
        cube_state::{
            has_edge_on_negative_side, has_edge_on_positive_side, invert_face_index_x,
            invert_face_index_y, Face,
        },
        rotation::{CubeRotation, FaceRotation, Rotation},
        CubeRotationEvent,
    };

    use super::{CubeState, FaceState};

    #[test]
    fn test_face_state_is_solved() {
        let unsolved_face_state = FaceState(vec![Face::Left, Face::Left, Face::Right]);
        let solved_face_state = FaceState(vec![Face::Left, Face::Left, Face::Left]);

        assert!(!unsolved_face_state.is_solved());
        assert!(solved_face_state.is_solved());
    }

    #[test]
    fn rotate_face_index_left() {
        // 2x2
        assert_eq!(2, FaceState::rotate_face_index_left(0, 2));
        assert_eq!(0, FaceState::rotate_face_index_left(1, 2));
        assert_eq!(3, FaceState::rotate_face_index_left(2, 2));
        assert_eq!(1, FaceState::rotate_face_index_left(3, 2));

        // 3x3
        assert_eq!(6, FaceState::rotate_face_index_left(0, 3));
        assert_eq!(3, FaceState::rotate_face_index_left(1, 3));
        assert_eq!(0, FaceState::rotate_face_index_left(2, 3));
        assert_eq!(7, FaceState::rotate_face_index_left(3, 3));
        assert_eq!(4, FaceState::rotate_face_index_left(4, 3));
        assert_eq!(1, FaceState::rotate_face_index_left(5, 3));
        assert_eq!(8, FaceState::rotate_face_index_left(6, 3));
        assert_eq!(5, FaceState::rotate_face_index_left(7, 3));
        assert_eq!(2, FaceState::rotate_face_index_left(8, 3));
    }

    #[test]
    fn rotate_face_index_right() {
        // 2x2
        assert_eq!(1, FaceState::rotate_face_index_right(0, 2));
        assert_eq!(3, FaceState::rotate_face_index_right(1, 2));
        assert_eq!(0, FaceState::rotate_face_index_right(2, 2));
        assert_eq!(2, FaceState::rotate_face_index_right(3, 2));

        // 3x3
        assert_eq!(2, FaceState::rotate_face_index_right(0, 3));
        assert_eq!(5, FaceState::rotate_face_index_right(1, 3));
        assert_eq!(8, FaceState::rotate_face_index_right(2, 3));
        assert_eq!(1, FaceState::rotate_face_index_right(3, 3));
        assert_eq!(4, FaceState::rotate_face_index_right(4, 3));
        assert_eq!(7, FaceState::rotate_face_index_right(5, 3));
        assert_eq!(0, FaceState::rotate_face_index_right(6, 3));
        assert_eq!(3, FaceState::rotate_face_index_right(7, 3));
        assert_eq!(6, FaceState::rotate_face_index_right(8, 3));
    }

    #[test]
    fn test_rotate_face_state_3x3() {
        #[rustfmt::skip]
        let original_state = FaceState(vec![
            Face::Left,     Face::Top,      Face::Right,
            Face::Right,    Face::Bottom,   Face::Left,
            Face::Top,      Face::Left,     Face::Bottom,
        ]);

        let mut current_state = original_state.clone();

        #[rustfmt::skip]
        let face_state_after_rotate_right = FaceState(vec![
            Face::Top,      Face::Right,    Face::Left,
            Face::Left,     Face::Bottom,   Face::Top,
            Face::Bottom,   Face::Left,     Face::Right,
        ]);

        current_state.rotate_right(3);
        assert_eq!(face_state_after_rotate_right, current_state);
        current_state.rotate_left(3);
        assert_eq!(original_state, current_state);
    }

    #[test]
    fn test_rotate_face_state_4x4() {
        #[rustfmt::skip]
        let original_state = FaceState(vec![
            Face::Left,     Face::Top,      Face::Right,    Face::Back,
            Face::Right,    Face::Back,     Face::Left,     Face::Top,
            Face::Top,      Face::Front,    Face::Bottom,   Face::Right,
            Face::Bottom,   Face::Right,    Face::Top,      Face::Front,
        ]);

        let mut current_state = original_state.clone();

        #[rustfmt::skip]
        let face_state_after_rotate_right = FaceState(vec![
            Face::Bottom,   Face::Top,      Face::Right,    Face::Left,
            Face::Right,    Face::Front,    Face::Back,     Face::Top,
            Face::Top,      Face::Bottom,   Face::Left,     Face::Right,
            Face::Front,    Face::Right,    Face::Top,      Face::Back,
        ]);

        current_state.rotate_right(4);
        assert_eq!(face_state_after_rotate_right, current_state);
        current_state.rotate_left(4);
        assert_eq!(original_state, current_state);
    }

    #[test]
    fn test_handle_face_rotation_event() {
        // tests that the cube state is solved after doing the same rotation 4 times
        let test_4_face_rotations =
            |cube_size: usize, face_rotation: FaceRotation, invert_direction: bool| {
                let mut cube_state = CubeState::new(cube_size);
                assert!(cube_state.is_solved());

                cube_state.print_indices();
                println!();

                for i in 1..=4 {
                    cube_state.handle_rotate_event(&CubeRotationEvent {
                        rotation: Rotation::Face(face_rotation.clone()),
                        negative_direction: invert_direction,
                        twice: false,
                        animation: None,
                    });

                    if i < 4 {
                        assert!(!cube_state.is_solved());
                    }

                    println!("========================");
                    cube_state.print_indices();
                    println!();
                }

                assert!(cube_state.is_solved());
            };

        for cube_size in 2..=4 {
            // text x rotations
            test_4_face_rotations(cube_size, FaceRotation::x(-1), false);
            test_4_face_rotations(cube_size, FaceRotation::x(0), false);
            test_4_face_rotations(cube_size, FaceRotation::x(1), false);
            test_4_face_rotations(cube_size, FaceRotation::x(-1), true);
            test_4_face_rotations(cube_size, FaceRotation::x(0), true);
            test_4_face_rotations(cube_size, FaceRotation::x(1), true);

            // text y rotations,
            test_4_face_rotations(cube_size, FaceRotation::y(-1), false);
            test_4_face_rotations(cube_size, FaceRotation::y(0), false);
            test_4_face_rotations(cube_size, FaceRotation::y(1), false);
            test_4_face_rotations(cube_size, FaceRotation::y(-1), true);
            test_4_face_rotations(cube_size, FaceRotation::y(0), true);
            test_4_face_rotations(cube_size, FaceRotation::y(1), true);

            // text z rotations,
            test_4_face_rotations(cube_size, FaceRotation::z(-1), false);
            test_4_face_rotations(cube_size, FaceRotation::z(0), false);
            test_4_face_rotations(cube_size, FaceRotation::z(1), false);
            test_4_face_rotations(cube_size, FaceRotation::z(-1), true);
            test_4_face_rotations(cube_size, FaceRotation::z(0), true);
            test_4_face_rotations(cube_size, FaceRotation::z(1), true);
        }

        println!("====== END test_4_face_rotations ======\n");

        // rotation sequence where pieces are in the same spot, but not in the correct orientation.
        let mut cube_state = CubeState::new(3);
        assert!(cube_state.is_solved());
        let scramble =
            create_scramble_sequence_from_algorithm(algorithms::size_3x3::flipped_pieces());

        cube_state.print_indices();
        println!("start: -------------------------");

        for event in &scramble {
            cube_state.handle_rotate_event(event);

            cube_state.print_indices();
            println!("-------------------------");
        }
        assert!(!cube_state.is_solved());

        println!("end first sequence: -------------------------");
        cube_state.print_indices();

        for event in &scramble {
            cube_state.handle_rotate_event(event);
        }

        println!("end second sequence: -------------------------");
        cube_state.print_indices();

        assert!(cube_state.is_solved());
    }

    #[test]
    fn test_handle_cube_rotation_event() {
        // tests that the cube state is solved after doing the same rotation 4 times
        let test_4_cube_rotations =
            |cube_size: usize, cube_rotation: CubeRotation, invert_direction: bool| {
                let mut cube_state = CubeState::new(cube_size);
                assert!(cube_state.is_solved());

                for _ in 1..=4 {
                    cube_state.handle_rotate_event(&CubeRotationEvent {
                        rotation: Rotation::Cube(cube_rotation.clone()),
                        negative_direction: invert_direction,
                        twice: false,
                        animation: None,
                    });

                    assert!(cube_state.is_solved());
                }
            };

        test_4_cube_rotations(3, CubeRotation::X, true);
        test_4_cube_rotations(3, CubeRotation::X, false);
        test_4_cube_rotations(3, CubeRotation::Y, true);
        test_4_cube_rotations(3, CubeRotation::Y, false);
        test_4_cube_rotations(3, CubeRotation::Z, true);
        test_4_cube_rotations(3, CubeRotation::Z, false);

        test_4_cube_rotations(4, CubeRotation::X, true);
        test_4_cube_rotations(4, CubeRotation::X, false);
        test_4_cube_rotations(4, CubeRotation::Y, true);
        test_4_cube_rotations(4, CubeRotation::Y, false);
        test_4_cube_rotations(4, CubeRotation::Z, true);
        test_4_cube_rotations(4, CubeRotation::Z, false);
    }

    #[test]
    fn test_has_edge_on_positive_side() {
        // 3x3
        assert_eq!(false, has_edge_on_positive_side(&-1, 3));
        assert_eq!(false, has_edge_on_positive_side(&0, 3));
        assert_eq!(true, has_edge_on_positive_side(&1, 3));

        // 4x4
        assert_eq!(false, has_edge_on_positive_side(&-2, 4));
        assert_eq!(false, has_edge_on_positive_side(&-1, 4));
        assert_eq!(false, has_edge_on_positive_side(&1, 4));
        assert_eq!(true, has_edge_on_positive_side(&2, 4));
    }

    #[test]
    fn test_has_edge_on_negative_side() {
        // 3x3
        assert_eq!(true, has_edge_on_negative_side(&-1, 3));
        assert_eq!(false, has_edge_on_negative_side(&0, 3));
        assert_eq!(false, has_edge_on_negative_side(&1, 3));

        // 4x4
        assert_eq!(true, has_edge_on_negative_side(&-2, 4));
        assert_eq!(false, has_edge_on_negative_side(&-1, 4));
        assert_eq!(false, has_edge_on_negative_side(&1, 4));
        assert_eq!(false, has_edge_on_negative_side(&2, 4));
    }

    #[test]
    fn test_invert_face_index_x() {
        // 3x3
        assert_eq!(2, invert_face_index_x(0, 3));
        assert_eq!(1, invert_face_index_x(1, 3));
        assert_eq!(0, invert_face_index_x(2, 3));

        assert_eq!(5, invert_face_index_x(3, 3));
        assert_eq!(4, invert_face_index_x(4, 3));
        assert_eq!(3, invert_face_index_x(5, 3));

        assert_eq!(8, invert_face_index_x(6, 3));
        assert_eq!(7, invert_face_index_x(7, 3));
        assert_eq!(6, invert_face_index_x(8, 3));

        // 4x4
        assert_eq!(3, invert_face_index_x(0, 4));
        assert_eq!(2, invert_face_index_x(1, 4));
        assert_eq!(1, invert_face_index_x(2, 4));
        assert_eq!(0, invert_face_index_x(3, 4));

        assert_eq!(7, invert_face_index_x(4, 4));
        assert_eq!(6, invert_face_index_x(5, 4));
        assert_eq!(5, invert_face_index_x(6, 4));
        assert_eq!(4, invert_face_index_x(7, 4));
    }

    #[test]
    fn test_invert_face_index_y() {
        // 3x3
        assert_eq!(6, invert_face_index_y(0, 3));
        assert_eq!(3, invert_face_index_y(3, 3));
        assert_eq!(0, invert_face_index_y(6, 3));

        assert_eq!(7, invert_face_index_y(1, 3));
        assert_eq!(4, invert_face_index_y(4, 3));
        assert_eq!(1, invert_face_index_y(7, 3));

        assert_eq!(8, invert_face_index_y(2, 3));
        assert_eq!(5, invert_face_index_y(5, 3));
        assert_eq!(2, invert_face_index_y(8, 3));

        // 4x4
        assert_eq!(12, invert_face_index_y(0, 4));
        assert_eq!(8, invert_face_index_y(4, 4));
        assert_eq!(4, invert_face_index_y(8, 4));
        assert_eq!(0, invert_face_index_y(12, 4));

        assert_eq!(13, invert_face_index_y(1, 4));
        assert_eq!(9, invert_face_index_y(5, 4));
        assert_eq!(5, invert_face_index_y(9, 4));
        assert_eq!(1, invert_face_index_y(13, 4));
    }
}
