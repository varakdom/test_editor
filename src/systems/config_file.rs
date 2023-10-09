// Extract information from configuration file.
use bevy::prelude::*;

use crate::interactions::keybinds::{KeyBinds, KeyBind};
use crate::resources::configuration::Configuration;

// Insert here the default bindings for your actions
pub fn default_binds() -> Vec<KeyBind> {
    let mut default_binds: Vec<KeyBind> = Vec::new();
    default_binds.push(KeyBind { action: "CameraMoveForward".to_string(), key_code: KeyCode::Z });
    default_binds.push(KeyBind { action: "CameraMoveBackward".to_string(), key_code: KeyCode::S });
    default_binds.push(KeyBind { action: "CameraMoveLeft".to_string(), key_code: KeyCode::Q });
    default_binds.push(KeyBind { action: "CameraMoveRight".to_string(), key_code: KeyCode::D });
    default_binds.push(KeyBind { action: "CameraMoveUp".to_string(), key_code: KeyCode::C });
    default_binds.push(KeyBind { action: "CameraMoveDown".to_string(), key_code: KeyCode::X });
    default_binds
}

// Operations done inside the bevy app, with existing Configuration
pub fn config_system(
    world: &mut World,
    ) {
        let config: &Configuration = world.resource::<Configuration>();

        let default_binds: Vec<KeyBind> = default_binds();

        world.insert_resource(KeyBinds::new_with_config(default_binds, &config.key_binds_vec));
}
