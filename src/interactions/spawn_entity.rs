use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::RaycastMesh;
use crate::systems::mouse_interaction_system::MyRaycastSet;


pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: &Vec3,
    assets: Res<AssetServer>,

)
{
    println!("test: {:?}", pos);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        },
        // PickableBundle::default(),    // <- Makes the mesh pickable.
        RaycastPickTarget::default(), // <- Needed for the raycast backend.
        RaycastMesh::<MyRaycastSet>::default(), // Make this mesh ray cast-able
        OnPointer::<Click>::target_commands_mut(|click, target_commands| {
            println!("click: {:?}", click);
            if /*click.target != click.listener && */click.button == PointerButton::Secondary {
            target_commands.despawn();
            println!("despawning entity...");
            }
            println!("not going in if statement.");
        }),
    )).insert(RaycastMesh::<MyRaycastSet>::default());

    // commands.spawn(PbrBundle {
    //     transform: Transform::from_xyz(0.0, 10.0, 0.0).with_scale(Vec3::from_array([0.1, 0.1, 0.1])),
    //     mesh: assets.load("chicken.vox"),
    //     material: materials.add(Color::rgb(1., 1., 1.).into()),
    //     ..Default::default()
    // });
}