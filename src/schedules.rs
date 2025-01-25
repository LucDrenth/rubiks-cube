use bevy::prelude::*;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum CubeStartupSet {
    SpawnCube,
    ApplyScramble,
}

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum CubeScheduleSet {
    InterfaceTimer,
    HandleUserInput,
    HandleEvents,
    UpdateAnimations,
}

pub struct SchedulesPlugin;

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (CubeStartupSet::SpawnCube, CubeStartupSet::ApplyScramble).chain(),
        )
        .configure_sets(
            Update,
            (
                CubeScheduleSet::InterfaceTimer,
                CubeScheduleSet::HandleUserInput,
                CubeScheduleSet::HandleEvents,
                CubeScheduleSet::UpdateAnimations,
            )
                .chain(),
        );
    }
}
