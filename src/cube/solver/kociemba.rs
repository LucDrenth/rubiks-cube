use bevy::log;

use crate::cube::{CubeRotationEvent, CubeState};

/// This algorithm consist of two phases.
///
/// In phase 1 we reduce the cube state down to a state that can be solved by using only
/// the following move set: U, D, R2, F2, L2, B2.
/// The orientation of edge and corner pieces can not change in this state, regardless of
/// order or number of moves.
/// We do this by running an algorithm a couple of times and stop once we find a short enough
/// set of moves to reach this sate.
pub fn get_solve_sequence_with_kociemba(_cube_state: &CubeState) -> Vec<CubeRotationEvent> {
    log::info!("TODO implement");
    return vec![];
}
