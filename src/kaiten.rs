use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_derive::{Deserialize, Serialize};
use tabled::{object::Rows,Disable, Rotate, MaxWidth, Modify, Table, Tabled};
use clap::{Args, Parser, Subcommand};

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
pub struct Author{
    id: u32,
    username: String
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Card {
    id: u32,
    pub title: String,
    pub column: Column_,
    lane: Lane,
    #[tabled(rename="type")]
    r#type: CardType,
    pub sort_order: f32,
    // hiden
    #[tabled(display_with="display_option")]
    members: Option<Vec<Member>>,
    #[tabled(display_with="display_description")]
    description: Option<String>,
    pub archived: bool,
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
pub struct Column_ {
    id: u32,
    pub title: String,
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
