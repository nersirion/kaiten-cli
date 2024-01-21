use super::card::Card;
use super::comment::Comment;
use crate::models::common::INFO;
use crate::models::{User, Info, Config as ModelsConfig};
use clap::{Parser, Subcommand};
use tabled::{settings::Style, Table};
use super::{Init, Config};
use crate::api::ApiClient;

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
    Config(Config),
}

impl Cli {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        ModelsConfig::init_global();
        let client = ApiClient::default();
        let result = match &self.command {
            Commands::Init(init) => {
                init.execute(client).await?
            }
            Commands::Config(config) => {
                config.execute().await?
            }
            Commands::Columns{} => {
                Info::init_global();
                let columns = INFO.get().unwrap().get_columns();
                Table::new(columns).with(Style::markdown()).to_string()
            }
            Commands::Users{} => {
                Info::init_global();
                let users = INFO.get().unwrap().get_users();
                Table::new(users).with(Style::markdown()).to_string()
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
}
