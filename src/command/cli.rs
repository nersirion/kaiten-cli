use super::card::Card;
use super::comment::Comment;
use super::{Config, Init};
use crate::api::ApiClient;
use crate::models::common::{CONFIG, INFO};
use crate::models::{Config as ModelsConfig, Info, User};
use clap::{Parser, Subcommand};
use tabled::{settings::Style, Table};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[arg(long, short, global = true)]
    space_id: Option<u32>,
    #[arg(long, short, global = true)]
    board_id: Option<u32>,
}

#[derive(Subcommand)]
pub enum Commands {
    Cards(Card),
    Columns {},
    Users {},
    Tags {},
    Comments(Comment),
    /// Download all info for long-term entity
    Init(Init),
    Config(Config),
}

impl Cli {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        ModelsConfig::init_global();
        {
            let mut config = CONFIG.lock().unwrap();
            config.update(self.space_id, self.board_id);
        }
        let client = ApiClient::default();
        let result = match &self.command {
            Commands::Init(init) => init.execute(client).await?,
            Commands::Config(config) => config.execute().await?,
            Commands::Columns {} => {
                Info::init_global();
                let config = CONFIG.lock().unwrap();
                let columns = INFO.get().unwrap().get_columns(config.get_board_id());
                Table::new(columns).with(Style::markdown()).to_string()
            }
            Commands::Users {} => {
                Info::init_global();
                let config = CONFIG.lock().unwrap();
                let users = INFO.get().unwrap().get_users(config.get_space_id());
                Table::new(users).with(Style::markdown()).to_string()
            }
            Commands::Tags {} => {
                Info::init_global();
                let tags = INFO.get().unwrap().get_tags();
                Table::new(tags).with(Style::markdown()).to_string()
            }
            Commands::Cards(card) => {
                let api_url = card.get_url();
                println!("{}", api_url);
                let response = client.get_data(&api_url).await?;
                card.get_table(response).await?
            }
            _ => String::new(),
        };
        Ok(result)
    }
}
