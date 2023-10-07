use bevy_egui::EguiContexts;
use bevy::prelude::*;
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;

use crate::resources::ui_state::{ToolBox, UiState};

pub fn ui_top_bar_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
) {
    let ctx = contexts.ctx_mut();

    let mut context = ctx.clone();

    egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui_state.top_bar.draw_sections(ui);
        });

    egui::TopBottomPanel::top("second_top_panel_file").resizable(false).show(ctx, |ui| {
        let mut t = ToolBox { gizmo_mode: ui_state.toolbox.gizmo_mode, project_path: ui_state.toolbox.project_path.clone() };

        ui_state.top_bar.draw_drawers(ui, &mut t, &mut context);

        ui_state.toolbox = t;
    });
}

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world) else { return };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}