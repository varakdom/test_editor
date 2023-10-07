pub mod top_bar;
mod gizmo_ui;
mod bevy_ressource_ui;
mod asset_ui;
mod project_resources_ui;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::{hierarchy_ui, SelectedEntities};
use std::any::TypeId;
use bevy_asset::HandleId;
use egui::{Context, Widget};
use bevy_inspector_egui::bevy_inspector::{
    self, ui_for_entities_shared_components, ui_for_entity_with_children,
};

use crate::bv_gui::gizmo_ui::draw_gizmo;
use crate::resources::ui_state::{AssetInfo, Catalog, ToolBox};

#[derive(Debug)]
pub enum EguiWindow {
    GameView,
    Hierarchy,
    Resources,
    // Assets,
    Console,
    Catalog,
    Animation,
    Inspector,
}

#[derive(Eq, PartialEq)]
pub enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, HandleId),
}

pub struct TabViewer<'a> {
    pub world: &'a mut World,
    pub selected_entities: &'a mut SelectedEntities,
    pub selection: &'a mut InspectorSelection,
    pub viewport_rect: &'a mut egui::Rect,
    pub catalog: &'a mut Catalog,
    pub ctx: Context,
    pub toolbox: &'a mut ToolBox
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::GameView => {
                *self.viewport_rect = ui.clip_rect();

                draw_gizmo(ui, self.world, self.selected_entities, self.toolbox.gizmo_mode);
            }
            EguiWindow::Hierarchy => {
                let selected = hierarchy_ui(self.world, ui, self.selected_entities);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Console => {},
            EguiWindow::Animation => {},
            EguiWindow::Catalog => {
                if ui.add(egui::Button::new("Refresh")).clicked() {
                    //@todo fetch image information from Better Voxel API
                    let tmp_fetched = vec![
                        "https://i.ytimg.com/an/lcphuaOVGeE/12453724661110169088_mq.jpg?v=5f69b6e1",
                        "https://mir-s3-cdn-cf.behance.net/project_modules/max_1200/4ba579120661385.60b63d726651d.png",
                        "https://cdn.80.lv/api/upload/content/14/images/62b1a6e5a0ff8/widen_920x0.jpeg",
                        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRjqhRetAZQ_oF3IWx8HqRBk1JKtfMcnI48TQ&usqp=CAU",
                    ];
                    for uri in tmp_fetched {
                        self.catalog.queue.push_back(AssetInfo { url: uri.to_string() });
                        self.catalog.nr_fetch += 1;
                    }
                    // fetch image
                    //@todo fetch API
                    // ui_state.catalog.get_image(&"https://s3.johanhelsing.studio/dump/favicon.png".to_string());
                }

                ui.horizontal(|ui| {
                    for img in &self.catalog.cache {
                        //@todo import asset to scene
                        let _btn = egui::ImageButton::new(
                            img.texture_id(&self.ctx),
                            egui::Vec2{x: 200.0, y: 200.0},
                            // img.size_vec2()
                        ).ui(ui);
                    }

                    for _ in 0..self.catalog.nr_fetch {
                        ui.spinner();
                    }
                });
            },
            EguiWindow::Resources => {},//select_resource(ui, &type_registry, self.selection),
            // EguiWindow::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    )
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}