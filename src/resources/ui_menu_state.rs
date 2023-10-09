use std::str::Split;

use bevy::prelude::{Entity, Resource, World};
use egui::*;
use egui_extras::RetainedImage;
use rfd::FileDialog;

use crate::{
    serialization::LoadSceneEvent,
    systems::{scene_system::NewSceneEvent, ui_system::get_project_file},
};

#[derive(Resource)]
pub struct UiMenuState {
    open_svg: RetainedImage,
    new_svg: RetainedImage,
}

const MENU_COLOR: Color32 = Color32::from_rgb(27, 27, 27);

impl UiMenuState {
    pub fn new() -> Self {
        Self {
            open_svg: RetainedImage::from_image_bytes(
                "open_folder.png",
                include_bytes!("../../assets/open_folder.png"),
            )
            .unwrap(),
            new_svg: RetainedImage::from_image_bytes(
                "new_project.png",
                include_bytes!("../../assets/new_project.png"),
            )
            .unwrap(),
        }
    }

    pub(crate) fn ui(&mut self, world: &mut World, window_entity: Entity, ctx: &mut Context) {
        CentralPanel::default()
            .frame(
                Frame::central_panel(&ctx.style())
                    .inner_margin(0.)
                    .fill(MENU_COLOR),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Welcome to Better Voxel Editor").size(32.));

                    ui.horizontal_centered(|ui| {
                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                if egui::ImageButton::new(
                                    self.new_svg.texture_id(ctx),
                                    egui::vec2(40.0, 40.0),
                                )
                                .ui(ui)
                                .clicked()
                                {
                                    world.send_event::<NewSceneEvent>(NewSceneEvent);
                                    world.despawn(window_entity);
                                }

                                ui.label("New Project");
                            });
                        });

                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                if egui::ImageButton::new(
                                    self.open_svg.texture_id(ctx),
                                    egui::vec2(40.0, 40.0),
                                )
                                .ui(ui)
                                .clicked()
                                {
                                    if let Some(path) = get_project_file() {
                                        world.send_event::<LoadSceneEvent>(LoadSceneEvent(path));
                                        world.despawn(window_entity);
                                    }
                                }

                                ui.label("Open Project");
                            });
                        });
                    });
                });
            });
    }
}
