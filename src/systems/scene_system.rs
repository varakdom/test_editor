use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::RaycastSource;

use crate::interactions::spawn_entity::spawn_cube;
use crate::systems::mouse_interaction_system::MyRaycastSet;

#[derive(Component)]
pub struct MainCamera;

pub fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    // mut stdmats: ResMut<Assets<StandardMaterial>>,
) {
    // commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());


    
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, -4.0),
    //     ..Default::default()
    // }).with_children(|parent| {
    //     parent.spawn((
    //             // As noted above, we are adding children here but we don't need to add an event
    //             // listener. Events on children will bubble up to the parent!
    //             PbrBundle {
    //                 mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
    //                 material: materials.add(Color::RED.into()),
    //                 transform: Transform::from_xyz(0.0, 2.0, 0.0),
    //                 ..Default::default()
    //             },
    //             PickableBundle::default(),
    //             RaycastPickTarget::default(),
    //         ));
    // });// .insert(RaycastMesh::<MyRaycastSet>::default());
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MainCamera,
        RaycastPickCamera::default(), // <- Enable picking for this camera
        //RaycastSource::<MyRaycastSet>::new_transform_empty(),
        RaycastSource::<MyRaycastSet>::new()
    )); // .insert(RaycastSource::<MyRaycastSet>::new());
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.01, 0.01, 0.01).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.load("horse.vox"),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
    // spawn_cube(&mut commands, &mut meshes, &mut materials, &Vec3 { x: 0.0, y: 0.5, z: 0.0 }, assets);

}