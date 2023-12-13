use std::fs;
use std::path::{Path, PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

const APP_DIR: &str = "FileAlert";
const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
   pub data: Data
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub file_to_watch: String,
    pub sound_file: String,
    pub alert_strings: Vec<String>
}

pub struct ConfigHandler{
    config_file_path: PathBuf,
    pub config: Config
}

impl ConfigHandler {

    pub fn save_config(&self) {
        let toml_config = toml::to_string(&self.config).expect("Unable to create default config.");
        fs::write(&self.config_file_path, toml_config).expect("Unable to write default config.");
    }

    pub fn new() -> Self {
        let config_local_dir = dirs::config_local_dir().expect("Unable to find local configuration directory.");
        let app_config_dir = config_local_dir.join(APP_DIR);


        // Always try to create the folder.
        if !Path::new(&app_config_dir).exists() {
            fs::create_dir_all(&app_config_dir).expect("Unable to create config directory.");
        }


        let config_file_path = app_config_dir.join(CONFIG_FILE);


        // If the config file itself does not exit in the path, try to create an empty one.
        if !Path::new(&config_file_path).exists() {
            let empty_config = Config { data: Data {
                file_to_watch: "".to_string(),
                sound_file: "".to_string(),
                alert_strings: vec![],
                }
            };
            let toml_config = toml::to_string(&empty_config).expect("Unable to create default config.");
            fs::write(&config_file_path, toml_config).expect("Unable to write default config.");
            return Self {config_file_path, config: empty_config};
        }

        // Config file exists, load it
        let config_file_contents = fs::read_to_string(&config_file_path).expect("Unable to read config file.");
        let config = toml::from_str(&config_file_contents).expect("Unable to deserialize config file contents.");


        Self {config_file_path, config}
    }

}