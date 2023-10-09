use bevy_egui::EguiContexts;
use bevy::prelude::*;
use bevy_window::PrimaryWindow;
use bevy_egui::EguiContext;
use egui::{CentralPanel, Frame, Color32};
use egui_dock::*;
use rfd::FileDialog;

use crate::{resources::{ui_state::{ToolBox, UiState}, ui_menu_state::UiMenuState}, serialization::{SaveSceneEvent, LoadSceneEvent}};

use super::scene_system::NewSceneEvent;

#[derive(Component)]
pub struct OpenMenuWindow;

pub fn ui_top_bar_system(
    mut ev_save_scene: EventWriter<SaveSceneEvent>,
    mut ev_load_scene: EventWriter<LoadSceneEvent>,
    mut ev_new_scene: EventWriter<NewSceneEvent>,
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

        if ui.button("Save Project").clicked() {
            if let Some(path) = file_dialog_new() {
                ev_save_scene.send(SaveSceneEvent(path));
            }

        }
        else if ui.button("Load Project").clicked() {
            if let Some(path) = get_project_file() {
                ev_load_scene.send(LoadSceneEvent(path));
            }
        }
        else if ui.button("New Project").clicked() {
            ev_new_scene.send(NewSceneEvent);
        }
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

    let Ok((entity, egui_context)) = world
        .query_filtered::<(Entity, &mut EguiContext), With<OpenMenuWindow>>()
        .get_single(world) else { return };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiMenuState, _>(|world, mut ui_state| {
        ui_state.ui(world, entity, egui_context.get_mut())
    });
}

pub fn get_project_file() -> Option<String>
{
    if let Some(files) = FileDialog::new().add_filter("project", &["scn.ron"]).set_directory("./").pick_file() {
        let path = files.display().to_string();
        let path = path.split("assets\\"); //TODO code for all platform support not for windows only
        let path = path.collect::<Vec<&str>>();

        if path.len() > 1 {
            return Some((*path.last().unwrap()).to_owned());
        }
    }
    return None;
}

pub fn file_dialog_new() -> Option<String>
{
    if let Some(files) = FileDialog::new().set_file_name("New Project").add_filter("project", &["scn.ron"]).set_directory("./").save_file() {
        let path = files.display().to_string();
        let path = path.split("assets\\"); //TODO code for all platform support not for windows only
        let path = path.collect::<Vec<&str>>();

        if path.len() > 1 {
            return Some((*path.last().unwrap()).to_owned());
        }
    }
    return None;
}