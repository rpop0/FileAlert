use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::manager::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData};
use crate::CONFIG_HANDLER;
use crate::input_handler::InputHandler;

pub struct FileAlert {
    file_reader: BufReader<File>,
    audio_manager: AudioManager,
    alert_sound: StaticSoundData,
    alert_strings: Vec<String>,
}

impl FileAlert {

    // What to do next: have a config file and save the previous path in it, load previous path
    // as a suggestion "Please enter a file path (F:\Games\RageMP\clientdata\console.txt)"


    // pub fn new(file_reader: BufReader<File>, alert_sound: StaticSoundData, alert_strings: Vec<String>) -> Self {
    //     let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Unable to create AudioManager.");
    //     Self { file_reader, audio_manager, alert_sound, alert_strings }
    // }

    pub fn new() -> Self{
        let audio_manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Unable to create AudioManager.");

        let file_reader = InputHandler::get_alert_file_reader();

        let alert_sound = InputHandler::get_alert_sound();

        let alert_strings = InputHandler::get_alert_strings();

        CONFIG_HANDLER.write().unwrap().save_config();

        Self { file_reader, audio_manager, alert_sound, alert_strings}
    }

    pub fn play_alert_sound(&mut self) {
        self.audio_manager.play(self.alert_sound.clone()).expect("Unable to play sound.");
    }

    pub fn watch_file(&mut self) {
        let mut line = String::new();
        println!("Watching file for changes...");
        loop {
            thread::sleep(time::Duration::from_millis(100));
            line.clear();
            self.file_reader.read_line(&mut line).expect("Unable to read line from file.");

            if line.is_empty() {
                continue
            }

            line = line.trim().to_lowercase();

            for alert_string in self.alert_strings.iter() {
                if line.contains(alert_string){
                    self.play_alert_sound();
                    break;
                }
            }
        }
    }
}