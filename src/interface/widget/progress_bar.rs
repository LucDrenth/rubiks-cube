use std::time::Duration;

use bevy::prelude::*;

use crate::schedules::CubeScheduleSet;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_progress_bar.in_set(CubeScheduleSet::Timers));
    }
}

#[derive(Component)]
pub struct ProgressBar {
    timer: Option<Timer>,
}

impl ProgressBar {
    pub fn set_timer(&mut self, timer: Timer) {
        self.timer = Some(timer);
    }

    pub fn update_timer(&mut self, time_until_done: f32) {
        let timer = match &mut self.timer {
            Some(timer) => timer,
            None => {
                warn!("tried to update ProgressBar timer while timer is None");
                return;
            }
        };

        let total = timer.elapsed_secs() + time_until_done;
        let mut new_timer = Timer::from_seconds(total, timer.mode());
        new_timer.tick(Duration::from_secs_f32(timer.elapsed_secs()));
        self.timer = Some(new_timer);
    }

    pub fn cancel(&mut self, node: &mut Node) {
        self.timer = None;
        node.width = Val::ZERO;
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self { timer: None }
    }
}

fn handle_progress_bar(
    mut query: Query<(&mut ProgressBar, &mut Node, &mut Visibility)>,
    time: Res<Time>,
) {
    for (mut progress_bar, mut node, mut node_visibility) in query.iter_mut() {
        let timer = match &mut progress_bar.timer {
            Some(timer) => timer,
            None => continue,
        };

        timer.tick(time.delta());

        if timer.finished() {
            progress_bar.timer = None;
            *node_visibility = Visibility::Hidden;
            continue;
        }

        *node_visibility = Visibility::Visible;
        node.width = Val::Percent((timer.fraction() * 100.).min(100.));
    }
}
