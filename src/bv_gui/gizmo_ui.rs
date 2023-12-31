use bevy::prelude::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_render::camera::CameraProjection;
use bevy_render::prelude::Projection;
use egui_gizmo::GizmoMode;

use crate::MainCamera;

pub fn draw_gizmo(
    ui: &mut egui::Ui,
    world: &mut World,
    selected_entities: &SelectedEntities,
    gizmo_mode: GizmoMode,
) {
    let camera = world.query_filtered::<(&GlobalTransform, &Projection), With<MainCamera>>().get_single(world);

    if let Ok(camera) = camera {
        let (cam_transform, projection) = camera;
        let view_matrix = Mat4::from(cam_transform.affine().inverse());
        let projection_matrix = projection.get_projection_matrix();
    
        if selected_entities.len() != 1 {
            return;
        }
    
        for selected in selected_entities.iter() {
            let Some(transform) = world.get::<Transform>(selected) else { continue };
            let model_matrix = transform.compute_matrix();
    
            let Some(result) = egui_gizmo::Gizmo::new(selected)
                .model_matrix(model_matrix.to_cols_array_2d())
                .view_matrix(view_matrix.to_cols_array_2d())
                .projection_matrix(projection_matrix.to_cols_array_2d())
                .orientation(egui_gizmo::GizmoOrientation::Local)
                .mode(gizmo_mode)
                .interact(ui)
                else { continue };
    
            let mut transform = world.get_mut::<Transform>(selected).unwrap();
            *transform = Transform {
                translation: Vec3::from(<[f32; 3]>::from(result.translation)),
                rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
                scale: Vec3::from(<[f32; 3]>::from(result.scale)),
            };
        }
    }
}