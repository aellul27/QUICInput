use crate::config;
use std::fs::{read, write, File};
use std::io::Write;
use std::path;
use toml::Value;

pub fn load_config(config_file: &String) -> config::QUICInputConfig {
    if path::Path::new(config_file).is_file() {
        let data = read(config_file).unwrap_or_else(|err| {
            panic!("Failed to read config file '{}': {}", config_file, err);
        });
        let config = toml::from_slice::<config::QUICInputConfig>(&data).unwrap_or_else(|err| {
            panic!("Failed to parse config file '{}': {}", config_file, err);
        });
        config.validate().unwrap_or_else(|err| {
            panic!("Invalid configuration in '{}': {}", config_file, err);
        });
        let serialized_config = toml::to_string_pretty(&config)
            .expect("Failed to serialize configuration");

        let normalized_existing = toml::from_slice::<Value>(&data)
            .ok()
            .and_then(|existing| toml::to_string_pretty(&existing).ok());

        let needs_update = normalized_existing
            .as_ref()
            .map(|normalized| normalized != &serialized_config)
            .unwrap_or(true);

        if needs_update {
            println!("UPdating config file:");
            if let Err(err) = write(config_file, &serialized_config) {
                println!(
                    "Failed to update configuration file '{}': {}",
                    config_file, err
                );
            }
        }

        config
    } else {
        println!("The file '{}' does not exist. Creating one now", config_file);

        let default = config::QUICInputConfig::default();

        let data = toml::to_string_pretty(&default)
            .expect("Failed to serialize default configuration");

        let mut file = match File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_file)
        {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to create file '{}', using default. {}", config_file, err);
                return default;
            }
        };

        if let Err(err) = file.write_all(data.as_bytes()) {
            println!(
                "Failed to write default configuration to file '{}': {}",
                config_file, err
            );
        }

        default
    }
}