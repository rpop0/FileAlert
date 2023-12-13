use std::fs::{File};
use std::io::{BufReader, Seek, SeekFrom};
use kira::sound::FromFileError;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use crate::CONFIG_HANDLER;

pub struct InputHandler {

}

impl InputHandler {

    fn get_user_input(prompt_message: &str) -> std::io::Result<String>{
        let mut input = String::new();
        println!("{}", prompt_message);
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn get_reader() -> std::io::Result<BufReader<File>>{
        let read_config = CONFIG_HANDLER.read().unwrap();

        let prompt = match read_config.config.data.file_to_watch.is_empty(){
            true => "Please enter a file path: ".to_string(),
            false => format!("Please enter a file path ({}): ", CONFIG_HANDLER.read().unwrap().config.data.file_to_watch)
        };

        let mut file_path = InputHandler::get_user_input(&prompt)?;

        match file_path.is_empty() {
            true if !read_config.config.data.file_to_watch.is_empty() => {
                file_path = read_config.config.data.file_to_watch.clone();
            }
            false => {
                // Release the read lock to allow writing.
                drop(read_config);
                let mut write_config = CONFIG_HANDLER.write().unwrap();
                write_config.config.data.file_to_watch = file_path.clone();
                // Write lock released.
            }
            _ => {}
        };

        let file = File::open(file_path.trim_end())?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0))?;
        Ok(reader)
    }

    fn get_sound_data() -> Result<StaticSoundData, FromFileError> {
        let read_config = CONFIG_HANDLER.read().unwrap();
        let prompt = match read_config.config.data.sound_file.is_empty(){
            true => "Path to sound file: ".to_string(),
            false => format!("Path to sound file ({}): ", read_config.config.data.sound_file)
        };
        let mut  file_path = InputHandler::get_user_input(&prompt)?;

        match file_path.is_empty() {
            true if !read_config.config.data.sound_file.is_empty() => {
                file_path = read_config.config.data.sound_file.clone();
            }
            false => {
                // Release the read lock to allow writing.
                drop(read_config);
                let mut write_config = CONFIG_HANDLER.write().unwrap();
                write_config.config.data.sound_file = file_path.clone();
                // Write lock released.
            }
            _ => {}
        };

        Ok(StaticSoundData::from_file(file_path, StaticSoundSettings::default())?)
    }


    pub fn get_alert_file_reader() -> BufReader<File> {
        let file_reader = loop {
            match InputHandler::get_reader() {
                Ok(reader) => break reader,
                Err(_) => {},
            }
        };

        return file_reader;
    }

    pub fn get_alert_sound() -> StaticSoundData {
        let alert_sound = loop {
            match InputHandler::get_sound_data() {
                Ok(sound_data) => break sound_data,
                Err(_) => {},
            }
        };
        return alert_sound;
    }


    pub fn get_alert_strings() -> Vec<String> {
        let read_config = CONFIG_HANDLER.read().unwrap();
        let mut prompt = match read_config.config.data.alert_strings.is_empty(){
            true => "Enter some text to be alerted by: ".to_string(),
            false => format!("Enter some text to be alerted by {:?}: ", read_config.config.data.alert_strings)
        };

        let mut alert_strings = Vec::new();
        loop {
            let Ok(input) = InputHandler::get_user_input(&prompt) else {
                continue;
            };
            if input.is_empty() && alert_strings.is_empty() {
                if !read_config.config.data.alert_strings.is_empty(){
                    return read_config.config.data.alert_strings.clone();
                }
                continue;
            }
            if input.is_empty() && !alert_strings.is_empty() {
                break;
            }

            alert_strings.push(input.to_lowercase());
            prompt = "Enter another alert text or press enter to continue: ".to_string();
        };

        println!("{:?}", alert_strings);
        drop(read_config); // Release the read lock.
        let mut write_config = CONFIG_HANDLER.write().unwrap();
        write_config.config.data.alert_strings = alert_strings.clone();
        // Write lock released.

        return alert_strings;
    }

}