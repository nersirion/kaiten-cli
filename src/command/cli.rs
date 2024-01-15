use crate::models::*;
use crate::models::common::{USERS, COLUMNS};
use clap::{Args, Parser, Subcommand};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tabled::{settings::{object::Rows, Width, Modify, Style}, Table };
use super::card::CardOptions;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Card {
        #[clap(subcommand)]
        options: CardOptions,
    },
    Columns {},
    Users {},
}

impl Cli {
    pub async fn get_table(
        &self,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn std::error::Error>> {
    
        let info = match &self.command {
            Commands::Card { options } => {
                match options {
                    CardOptions::Get { card_id: _ } => {
                        let json: Card = response.json().await?;
                        // card.get_table(vec![json])
                        json.to_string()
                    }
                    CardOptions::Ls {} => {
                        let json: Vec<Card> = response.json().await?;
                        options.get_table(json)
                    }

                    CardOptions::New{} => {
                        Card::from_string()
                    }

                    CardOptions::Mv { card_id: _ } => {
                        let mut json: Card = response.json().await?;
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
        };
        Ok(info)
    }
}
