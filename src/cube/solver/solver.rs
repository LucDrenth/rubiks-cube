use bevy::log;

use crate::cube::{CubeRotationEvent, CubeState};

use super::kociemba::get_solve_sequence_with_kociemba;

#[allow(dead_code)]
pub enum SolveStrategy {
    /// Finds a solution with a number of moves close to an optimal solution.
    /// This algorithm is very fast, and can be run within 1 game tick (1/60s) on most modern computers.
    Kociemba,
    /// Find a solution with the least moves possible.
    /// This algorithm is slow and is not expected to be run within 1 game tick (1/60s).
    GodsAlgorithm,
}

pub fn get_solve_sequence(
    strategy: SolveStrategy,
    cube_state: &CubeState,
) -> Vec<CubeRotationEvent> {
    match strategy {
        SolveStrategy::Kociemba => return get_solve_sequence_with_kociemba(cube_state),
        SolveStrategy::GodsAlgorithm => return get_solve_sequence_with_gods_algorithm(cube_state),
    }
}

pub fn get_solve_sequence_with_gods_algorithm(_cube_state: &CubeState) -> Vec<CubeRotationEvent> {
    log::info!("TODO implement");
    return vec![];
}
