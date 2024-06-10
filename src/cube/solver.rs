use super::{cube_state::CubeState, CubeRotationEvent};
use bevy::prelude::*;

pub fn get_solve_sequence(cube_state_query: Query<&CubeState>) -> Vec<CubeRotationEvent> {
    let Ok(cube_state) = cube_state_query.get_single() else {
        error!("expecte exactly 1 CubeState component");
        return vec![];
    };

    todo!()
}
