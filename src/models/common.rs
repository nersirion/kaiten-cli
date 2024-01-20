use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use super::{Column, Space, Tag, CardType};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use once_cell::sync::OnceCell;
use std::env;

lazy_static! {
    pub static ref COLUMNS: Mutex<HashMap<String, Column>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());
    pub static ref INFO: OnceCell<Info> = OnceCell::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    spaces: Vec<Space>,
    tags: Vec<Tag>,
    card_types: Vec<CardType>
}

impl Info {
    pub fn from(spaces: Vec<Space>, tags: Vec<Tag>, card_types: Vec<CardType>) -> Self {
        Self {
            spaces,
            tags,
            card_types
        }


    }
    pub fn load() -> Result<Self, io::Error> {
        let file = format!("{}/.config/kaiten-cli/entities.yaml", env!["HOME"]);
        let file_path = Path::new(file.as_str());
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File {:?} not exist. Please run `kaiten-cli init`", file_path),
            ));
        }
        let content = fs::read_to_string(file_path)?;

        let info: Self = serde_yaml::from_str(&content).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to deserialize {:?}: {}", file_path, err),
            )
        })?;

        Ok(info)

    }

    pub fn save(&self) -> io::Result<()> {
        let content = serde_yaml::to_string(self).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize data: {}", err),
            )
        })?;
        let file = format!("{}/.config/kaiten-cli/entities.yaml", env!["HOME"]);
        let file_path = Path::new(file.as_str());
        if !file_path.parent().unwrap().exists() {
            fs::create_dir_all(file_path.parent().unwrap())?;
        }
        fs::write(file_path, content)?;
        Ok(())
    }
    pub fn init_global() {
        match Info::load() {
            Ok(info) => {
                INFO.set(info).expect("Failed to initialize global Info");
            }
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }
}

