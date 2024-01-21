use super::card::Card;
use super::comment::Comment;
use crate::models::common::{COLUMNS, USERS, INFO};
use crate::models::{User, Info};
use clap::{Parser, Subcommand};
use tabled::{settings::Style, Table};
use super::Init;
use crate::api::ApiClient;

const BOARD_ID: &str = "96239";
const SPACE_ID: u32 = 38223;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[arg(long, global=true)]
    space_id: Option<u32>
}

#[derive(Subcommand)]
pub enum Commands {
    Cards(Card),
    Columns {},
    Users {},
    Tags{},
    Comments(Comment),
    /// Download all info for long-term entity
    Init(Init),
}

impl Cli {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = ApiClient::default();
        let result = match &self.command {
            Commands::Init(init) => {
                init.execute(client).await?
            }
            Commands::Columns{} => {
                Info::init_global();
                let columns = INFO.get().unwrap().get_columns();
                Table::new(columns).with(Style::markdown()).to_string()
            }
            Commands::Tags{} => {
                Info::init_global();
                let tags = INFO.get().unwrap().get_tags();
                Table::new(tags).with(Style::markdown()).to_string()
            }
            Commands::Cards(card) => {
                let api_url = card.get_url();
                let response = client.get_data(&api_url).await?;
                card.get_table(response).await?
            }
            _ => String::new()
        };
        Ok(result)
    }
    pub fn get_url(&self) -> String {
        match &self.command {
            Commands::Cards(card) => card.get_url(),
            Commands::Columns {} => {
                format!("boards/{}/columns/", BOARD_ID)
            }
            Commands::Users {} => {
                format!("spaces/{}/users/", SPACE_ID)
            }
            Commands::Comments(comment) => comment.get_url(),
            _ => String::new()
        }
    }
    pub async fn get_table(
        &self,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let table = match &self.command {
            Commands::Cards(card) => card.get_table(response).await?,
            Commands::Columns {} => {
                // let data= json.iter().map(|c| (c.title.as_str(), c.id)).collect::<HashMap<_,_>>();

                String::new()
                // let columns = COLUMNS.lock().unwrap();
                // let mut columns_vec = Vec::from_iter(columns.iter().map(|(_, column)| column));
                // columns_vec.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                // let table = Table::new(columns_vec).with(Style::markdown()).to_string();
                // table
            }
            Commands::Users {} => {
                let users = USERS.lock().unwrap();
                let users_vec = Vec::from_iter(users.iter().map(|(username, id)| User {
                    username: username.to_string(),
                    id: *id,
                    r#type: None

                }));
                // let users = Vec::from_iter(USERS.lock().unwrap().iter());
                Table::new(users_vec).with(Style::markdown()).to_string()
            }
            Commands::Comments(comment) => comment.get_table(response).await?,
            _ => String::new()
        };
        Ok(table)
    }
}
