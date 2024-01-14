use std::collections::HashMap;
use crate::kaiten::*;
use clap::{Args, Parser, Subcommand, lazy_static::lazy_static};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tabled::{object::Rows, MaxWidth, Modify, Table};

const API_URL: &str = "https://rubbles-stories.kaiten.ru/api/latest";

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct M {
    column_id: u32
}

async fn get_data(
    client: &reqwest::Client,
    url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", env!("KT")))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?;
    Ok(response)
}
async fn patch_data(
    client: &reqwest::Client,
    url: &str,
    data: M
) -> Result<reqwest::Response, reqwest::Error> {
    let response = client
        .patch(url)
        .json(&data)
        .header(AUTHORIZATION, format!("Bearer {}", env!("KT")))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await?;
    Ok(response)
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {


    async fn init(&self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/boards/96239/columns/", API_URL);
        let response = get_data(&client, url.as_str()).await?;
        let json: Vec<Column_> = response.json().await?;
        for d in json.iter() {
            let title = d.title.as_str();
            let column = Column_{title: d.title.to_string(), id: d.id, sort_order: d.sort_order};
            COLUMNS.lock().unwrap().insert(title.to_string(), column);
        };
        let url = format!("{}/users/", API_URL);
        let response = get_data(&client, url.as_str()).await?;
        let json: Vec<Author> = response.json().await?;
        for d in json.iter() {
            let author = d.username.as_str();
            USERS.lock().unwrap().insert(author.to_string(), d.id);
        }
        Ok(())


    }
    pub fn get_url(&self) -> String {
        match &self.command {
            Commands::Card { options } => match options {
                CardOptions::Ls{} | CardOptions::New {} => {
                    format!("{}/cards", API_URL,)
                }
                CardOptions::Get { card_id }
                | CardOptions::Edit { card_id }
                | CardOptions::Mv { card_id } => {
                    format!("{}/cards/{}", API_URL, card_id)
                }
            },
            Commands::Columns {} => {
                format!("{}/boards/96239/columns/", API_URL)
            }
            Commands::Users {} => {
                format!("{}/users/", API_URL)
            }
        }
    }

    pub async fn get_table(
        &self,
        client: reqwest::Client,
    ) -> Result<String, Box<dyn std::error::Error>> {
    
    
        let url = self.get_url();
        self.init(&client).await?;
        let info = match &self.command {
            Commands::Card { options } => {
                let response = get_data(&client, url.as_str()).await?;
                match options {
                    CardOptions::Get { card_id: _ } => {
                        let json: Card = response.json().await?;
                        // card.get_table(vec![json])
                        json.to_string()
                    }
                    CardOptions::Ls {} => {
                        let json: Vec<Card> = response.json().await?;
                        options.get_table(json).to_string()
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
                        let m = M{column_id: columns_vec[idx+1].id};
                        let res = patch_data(&client, url.as_str(), m).await?;
                        println!("{:?}", res);
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
                let users_vec = Vec::from_iter(users.iter().map(|(username, id)| Author{username: username.to_string(), id: *id}));
                // let users = Vec::from_iter(USERS.lock().unwrap().iter());
                Table::new(users_vec).to_string()
            }
        };
        Ok(info)
    }
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

#[derive(Subcommand)]
enum CardOptions {
    /// print all cards for user
    Ls {},
    /// get card info
    Get { card_id: String },
    /// edit card
    Edit { card_id: String },
    /// create new card
    New {},
    /// move card to next column
    Mv { card_id: String },
}

#[derive(Args)]
pub struct Card_ {
    #[clap(long, short)]
    columns: Option<String>,
}
impl CardOptions {
    pub fn get_table(&self, json: Vec<Card>) -> Table {
        match self {
            CardOptions::Ls{} => {
                let mut filter_cards: Vec<&Card> = json
                    .iter()
                    .filter(|card| card.column.title != "Done" && !card.archived)
                    .collect();
                filter_cards.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                Table::new(filter_cards)
                    .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(70)))
                    // .with(Disable::Column(6..10))
            }
            _ => Table::new(vec![""]),
        }
    }
}
