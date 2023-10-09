use bevy::prelude::*;
use bevy_asset::{Asset, HandleId};
use bevy_reflect::{TypeUuid};

#[derive(Resource)]
pub struct FilelessAssets
{
    pub default_material: Handle<StandardMaterial>,
    pub cube_mesh: Handle<Mesh>
}

/// Create a HandleId::AssetsPath with a T type based on the given string
/// Ussed to have persitant HandleId between run and save file for code based assets
fn v_id<T: TypeUuid>(id: u64) -> HandleId
{
    HandleId::new(T::TYPE_UUID, id)
}

pub fn load_fileless_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
)

{
    commands.insert_resource(
        FilelessAssets
        {
            default_material: materials.set(v_id::<StandardMaterial>(100), Color::WHITE.into()),
            cube_mesh: meshes.set(v_id::<Mesh>(101), Mesh::from(shape::Cube { size: 1.0 })),
        }
    )
}