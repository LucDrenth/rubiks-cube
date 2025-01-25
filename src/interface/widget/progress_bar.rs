use bevy::prelude::*;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_progress_bar);
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

fn handle_progress_bar(mut query: Query<(&mut ProgressBar, &mut Node)>, time: Res<Time>) {
    for (mut progress_bar, mut node) in query.iter_mut() {
        let timer = match &mut progress_bar.timer {
            Some(timer) => timer,
            None => continue,
        };

        timer.tick(time.delta());

        if timer.finished() {
            progress_bar.timer = None;
            node.width = Val::ZERO;
            continue;
        }

        node.width = Val::Percent(timer.fraction() * 100.);
    }
}
