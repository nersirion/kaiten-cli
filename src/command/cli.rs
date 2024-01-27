use super::card::Card;
use super::comment::Comment;
use super::Link;
use super::{Config, Init};
use crate::api::ApiClient;
use crate::models::common::{CONFIG, INFO};
use crate::models::{Config as ModelsConfig, Info, User};
use clap::{Parser, Subcommand};
use tabled::{settings::Style, Table};
use tabled::settings::{object::{Columns as Cols}, Width, measurement::Percent};

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
    Lanes {},
    Spaces {},
    Boards {},
    Comments(Comment),
    Links(Link),
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
        let client = ApiClient::default()?;
        let result = match &self.command {
            Commands::Init(init) => init.execute(client).await?,
            Commands::Config(config) => config.execute().await?,
            Commands::Columns {} => {
                Info::init_global();
                let config = CONFIG.lock().unwrap();
                let columns = INFO.get().unwrap().get_columns(config.get_board_id());
                Table::new(columns).modify(Cols::last(), Width::wrap(80).keep_words()).with(Style::modern()).to_string()
            }
            Commands::Users {} => {
                Info::init_global();
                let config = CONFIG.lock().unwrap();
                let users = INFO.get().unwrap().get_users(config.get_space_id());
                Table::new(users).with(Style::modern()).to_string()
            }
            Commands::Tags {} => {
                Info::init_global();
                let tags = INFO.get().unwrap().get_tags();
                Table::new(tags).with(Style::modern()).to_string()
            }
            Commands::Lanes {} => {
                Info::init_global();
                let config = CONFIG.lock().unwrap();
                let lanes = INFO.get().unwrap().get_lanes(config.get_board_id());
                Table::new(lanes).with(Style::modern()).to_string()
            }
            Commands::Spaces {} => {
                Info::init_global();
                let spaces = INFO.get().unwrap().get_spaces();
                Table::new(spaces).with(Style::modern()).to_string()
            }
            Commands::Boards {} => {
                Info::init_global();
                let boards = INFO.get().unwrap().get_boards();
                Table::new(boards).with(Style::modern()).to_string()
            }
            Commands::Cards(card) => {
                Info::init_global();
                card.get_table(client).await?
            }
            Commands::Comments(comment) => {
                Info::init_global();
                comment.get_table(client).await?
            }
            Commands::Links(link) => {
                Info::init_global();
                link.get_table(client).await?
            }
            _ => String::new(),
        };
        Ok(result)
    }
}
