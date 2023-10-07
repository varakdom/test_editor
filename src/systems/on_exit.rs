use bevy::app::AppExit;
use bevy::prelude::EventReader;
// use bevy_egui::EguiContexts;

pub fn on_exit_system(events: EventReader<AppExit>) {
    if !events.is_empty() {
        //@todo save before quitting
    }
}
