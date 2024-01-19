use crate::models::User;
use crate::models::common::{USERS, COLUMNS};
use clap::{Parser, Subcommand};
use tabled::{settings::Style, Table };
use super::card::Card;
use super::comment::Comment;

const BOARD_ID: &str = "96239";
const SPACE_ID: u32 = 38223;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Cards(Card),
    Columns {},
    Users {},
    Comments(Comment)
}

impl Cli {
    pub fn get_url(&self) -> String {
        match &self.command {
            Commands::Cards (card) => {
                card.get_url()
            }
            Commands::Columns {} => {
                format!("boards/{}/columns/", BOARD_ID)
            }
            Commands::Users {} => {
                format!("spaces/{}/users/", SPACE_ID)
            }
            Commands::Comments(comment) => {
                comment.get_url()
            }
        }
    }
    pub async fn get_table(
        &self,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn std::error::Error>> {
    
        let table = match &self.command {
            Commands::Cards ( card ) => {
                card.get_table(response).await?
            }
            Commands::Columns {} => {
                // let data= json.iter().map(|c| (c.title.as_str(), c.id)).collect::<HashMap<_,_>>();

                let columns = COLUMNS.lock().unwrap();
                let mut columns_vec = Vec::from_iter(columns.iter().map(|(_, column)| column));
                columns_vec.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                let table = Table::new(columns_vec).with(Style::markdown()).to_string();
                table

            }
            Commands::Users {} => {
                let users = USERS.lock().unwrap();
                let users_vec = Vec::from_iter(users.iter().map(|(username, id)| User{username: username.to_string(), id: *id}));
                // let users = Vec::from_iter(USERS.lock().unwrap().iter());
                Table::new(users_vec).with(Style::markdown()).to_string()
            }
            Commands::Comments(comment) => {
                comment.get_table(response).await?
            }
        };
        Ok(table)
    }
}
