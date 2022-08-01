use crate::kaiten::*;
use clap::{Args, Parser, Subcommand};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tabled::{object::Rows, MaxWidth, Modify, Table};

const API_URL: &str = "https://rubbles-stories.kaiten.ru/api/latest";

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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
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
        let response = get_data(&client, url.as_str()).await?;
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
                        options.get_table(json).to_string()
                    }

                    CardOptions::New{} => {
                        Card::from_string()
                    }
                    _ => String::from(""),
                }
            }
            Commands::Columns {} => {
                let json: Vec<Column_> = response.json().await?;
                let table = Table::new(json);
                table.to_string()

            }
            Commands::Users {} => {
                let json: Vec<Author> = response.json().await?;
                Table::new(json).to_string()
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
