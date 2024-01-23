use std::collections::HashMap;
use crate::api::ApiClient;
use crate::models::Info;
use clap::Args;
use crate::models::{User, Board, Space, CardType, Tag};
use std::path::Path;
use crate::models::common::INFO;


#[derive(Debug)]
struct FileExistsError{
    file_name: String
}
impl FileExistsError {
    fn new(file_name: String) -> Self {
        Self { file_name }
    }
}

impl std::error::Error for FileExistsError{} 
impl std::fmt::Display for FileExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {} with entities info already exists. Use `kaiten-cli init --show` to view or `kaiten-cli init --update` for download data from API", self.file_name)
    }
}

#[derive(Args)]
pub struct Init {
    /// Show current entities info from cache file
    #[arg(long)]
    pub show: bool,
    /// Update entities info cache file from API
    #[arg(short, long)]
    pub update: bool,
}

impl Init {
    pub async fn execute(&self, client: ApiClient) -> Result<String, Box<dyn std::error::Error>> {
        if self.show {
            Info::init_global();
            let result = serde_yaml::to_string(INFO.get().unwrap())?;
            Ok(result)
        } else {
            if !self.update {
                let file = format!("{}/.config/kaiten-cli/entities.yaml", env!["HOME"]);
                let file_path = Path::new(file.as_str());
                if file_path.exists() {
                    let error = FileExistsError::new(file);
                    return Err(Box::new(error))
                }

            }
            let api_url = "spaces";
            let mut spaces_vec: Vec<Space> = client.get_data(api_url).await?.json().await?;
            let mut boards: HashMap<u32, Board> = HashMap::new();
            for space in spaces_vec.iter_mut() {
                let api_url = format!("spaces/{}/users", space.get_id());
                let users: Vec<User> = client.get_data(&api_url).await?.json().await?;
                for board_id in space.get_boards_ids().into_iter() {
                    let api_url = format!("spaces/{}/boards/{}", space.get_id(), board_id );
                    let board: Board = client.get_data(&api_url).await?.json().await?;
                    boards.insert(board.get_id(), board);
                }
                space.set_users(users);
            }
            let spaces: HashMap<u32, Space> = spaces_vec.into_iter().map(|s| (s.get_id(), s)).collect();
            let api_url = "tags";
            let tags: Vec<Tag> = client.get_data(api_url).await?.json().await?;
            let api_url = "card-types";
            let card_types: Vec<CardType> = client.get_data(api_url).await?.json().await?;
            let info = Info::from(spaces, boards, tags, card_types);
            let _ = info.save()?;
            let info_string = serde_yaml::to_string(&info)?;
            Ok(info_string)
        }
    }
}
