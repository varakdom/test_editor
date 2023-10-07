use egui::{Context, Ui, Widget};
use egui_extras::RetainedImage;
use egui_gizmo::GizmoMode;

use crate::bv_gui::top_bar::ShowDrawer;
use crate::resources::ui_state::ToolBox;

pub struct TransformDrawer {
    transform_svg: RetainedImage,
    rotate_svg: RetainedImage,
    scale_svg: RetainedImage,
    activated: [bool;3],
}

impl TransformDrawer {
    pub fn new() -> Self {
        Self {
            transform_svg: RetainedImage::from_svg_bytes_with_size(
                "transform.svg",
                include_bytes!("../../../assets/transform.svg"),
                egui_extras::image::FitTo::Original,
            ).unwrap(),
            rotate_svg: RetainedImage::from_svg_bytes_with_size(
                "rotate.svg",
                include_bytes!("../../../assets/rotate.svg"),
                egui_extras::image::FitTo::Original,
            ).unwrap(),
            scale_svg: RetainedImage::from_svg_bytes_with_size(
                "scale.svg",
                include_bytes!("../../../assets/scale.svg"),
                egui_extras::image::FitTo::Original,
            ).unwrap(),
            activated: [false, true, true]
        }
    }

    fn deactivate(&mut self, idx: usize) {
        if self.activated.len() <= idx { return; }
        for elem in self.activated.iter_mut() { *elem = true; }
        self.activated[idx] = false;
    }
}

impl ShowDrawer for TransformDrawer {
    fn draw(&mut self, ui: &mut Ui, toolbox: &mut ToolBox, ctx: &mut Context) {
        //@todo do this in a loop more intelligently
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_enabled(self.activated[0]);
                    if egui::ImageButton::new(
                        self.transform_svg.texture_id(ctx),
                        egui::vec2(40.0, 40.0)
                        // self.svg_image.size_vec2(),
                    ).ui(ui).clicked() {
                        toolbox.gizmo_mode = GizmoMode::Translate;
                        self.deactivate(0);
                    }

                    ui.label("Translate");
                });
            });

            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_enabled(self.activated[1]);
                    if egui::ImageButton::new(
                        self.rotate_svg.texture_id(ctx),
                        egui::vec2(40.0, 40.0)
                        // self.svg_image.size_vec2(),
                    ).ui(ui).clicked() {
                        toolbox.gizmo_mode = GizmoMode::Rotate;
                        self.deactivate(1);
                    }

                    ui.label("Rotate");
                });
            });

            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_enabled(self.activated[2]);
                    if egui::ImageButton::new(
                        self.scale_svg.texture_id(ctx),
                        egui::vec2(40.0, 40.0)
                        // self.svg_image.size_vec2(),
                    ).ui(ui).clicked() {
                        toolbox.gizmo_mode = GizmoMode::Scale;
                        self.deactivate(2);
                    }

                    ui.label("Scale");
                });
            });

        });


    }
}