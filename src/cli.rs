use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_derive::{Deserialize, Serialize};
use tabled::{object::Rows,Disable, Rotate, MaxWidth, Modify, Table, Tabled};
use clap::{Args, Parser, Subcommand};
use crate::kaiten::*;


const API_URL: &str = "https://rubbles-stories.kaiten.ru/api/latest";

async fn get_data(client: &reqwest::Client, url: &str) ->  Result<reqwest::Response, reqwest::Error>{
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
        Commands::Card ( card ) => {
            format!("{}/cards/{}", API_URL, card.id.as_ref().unwrap_or(&"".to_string()))
        }
        Commands::Columns{} => {format!("{}/boards/96239/columns/", API_URL)}
        Commands::Users{} => {format!("{}/users/", API_URL)}
        Commands::Test{ .. } => {format!("empty")}
        }
    }

    pub async fn get_table(&self, client: reqwest::Client) ->Result<Table, Box<dyn std::error::Error>>{
        let url = self.get_url();
        let response = get_data(&client, url.as_str()).await?;
        let table = match &self.command {
        Commands::Card ( card ) => {
            if card.id.is_some() {
                let json: Card = response.json().await?;
                 card.get_table(vec![json])
            }
            else {
                let json: Vec<Card> = response.json().await?;
                 card.get_table(json)
            }
        }
        Commands::Columns{} => {
            let json: Vec<Column_> = response.json().await?;
            Table::new(json)
        }
        Commands::Users{} => {
            let json: Vec<Author> = response.json().await?;
            Table::new(json)
        }
        Commands::Test{ .. } => {Table::new(vec![format!("hi")])}
        };
        Ok(table)
    }
}
#[derive(Subcommand)]
pub enum Commands {
    Card(Card_) ,
    Columns{},
    Users{},
    Test {          
          #[clap(subcommand)]
          cmd: Cmd2      
      },                                                                                                                                                                                                                 
}                                                                  
  #[derive(Subcommand)]                                              
  enum Cmd2 {                                                                                                                        
    Ls{},
    Get{}
}


#[derive(Args)]
pub struct Card_ {
    pub id: Option<String>,
    #[clap(long,short)]
    comments: bool
}
impl Card_ {
    pub fn get_table(&self, json: Vec<Card>) -> Table {

    if self.id.is_none(){
    let mut filter_cards: Vec<&Card> = json
        .iter()
        .filter(|card| card.column.title != "Done" && ! card.archived)
        .collect();
    filter_cards.sort_by(|a, b|  a.sort_order.partial_cmp(&b.sort_order).unwrap());
    Table::new(filter_cards)
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(80)))
        .with(Disable::Column(6..10))
    }
    else{
    Table::new(json)
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(110).keep_words()))
        .with(Rotate::Left)
        }
    }
}
