use std::path::Path;
use std::fs;
use std::io;
use serde_derive::{Deserialize, Serialize};
use super::common::CONFIG;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    space_id: Option<u32>,
    board_id: Option<u32>,
    exclude_board_ids: Option<String>,
    exclude_lane_ids: Option<String>,
    exclude_column_ids: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            space_id: None,
            board_id: None,
            exclude_board_ids: None,
            exclude_lane_ids: None,
            exclude_column_ids: None,
        }
    }
    pub fn load() -> Result<Option<Self>, io::Error> {
        let file = format!("{}/.config/kaiten-cli/config.yaml", env!["HOME"]);
        let file_path = Path::new(file.as_str());
        if !file_path.exists() {
            return Ok(None)
        }
        let content = fs::read_to_string(file_path)?;

        let info: Self = serde_yaml::from_str(&content).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to deserialize {:?}: {}", file_path, err),
            )
        })?;

        Ok(Some(info))
    }

    pub fn save(&self) -> io::Result<()> {
        let content = serde_yaml::to_string(self).map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize data: {}", err),
            )
        })?;
        let file = format!("{}/.config/kaiten-cli/config.yaml", env!["HOME"]);
        let file_path = Path::new(file.as_str());
        if !file_path.parent().unwrap().exists() {
            fs::create_dir_all(file_path.parent().unwrap())?;
        }
        fs::write(file_path, content)?;
        Ok(())
    }
    pub fn init_global() {
        match Config::load() {
            Ok(Some(config)) => {
                *CONFIG.lock().unwrap() = config
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("Err: {}", err);
            }
        }
    }

    pub fn set_space_id(&mut self, space_id: u32) {
        self.space_id = Some(space_id)
    }
    pub fn set_board_id(&mut self, board_id: u32) {
        self.board_id = Some(board_id)
    }
    pub fn set_exclude_board_ids(&mut self, exclude_board_ids: String) {
        self.exclude_board_ids = Some(exclude_board_ids)
    }
    pub fn set_exclude_column_ids(&mut self, exclude_column_ids: String) {
        self.exclude_column_ids = Some(exclude_column_ids)
    }
    pub fn set_exclude_lane_ids(&mut self, exclude_lane_ids: String) {
        self.exclude_lane_ids = Some(exclude_lane_ids)
    }
}
