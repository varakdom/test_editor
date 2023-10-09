mod async_systems;
mod bv_gui;
mod interactions;
mod resources;
mod serialization;
mod systems;

use bevy::prelude::*;
use bevy_debug_grid::*;
use bevy_egui::EguiSet;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_vox_mesh::VoxMeshPlugin;
use resources::ui_menu_state::UiMenuState;
use serialization::fileless_assets::{load_fileless_assets, FilelessAssets};
use systems::scene_system::{new_scene_system, NewSceneEvent};

use crate::async_systems::api_fetch_systems::{
    apply_asset_api_fetch, apply_asset_download, prepare_asset_api_fetch, prepare_asset_download,
};
use crate::async_systems::catalog_fetch_systems::{apply_image_preview, prepare_image_preview};
use crate::resources::configuration::Configuration;
use crate::resources::ui_state::UiState;
use crate::serialization::*;
use crate::systems::camera_system::*;
use crate::systems::config_file::config_system;
use crate::systems::gizmo_system::set_gizmo_mode;
use crate::systems::mouse_interaction_system::{
    mouse_left_click, update_raycast_with_cursor, MyRaycastSet,
};
use crate::systems::on_exit::on_exit_system;
use crate::systems::scene_system::{scene_setup, spawn_entity, MainCamera};
use crate::systems::ui_system::{show_ui_system, ui_top_bar_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(DefaultInspectorConfigPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_plugin(VoxMeshPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugGridPlugin::with_floor_grid())
        .insert_resource(UiState::new())
        .insert_resource(Configuration::file())
        .add_startup_system(config_system)
        .insert_resource(UiMenuState::new())
        .register_type::<InScene>()
        .add_event::<SaveSceneEvent>()
        .add_event::<LoadSceneEvent>()
        .add_event::<NewSceneEvent>()
        .add_startup_system(load_fileless_assets.in_base_set(StartupSet::PreStartup))
        .add_startup_system(scene_setup)
        .add_systems((save_scene_system, load_scene_system))
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
            ui_top_bar_system,
            prepare_asset_api_fetch,
            apply_asset_api_fetch,
            prepare_image_preview,
            apply_image_preview,
            prepare_asset_download,
            apply_asset_download,
            spawn_entity,
            new_scene_system,
            on_exit_system.in_base_set(CoreSet::PostUpdate),
        ))
        .register_type::<AlphaMode>()
        .run();
}
