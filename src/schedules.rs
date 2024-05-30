use bevy::prelude::*;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum CubeScheduleSet {
    HandleUserInput,
    HandleEvents,
    UpdateAnimations,
}

pub struct SchedulesPlugin;

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                CubeScheduleSet::HandleUserInput,
                CubeScheduleSet::HandleEvents,
                CubeScheduleSet::UpdateAnimations,
            )
                .chain(),
        );
    }
}
