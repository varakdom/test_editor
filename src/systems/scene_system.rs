use std::f32::consts::PI;

use crate::interactions::spawn_entity::spawn_cube;
use crate::serialization::fileless_assets::FilelessAssets;
use crate::serialization::InScene;
use crate::systems::mouse_interaction_system::MyRaycastSet;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_raycast::RaycastSource;
use bevy_render::camera::RenderTarget;
use bevy_render::view::RenderLayers;
use bevy_window::{WindowRef, WindowResolution};

use super::ui_system::OpenMenuWindow;

pub struct NewSceneEvent;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct SpawnObject {
    pub(crate) name: String,
}

pub fn spawn_entity(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut query: Query<(Entity, &SpawnObject)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.for_each_mut(|(entity, object)| {
        commands.spawn((
            PbrBundle {
                transform: Transform::from_scale((0.01, 0.01, 0.01).into())
                    * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
                mesh: assets.load(&object.name),
                material: materials.add(Color::rgb(1., 1., 1.).into()),

                ..Default::default()
            },
            Name::new(object.name.clone()),
        ));
        commands.entity(entity).remove::<SpawnObject>();
        // commands.
    })
}

pub fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, -4.0),
            ..Default::default()
        },))
        .with_children(|parent| {
            parent.spawn((
                // As noted above, we are adding children here but we don't need to add an event
                // listener. Events on children will bubble up to the parent!
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..Default::default()
                },
                PickableBundle::default(),
                RaycastPickTarget::default(),
            ));
        }); // .insert(RaycastMesh::<MyRaycastSet>::default());

    let second_window = commands
        .spawn((
            Window {
                title: "Open Project".to_owned(),
                resolution: WindowResolution::new(500., 500.),
                ..Default::default()
            },
            OpenMenuWindow,
        ))
        .id();
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MainCamera,
        RaycastPickCamera::default(), // <- Enable picking for this camera
        //RaycastSource::<MyRaycastSet>::new_transform_empty(),
        RaycastSource::<MyRaycastSet>::new(),
    )); // .insert(RaycastSource::<MyRaycastSet>::new());

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Window(WindowRef::Entity(second_window)),
                ..Default::default()
            },
            ..Default::default()
        },
        RenderLayers::from_layers(&[1, 1]),
    ));
}

pub fn new_scene_system(
    mut commands: Commands,
    common_assets: Res<FilelessAssets>,
    ev_new_scene: EventReader<NewSceneEvent>,
    q: Query<Entity, With<InScene>>,
) {
    if ev_new_scene.is_empty() {
        return;
    }
    q.for_each(|entity| commands.entity(entity).despawn());

    spawn_cube(
        &mut commands,
        &common_assets,
        &Vec3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
    );
}
