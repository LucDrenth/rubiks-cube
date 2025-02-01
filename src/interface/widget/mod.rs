pub mod button;
pub mod dropdown;
pub mod progress_bar;

pub struct WidgetPlugin;

impl bevy::prelude::Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            button::ButtonPlugin,
            dropdown::DropdownPlugin,
            progress_bar::ProgressBarPlugin,
        ));
    }
}
