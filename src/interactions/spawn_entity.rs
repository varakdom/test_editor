use crate::{
    serialization::{fileless_assets::FilelessAssets, InScene},
    systems::mouse_interaction_system::MyRaycastSet,
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::RaycastMesh;

pub fn spawn_cube(commands: &mut Commands, common_assets: &Res<FilelessAssets>, pos: &Vec3) {
    commands
        .spawn((
            PbrBundle {
                mesh: common_assets.cube_mesh.clone(),
                material: common_assets.default_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..Default::default()
            },
            InScene,
            // PickableBundle::default(),    // <- Makes the mesh pickable.
            RaycastPickTarget::default(), // <- Needed for the raycast backend.
            RaycastMesh::<MyRaycastSet>::default(), // Make this mesh ray cast-able
            OnPointer::<Click>::target_commands_mut(|click, target_commands| {
                println!("click: {:?}", click);
                if
                /*click.target != click.listener && */
                click.button == PointerButton::Secondary {
                    target_commands.despawn();
                    println!("despawning entity...");
                }
                println!("not going in if statement.");
            }),
        ))
        .insert(RaycastMesh::<MyRaycastSet>::default());
}
