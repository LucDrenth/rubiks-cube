// This file contains cubing algorithms. A cubing algorithm is a sequence of moves.

pub mod size_3x3 {
    use crate::cube::Rotation3x3;

    // Keep the pieces in the same place, but with 2 edge pieces be flipped.
    // Run this algorithm 2 times to end up in the initial state.
    pub fn flipped_pieces() -> Vec<Rotation3x3> {
        vec![
            Rotation3x3::B2,
            Rotation3x3::R2,
            Rotation3x3::UPrime,
            Rotation3x3::B2,
            Rotation3x3::L2,
            Rotation3x3::F2,
            Rotation3x3::D2,
            Rotation3x3::R2,
            Rotation3x3::DPrime,
            Rotation3x3::L,
            Rotation3x3::RPrime,
            Rotation3x3::F,
            Rotation3x3::R2,
            Rotation3x3::F,
            Rotation3x3::L,
            Rotation3x3::R,
        ]
    }

    /// An algorithm used in F2L.
    /// Run this algorithm 6 times to end up in the inital state.
    /// And yes, this is an actual cubing algorithm name :)
    pub fn sexy_right() -> Vec<Rotation3x3> {
        vec![
            Rotation3x3::R,
            Rotation3x3::U,
            Rotation3x3::RPrime,
            Rotation3x3::UPrime,
        ]
    }

    /// An algorithm used in F2L.
    /// Run this algorithm 6 times to end up in the inital state.
    /// And yes, this is an actual cubing algorithm name :)
    pub fn sexy_left() -> Vec<Rotation3x3> {
        vec![
            Rotation3x3::LPrime,
            Rotation3x3::UPrime,
            Rotation3x3::L,
            Rotation3x3::U,
        ]
    }

    /// An algorithm used in F2L.
    /// Run this algorithm 6 times to end up in the inital state.
    /// And yes, this is an actual cubing algorithm name :)
    pub fn sexy_right_inverted() -> Vec<Rotation3x3> {
        vec![
            Rotation3x3::U,
            Rotation3x3::R,
            Rotation3x3::UPrime,
            Rotation3x3::RPrime,
        ]
    }

    /// An algorithm used in F2L.
    /// Run this algorithm 6 times to end up in the inital state.
    /// And yes, this is an actual cubing algorithm name :)
    pub fn sexy_left_inverted() -> Vec<Rotation3x3> {
        vec![
            Rotation3x3::UPrime,
            Rotation3x3::LPrime,
            Rotation3x3::U,
            Rotation3x3::L,
        ]
    }
}
