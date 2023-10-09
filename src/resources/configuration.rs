// Extract information from configuration file.
use std::fs;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

use crate::interactions::keybinds::{KeyBinds, KeyActionDuo};
use crate::systems::config_file::default_binds;

// The main configuration skeleton, in whose form the JSON will be read.
#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct Configuration {
    pub key_binds_vec: Vec<KeyActionDuo>,  // Vector, size is variable
}

impl Configuration {
    // Create the Configuration object from nothing
    fn fileless() -> Configuration {
        Configuration {
            key_binds_vec: KeyBinds::new(default_binds()).keybinds_to_keyactionduo()
        }
    }
    
    // Create the Configuration object from file
    pub fn file() -> Configuration {
    
        let json_string = fs::read_to_string("./BVconfig.json");
    
        match json_string {
            Ok(json_string) => {
    
                let _config_attempt: Result<Configuration, serde_json::Error> = serde_json::from_str(&json_string);
                match _config_attempt {
                    Ok(configuration) => { return configuration },
                    Err(_) => { println!("Corrupted config file, using default values."); },
                }
    
            },
            Err(_) => { println!("Error reading from file path"); },
        }
        return Configuration::fileless();
    }

    // Set the content of configuration into the file
    pub fn save(&self) {
        let str = serde_json::to_string::<Configuration>(self);

        match str {
            Ok(str) => {
                fs::write("./BVconfig.json", str).expect("Unable to write file");
            },
            Err(_) => { println!("[!]: Failed saving Config file!"); },
        }
    }
}
