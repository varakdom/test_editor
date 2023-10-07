use std::collections::VecDeque;

use bevy::app::AppLabel;
use bevy::prelude::{World, Resource};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use egui::Context;
use egui_dock::{NodeIndex, Tree};
use egui_extras::RetainedImage;
use egui_gizmo::GizmoMode;

use crate::bv_gui::{EguiWindow, TabViewer};
use crate::bv_gui::top_bar::TopBar;
use crate::bv_gui::top_bar::transformer_drawer::TransformDrawer;
use crate::bv_gui::InspectorSelection;
use crate::bv_gui::top_bar::project_drawer::ProjectDrawer;

pub struct Catalog {
    pub queue: VecDeque<AssetInfo>,
    pub cache: Vec<RetainedImage>,
    pub nr_fetch: i32,
}

pub struct AssetInfo {
    pub url: String,
    //image: Option<RetainedImage>
}

impl Catalog {
    fn new() -> Self {
        Catalog { queue: VecDeque::new(), cache: vec![], nr_fetch: 0 }
    }
}

pub struct ToolBox {
    pub gizmo_mode: GizmoMode,
    pub project_path: String,
}

impl ToolBox {
    pub fn new() -> Self {
        Self {
            gizmo_mode: GizmoMode::Translate,
            project_path: "/home/gameboyadvance/Programs/Epitech/EIP/bv_test_project".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct UiState {
    tree: Tree<EguiWindow>,
    pub top_bar: TopBar,
    pub viewport_rect: egui::Rect,
    pub selected_entities: SelectedEntities,
    selection: InspectorSelection,
    pub catalog: Catalog,
    pub toolbox: ToolBox
}

impl UiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![EguiWindow::GameView]);
        let [game, _bottom] =
            tree.split_below(NodeIndex::root(), 0.8, vec![EguiWindow::Resources, /*EguiWindow::Assets, */EguiWindow::Console, EguiWindow::Catalog, EguiWindow::Animation]);
        let [game, _inspector] =
            tree.split_right(game, 0.75, vec![EguiWindow::Inspector]);
        let [_game, _hierarchy] =
            tree.split_left(game, 0.2, vec![EguiWindow::Hierarchy]);

        let mut top_bar = TopBar::new();
        // Always add drawers first !!
        top_bar.register_drawer("TransformDrawer".as_str(), TransformDrawer::new());
        top_bar.register_drawer("ProjectDrawer".as_str(), ProjectDrawer::new());

        top_bar.add_section("Project".to_string(), vec!["ProjectDrawer".to_string()]);
        top_bar.add_section("Transform".to_string(), vec!["TransformDrawer".to_string(), "ProjectDrawer".to_string()]);
        top_bar.add_section("Help".to_string(), vec![]);
        top_bar.add_section("Test".to_string(), vec!["TransformDrawer".to_string()]);

        Self {
            tree,
            top_bar,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
            catalog: Catalog::new(),
            //@todo get path when creating new project
            toolbox: ToolBox::new(),
        }
    }

    pub(crate) fn ui(&mut self, world: &mut World, ctx: &mut Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
            catalog: &mut self.catalog,
            ctx: ctx.clone(),
            toolbox: &mut self.toolbox
        };
        egui_dock::DockArea::new(&mut self.tree)
            .style(egui_dock::Style {
                ..egui_dock::Style::from_egui(ctx.style().as_ref())
            })
            .show(ctx, &mut tab_viewer);
    }
}