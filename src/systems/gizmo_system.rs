use bevy::prelude::{KeyCode, Res, Input, ResMut};
use egui_gizmo::GizmoMode;

use crate::resources::ui_state::UiState;

pub fn set_gizmo_mode(input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    for (key, mode) in [
        (KeyCode::R, GizmoMode::Rotate),
        (KeyCode::T, GizmoMode::Translate),
        (KeyCode::S, GizmoMode::Scale),
    ] {
        if input.just_pressed(key) {
            ui_state.toolbox.gizmo_mode = mode;
        }
    }
}