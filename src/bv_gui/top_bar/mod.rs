pub mod transformer_drawer;
pub mod project_drawer;

use std::collections::HashMap;
use egui::{Context, Ui, Vec2};
use crate::resources::ui_state::ToolBox;

pub struct Section {
    pub display_name: String,
    pub drawers: Vec<String>,
}

pub trait ShowDrawer {
    fn draw(&mut self, ui: &mut Ui, toolbox: &mut ToolBox, ctx: &mut Context);
}

pub struct TopBar {
    pub sections: Vec<Section>,
    active_section_idx: usize,
    drawers: HashMap<String, Box<dyn ShowDrawer + Send + Sync>>,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            active_section_idx: 0,
            drawers: HashMap::new(),
        }
    }

    pub fn register_drawer<T: ShowDrawer + Send + Sync + 'static>(&mut self, name: &str, drawer: T) /*-> &mut Self*/ {
        if self.drawers.contains_key(name) {
            panic!("The drawer already exists");
        }
        self.drawers.insert(name.to_string(), Box::new(drawer));
        // self
    }

    pub fn add_section(&mut self, display_name: String, drawers: Vec<String>) /*-> &mut Self*/ {
        for name in &drawers {
            if !self.drawers.contains_key(name) {
                panic!("Drawer {} doesn't exist", name);
            }
        }
        self.sections.push(Section{ display_name, drawers });
        // self
    }

    pub fn draw_sections(&mut self, ui: &mut Ui) {
        let button_width = 50.0;
        let button_size = Vec2::new(button_width, ui.spacing().interact_size.y);

        ui.horizontal(|ui| {
            for (idx, section) in self.sections.iter().enumerate() {
                if ui.add(egui::Button::new(&section.display_name).min_size(button_size)).clicked() {
                    self.active_section_idx = idx;
                }
            }
        });
    }

    pub fn draw_drawers(&mut self, ui: &mut Ui, toolbox: &mut ToolBox, ctx: &mut Context) {
        if let Some(section) = self.sections.get(self.active_section_idx) {
            ui.horizontal(|ui| {
                for drawer_key in &section.drawers {
                    let drawer = self.drawers.get_mut(drawer_key).unwrap();
                    drawer.draw(ui, toolbox, ctx);
                    ui.add(egui::Separator::default().vertical());
                }
            });
        }
    }
}

// struct ActivationList<T, S> {
//     vec: Vec<T>,
//     state: S,
//     pos: i32
// }
//
// trait OnDrawerActivated<T> {
//     fn on_activated(&mut self);
// }

// impl<T, S> ActivationList<T, S> {
//     pub fn new<T, S>(vec: Vec<T>, state: S, pos: i32) -> Self {
//         Self {
//             vec,
//             state,
//             pos
//         }
//     }
// }
//
// impl OnDrawerActivated<T> for ActivationList<T, S> {
//     fn on_activated(&mut self) {
//
//    }
// }