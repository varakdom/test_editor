use egui::{Ui, Context};
use webbrowser;

use crate::bv_gui::top_bar::ShowDrawer;
use crate::resources::ui_state::ToolBox;

pub struct HelpDrawer {
    wiki: (),
    support: (),
}

impl HelpDrawer {
    pub fn new() -> Self {
        Self {
            wiki: (),
            support: (),
        }
    }
}

impl ShowDrawer for HelpDrawer {
    fn draw(&mut self, ui: &mut Ui, toolbox: &mut ToolBox, _ctx: &mut Context) {
        ui.horizontal(|ui| {
            if ui.button("Wiki").clicked() {
                if let Err(err) = webbrowser::open("https://www.wikipedia.org/") {
                    eprintln!("Failed to open Wiki: {}", err);
                }
            }
            if ui.button("Support").clicked() {
                if let Err(err) = webbrowser::open("https://www.youtube.com/watch?v=eBEHiZITg4o") {
                    eprintln!("Failed to open Support: {}", err);
                }
            }
        });
    }
}