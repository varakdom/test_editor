use std::ffi::OsString;
use std::fs;

use egui::{Ui, Context};

use crate::bv_gui::top_bar::ShowDrawer;
use crate::resources::ui_state::ToolBox;

pub struct ProjectDrawer {
    files: Vec<OsString>,
}

impl ProjectDrawer {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }
}

impl ShowDrawer for ProjectDrawer {
    fn draw(&mut self, ui: &mut Ui, toolbox: &mut ToolBox, _ctx: &mut Context) {
        ui.horizontal(|ui| {
            if ui.button("Scan directory").clicked() {
                println!("project path: {}", toolbox.project_path);
                scan_dir(&toolbox.project_path, &mut self.files);
                // scan dirs
                for f in &self.files {
                    println!("{}", f.clone().into_string().unwrap());
                }
            }
        });
    }
}

fn scan_dir(path: &String, files: &mut Vec<OsString>) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        files.push(path.unwrap().file_name());
        // println!("Name: {}", path.unwrap().path().display())
    }
}