mod systems;
mod resources;
mod interactions;
mod bv_gui;
mod async_systems;
use bevy_vox::VoxPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_mod_picking::prelude::*;
use bevy_egui::EguiSet;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_debug_grid::*;

use crate::resources::ui_state::UiState;
use crate::systems::camera_system::set_camera_viewport;
use crate::systems::gizmo_system::set_gizmo_mode;
use crate::systems::mouse_interaction_system::{mouse_left_click, MyRaycastSet, update_raycast_with_cursor};
use crate::systems::scene_system::{MainCamera, scene_setup};
use crate::systems::ui_system::{show_ui_system, ui_top_bar_system};
use crate::async_systems::catalog_fetch_systems::{apply_gen_tasks, prepare_gen_tasks};
use crate::systems::on_exit::on_exit_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(DefaultInspectorConfigPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_plugin(VoxPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .insert_resource(UiState::new())
        .add_startup_system(scene_setup)
        .add_systems((
            show_ui_system
                .in_base_set(CoreSet::PostUpdate)
                .before(EguiSet::ProcessOutput)
                .before(bevy::transform::TransformSystem::TransformPropagate),
            set_camera_viewport
                .in_base_set(CoreSet::PostUpdate)
                .after(show_ui_system),
            set_gizmo_mode,
            update_raycast_with_cursor,
            mouse_left_click,
            prepare_gen_tasks,
            apply_gen_tasks,
            ui_top_bar_system,
            on_exit_system.in_base_set(CoreSet::PostUpdate)
        ))
        .register_type::<AlphaMode>()
        .run();
}
