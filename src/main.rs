mod file_alert;
mod input_handler;
mod config_handler;


use std::sync::RwLock;
use lazy_static::lazy_static;
use file_alert::FileAlert;
use config_handler::ConfigHandler;

lazy_static! {
    static ref CONFIG_HANDLER: RwLock<ConfigHandler> = RwLock::new(
        ConfigHandler::new()
    );
}

fn main() {
    let mut file_alert = FileAlert::new();
    file_alert.watch_file();
}
