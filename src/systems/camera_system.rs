use bevy::prelude::*;
use bevy_render::camera::Viewport;
use bevy_window::PrimaryWindow;
use crate::MainCamera;

use crate::resources::ui_state::UiState;

// make camera only render to view not obstructed by UI
pub fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let cam = cameras.get_single_mut();

    if let Ok(mut cam) = cam {
        let Ok(window) = primary_window.get_single() else { return };

        let scale_factor = window.scale_factor() * egui_settings.scale_factor;
    
        let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
        let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;
    
        if viewport_size.x < 1_f32 || viewport_size.y < 1_f32 { return; }
        cam.viewport = Some(Viewport {
            physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
            physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
            depth: 0.0..1.0,
        });
    }
}