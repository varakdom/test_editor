use bevy::prelude::*;
use bevy_window::PrimaryWindow;
use egui::Pos2;
use bevy_mod_raycast::{Intersection, RaycastMethod, RaycastSource};

use crate::resources::ui_state::UiState;


#[derive(Reflect, Clone)]
pub struct MyRaycastSet;

/*
pub fn intersection(query: Query<&Intersection<MyRaycastSet>>) {
    for intersection in query.iter() {
        info!(
            "Distance {:?}, Position {:?}",
            intersection.distance(),
            intersection.position()
        );
    }
}
*/

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };
    for mut pick_source in &mut query {
        // println!("vec: {:?}", pick_source.intersections());
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

pub fn mouse_left_click(
    ui_state: Res<UiState>,
    query: Query<&Intersection<MyRaycastSet>>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<Input<MouseButton>>,
) {
    let Ok(window) = primary_window.get_single() else { return };

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            if ui_state.viewport_rect.contains(Pos2 { x: position.x, y: position.y }) {
                for intersection in query.iter() {
                    if let Some(vec) = intersection.position() {
                        info!("Position: {:?}", vec);
                        // spawn_cube(&mut commands, &mut meshes, &mut materials, vec );
                    }
                }
            }
        }
    }
}
