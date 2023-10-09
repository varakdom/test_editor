use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

pub mod fileless_assets;

pub struct SaveSceneEvent(pub String);
pub struct LoadSceneEvent(pub String);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InScene;

impl FromWorld for InScene
{
    fn from_world(world: &mut World) -> Self {
        InScene
    }
}

pub fn save_scene_system
(
    world: &mut World,
) {
    
    let mut filter = world.query_filtered::<Entity, With<InScene>>();
    let mut builder: DynamicSceneBuilder = DynamicSceneBuilder::from_world(&world);
 
    builder.extract_entities(filter.iter(world));

    let scene = builder.build();

    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap().clone();

    if let Some(res) = world.get_resource_mut::<Events<SaveSceneEvent>>() {
        let mut reader = res.get_reader();
        let mut ev_path: Option<String> = None;

        for evt in reader.iter(&res) {
            ev_path = Some(evt.0.clone());
        }
        if let Some(path) = ev_path {

            #[cfg(not(target_arch = "wasm32"))]
            IoTaskPool::get()
                .spawn(async move {
                    info!("Saving world...");

                    File::create("./assets/".to_owned() + &path)
                        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                        .expect("Error while writing scene to file");
                })
                .detach();
        }
    } else { return };
}

pub fn load_scene_system
(
    mut ev_load_scene: EventReader<LoadSceneEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q: Query<Entity, With<InScene>>
) {
    for evt in ev_load_scene.iter() {
        q.for_each(|entity| commands.entity(entity).despawn());

        commands.spawn((
            DynamicSceneBundle {
                scene: asset_server.load(evt.0.to_owned()),
                ..default()
            },
            InScene
        ));
    }
}