use super::{Board, CardType, Column, Config, Space, Tag, User};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    pub static ref INFO: OnceCell<Info> = OnceCell::new();
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    spaces: HashMap<u32, Space>,
    boards: HashMap<u32, Board>,
    tags: Vec<Tag>,
    card_types: Vec<CardType>,
}

impl Info {
    pub fn from(
        spaces: HashMap<u32, Space>,
        boards: HashMap<u32, Board>,
        tags: Vec<Tag>,
        card_types: Vec<CardType>,
    ) -> Self {
        Self {
            spaces,
            boards,
            tags,
            card_types,
        }
    }
    pub fn load() -> Result<Self, io::Error> {
        let file = format!("{}/.config/kaiten-cli/entities.yaml", env!["HOME"]);
        let file_path = Path::new(file.as_str());
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "File {:?} not exist. Please run `kaiten-cli init`",
                    file_path
                ),
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
    pub fn get_columns(&self, board_id: Option<u32>) -> Vec<Column> {
        let mut columns = Vec::new();
        if let Some(board_id) = board_id {
            if let Some(board) = self.boards.get(&board_id) {
                columns.extend(board.get_columns())
            }
        } else {
            for board in self.boards.values() {
                columns.extend(board.get_columns())
            }
        }
        columns.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
        columns
    }

    pub fn get_tags(&self) -> &Vec<Tag> {
        &self.tags
    }
    pub fn get_card_types(&self) -> &Vec<CardType> {
        &self.card_types
    }
    pub fn get_users(&self, space_id: Option<u32>) -> Vec<User> {
        let mut users: Vec<User> = Vec::new();
        if let Some(space_id) = space_id {
            if let Some(space) = self.spaces.get(&space_id) {
                users.extend(space.get_users())
            }
        } else {
            for space in self.spaces.values() {
                users.extend(space.get_users())
            }
        }
        users
    }
}
