use crate::models::{User, Card as ModelsCard, Comment as ModelsComment};
use crate::models::common::{USERS, COLUMNS};
use clap::{Args, Parser, Subcommand};
use tabled::{settings::{object::Rows, Width, Modify, Style}, Table };
use super::card::{Card, CardCommands};
use super::comment::{Comment, CommentCommands};

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
    
        let info = match &self.command {
            Commands::Cards ( card ) => {
                match &card.command {
                    CardCommands::Get { card_id: _ } => {
                        let json: ModelsCard = response.json().await?;
                        // card.get_table(vec![json])
                        json.to_string()
                    }
                    CardCommands::Ls (arg) => {
                        let json: Vec<ModelsCard> = response.json().await?;
                        card.get_table(json)
                    }

                    CardCommands::New{} => {
                        ModelsCard::from_string()
                    }

                    CardCommands::Mv { card_id: _ } => {
                        let mut json: ModelsCard = response.json().await?;
                        let columns = COLUMNS.lock().unwrap();
                        let mut columns_vec = Vec::from_iter(columns.iter().map(|(_, column)| column));
                        columns_vec.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                        let idx = columns_vec.iter().position(|&x| x.title == json.column.title).unwrap();
                        if idx < columns_vec.len() {
                            json.column = columns_vec[idx+1].clone();
                        }
                        println!("{:?}", json);
                        // let m = M{column_id: columns_vec[idx+1].id};
                        // let res = patch_data(&client, url.as_str(), m).await?;
                        // println!("{:?}", res);
                        String::from("")

                    }
                    _ => String::from(""),
                }
            }
            Commands::Columns {} => {
                // let data= json.iter().map(|c| (c.title.as_str(), c.id)).collect::<HashMap<_,_>>();

                let columns = COLUMNS.lock().unwrap();
                let mut columns_vec = Vec::from_iter(columns.iter().map(|(_, column)| column));
                columns_vec.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                let table = Table::new(columns_vec);
                table.to_string()

            }
            Commands::Users {} => {
                let users = USERS.lock().unwrap();
                let users_vec = Vec::from_iter(users.iter().map(|(username, id)| User{username: username.to_string(), id: *id}));
                // let users = Vec::from_iter(USERS.lock().unwrap().iter());
                Table::new(users_vec).to_string()
            }
            Commands::Comments(comment) => {
                comment.get_table(response).await?
            }
            _ => String::new()
        };
        Ok(info)
    }
}
