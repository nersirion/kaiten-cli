use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_derive::{Deserialize, Serialize};
use tabled::{object::Rows,Disable, Rotate, MaxWidth, Modify, Table, Tabled, Header};
use clap::{Args, Parser, Subcommand};


const API_URL: &str = "https://rubbles-stories.kaiten.ru/api/latest";

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
        Commands::Card_ ( card ) => {
            format!("{}/cards/{}", API_URL, card.id.as_ref().unwrap_or(&"".to_string()))
        }
        Commands::Columns{} => {format!("{}/boards/96239/columns/", API_URL)}
        Commands::Users{} => {format!("{}/boards/96239/users/", API_URL)}
        Commands::Test{ .. } => {format!("empty")}
        }
    }
}
#[derive(Subcommand)]
enum Commands {
    Card_(Card_) ,
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
struct Card_ {
    id: Option<String>,
    #[clap(long,short)]
    comments: bool
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Comment {
    #[tabled(skip)]
    id: u32,
    text: String,
    #[tabled(skip)]
    created: String,
    author: Author
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Author{
    id: u32,
    username: String
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Card {
    id: u32,
    title: String,
    column: Column_,
    lane: Lane,
    #[tabled(rename="type")]
    r#type: CardType,
    sort_order: f32,
    // hiden
    #[tabled(display_with="display_option")]
    members: Option<Vec<Member>>,
    #[tabled(display_with="display_description")]
    description: Option<String>,
    archived: bool,
    // #[header(hidden)]
    created: String,
}

fn display_description(o: &Option<String>) -> String {
    match o{
    Some(descr) => { format!("{}", descr) }
    None => format!("")
    }
}

fn display_option(o: &Option<Vec<Member>>) -> String {
    match o{
    Some(members) => {let mems: Vec<&str> = members.into_iter().map(|m| m.username.as_str()).collect();
        format!("{}", mems.join(",\n"))
    }
    None => format!("")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Member {
    id: u32,
    username: String
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Column_ {
    id: u32,
    title: String,
}
impl std::fmt::Display for Column_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Lane {
    id: u32,
    title: String,
}
impl std::fmt::Display for Lane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CardType {
    name: String,
    letter: String,
}
impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.letter)
    }
}

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

impl Card_ {
async fn print_cards(&self) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://rubbles-stories.kaiten.ru/api/latest/cards/{}", self.id.as_ref().unwrap_or(&"".to_string()));
    let response = get_data(&client, &url).await?;
    if self.id.is_none(){
    let json_value: Vec<Card> = response.json().await?;
    let mut filter_cards: Vec<&Card> = json_value
        .iter()
        .filter(|card| card.column.title != "Done" && ! card.archived)
        .collect();
    // filter_cards.sort_by(|a, b| ( a.lane.title.cmp(&b.lane.title), b.created.cmp(&a.created)));
    filter_cards.sort_by(|a, b|  a.sort_order.partial_cmp(&b.sort_order).unwrap());
    // filter_cards.sort_by_key(|card| (&card.lane.title, &card.created, ));
    let table = Table::new(filter_cards)
        // .with(Disable::Column(5..6))
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(80)))
        .with(Disable::Column(6..10))
        .to_string();
    println!("{}", table);
    }
    else {
        let json: Card = response.json().await?;
    let table = Table::new(vec![json])
        // .with(Disable::Column(5..6))
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(110).keep_words()))
        .with(Rotate::Left)
        // .with(Modify::new(Row(1..)).with(MaxWidth::wrapping(80)))
        .to_string();
    println!("{}", table);
    let url = format!("https://rubbles-stories.kaiten.ru/api/latest/cards/{}/comments", self.id.as_ref().unwrap());
    let response = get_data(&client, &url).await?;
    let json_value: Vec<Comment> = response.json().await?;
    if self.comments {
    let coms_table = Table::new(json_value)
        .with(Modify::new(Rows::new(1..)).with(MaxWidth::wrapping(80)));
    println!("{}", coms_table);
        }
    }
    Ok(())
}
}

async fn print_columns() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://rubbles-stories.kaiten.ru/api/latest/boards/96239/columns";
    let response = get_data(&client, &url).await?;
    let json_value: Vec<Column_> = response.json().await?;
    let columns_table = Table::new(json_value);
    println!("{}", columns_table);
    Ok(())
}

async fn print_users() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://rubbles-stories.kaiten.ru/api/latest/users";
    let response = get_data(&client, &url).await?;
    let json_value: Vec<Author> = response.json().await?;
    let columns_table = Table::new(json_value);
    println!("{}", columns_table);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let cli = Cli::parse();
     let url = &cli.get_url();
     println!("{}", url);
     match &cli.command {
        Commands::Card_ ( card ) => {
            card.print_cards().await?;
        }
        Commands::Columns{} => {print_columns().await?;}
        Commands::Users{} => {print_users().await?;}
        Commands::Test{ .. } => {println!()}
    
}

    Ok(())
}
