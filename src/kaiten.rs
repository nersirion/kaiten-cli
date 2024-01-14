use std::{
    fs::File,
    process::Command,
};
use std::io::{Write, Read};
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use tempfile::Builder;
use clap::{Args, Parser, Subcommand};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLUMNS: Mutex<HashMap<String, Column_>> = Mutex::new(HashMap::new());
    pub static ref USERS: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());
    
}
    // impl COLUMNS {
    //     pub fn get_sort_vec(self) -> Vec<&Column_> {
    //         let columns = self.lock().unwrap().iter();
    //         let mut columns_vec = Vec::from_iter(columns.map(|(_, column)| column));
    //         columns_vec.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
    //         columns_vec
    //     }
    // }

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
    pub id: u32,
    pub username: String
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Option<u32>,
    name: String,
}

impl Tag {
    fn from_string(text: String) -> Self {
        Tag {
            id: None,
            name: text
        }
    }
}

fn display_tags(tags: &Option<Vec<Tag>>) -> String {
    match tags {
        Some(tags) => { 
            let str_tags: Vec<&str> = tags.into_iter().map(|tag| tag.name.as_str()).collect();
            format!("{}", str_tags.join(", "))
        }
        None => format!("")
    }
}

fn display_id(id: &Option<u32>) -> String {
    match id {
        Some(id) => { 
            format!("{}", id)
        }
        None => format!("")
    }
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Card {
    #[tabled(display_with="display_id")]
    id: Option<u32>,
    pub title: String,
    pub column: Column_,
    lane: Lane,
    #[tabled(rename="type")]
    r#type: CardType,
    #[tabled(skip)]
    pub sort_order: f32,
    #[tabled(display_with="display_members")]
    members: Option<Vec<Member>>,
    #[tabled(display_with="display_tags")]
    tags: Option<Vec<Tag>>,
    #[tabled(display_with="display_description")]
    description: Option<String>,
    #[tabled(skip)]
    pub archived: bool,
    // #[header(hidden)]
    #[tabled(skip)]
    created: String,
    #[tabled(skip)]
    checklists: Option<Vec<Checklist>>
}

impl Card {
    fn empty() -> Card {
        Card {
            id: None,
            title: "Title".to_string(),
            column: Column_ {
                id: 0,
                title: "Test".to_string(),
                sort_order: 0.0
            },
            lane: Lane{
                id: 0,
                title: "test".to_string()
        },
            r#type: CardType {
                name: "Test".to_string(),
                letter: "Test".to_string() 
            },
            tags: None,
            sort_order: 0.0,
            members: None,
            description: Some("".to_string()),
            archived: false,
            created: "Test".to_string(),
            checklists: None

        }
    }
    pub fn to_string(&self) -> String {
        let title = format!("# Title: {}\n\n", self.title);
        let lane = format!("## Lane: {}\n", self.lane);
        let columns = COLUMNS.lock().unwrap();
        let cs: Vec<String> = Vec::from_iter(columns.iter().map(|(k, _)| k.to_string()));
        let cols = format!("<!-- {} -->\n", cs.join("|"));
        let column = format!("## Column: {}\n", self.column);
        let card_type = format!("## Type: {}\n", self.r#type);
        let tags = format!("## Tags: {}\n", display_tags(&self.tags));
        let desc = format!("## Description: \n{}\n", self.description.as_ref().unwrap());
        let checklists: Vec<String> = if self.checklists.is_some() {self.checklists.as_ref().unwrap().into_iter().map(|x| x.to_string()).collect()} else {vec!["".to_string()]};
        let checklists_str = format!("## Checklists:\n {}\n", checklists.join("\n"));
        let text = format!("{}{}{}{}{}{}{}{}", title, lane, cols, column, card_type, tags, desc, checklists_str);
        text
    }

    pub fn from_string() -> String {
        let text = Card::empty().to_string();
        let editor = env!("EDITOR");
        let mut tmpfile = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile().unwrap();
        tmpfile.write(text.as_bytes()).expect("Can't write");
        let path = tmpfile.into_temp_path();

        Command::new(editor)
            .arg(&path)
            .status()
            .expect("Something went wrong");

        let mut card_text = String::new();
        File::open(&path)
            .expect("Could not open file")
        .read_to_string(&mut card_text).expect("Could not read");
        let title = Card::get_title(&card_text);
        let tags = Card::get_tags(&card_text);
        "".to_string()
    }

    fn get_title(card_text: &str) -> String {
        let title_idx ="## Title:".len();
        let lane_idx = card_text.find("## Line:").unwrap();
        card_text[title_idx..lane_idx].to_string()

    }

    fn get_lane(card_text: &str) -> Lane {
        let lane_idx = card_text.find("## Lane:").unwrap();
        let column_idx = card_text.find("## Column:").unwrap();
        let lane = &card_text[lane_idx+"## Lane:".len()..column_idx];
        Lane { id: 0, title: lane.to_string() }
    }

    fn get_column(card_text: &str) -> Column_ {
        let column_idx = card_text.find("## Column:").unwrap();
        let card_type_idx = card_text.find("## Type:").unwrap();
        let column = &card_text[column_idx+"## Column:".len()..card_type_idx];
        Column_ { id: 0, title: column.to_string(), sort_order: 0.0 }

    }

    fn get_type(card_text: &str) -> CardType {
        let card_type_idx = card_text.find("## Type:").unwrap();
        let checklists_idx = card_text.find("## Checklists:").unwrap();
        let card_type = &card_text[card_type_idx+"## Type:".len()..checklists_idx];
        CardType { name: card_type.to_string(), letter: "".to_string() }

    }

    fn get_checklists(card_text: &str) -> Option<Vec<Checklist>> {
        let checklists_idx = card_text.rfind("## Checklists:").unwrap();
        let checklists_strr = &card_text[checklists_idx..];
        let checklists_str: Vec<&str> = checklists_strr.split("###").collect();
        let checklists = match checklists_str.len() {
            1 => None,
            _ => {
                Some(checklists_str.into_iter().map(|chk| Checklist::from_string(format!("###{}", chk))).collect())
            }
        };
        checklists
        

    }

    fn get_tags(card_text: &str) -> Option<Vec<Tag>> {
        let tags_idx = card_text.rfind("## Tags:").unwrap();
        let desc_idx = card_text.find("## Description:").unwrap();
        let tags_str = &card_text["## Tags:".len()+tags_idx..desc_idx]; 
        let tags = match tags_str {
            "" => None,
            _ => {

            let tags_str = tags_str.replace(", ", ",");
            let tags: Vec<Tag> = tags_str.split(",").map(|tag| Tag::from_string(tag.to_string())).collect();
            Some(tags)
                }
        };
        tags

    }

    // pub fn from_string(text: String) -> Self {
    //     
    // }
}

fn display_description(o: &Option<String>) -> String {
    match o{
    Some(descr) => { format!("{}", descr) }
    None => format!("")
    }
}

fn display_members(o: &Option<Vec<Member>>) -> String {
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

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Column_ {
    pub id: u32,
    pub title: String,
    #[tabled(skip)]
    pub sort_order: f32,
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


#[derive(Debug, Serialize, Deserialize)]
pub struct Checklist {
    pub id: Option<u32>,
    pub name: String,
    pub items: Option<Vec<ChecklistItem>>
}

impl std::fmt::Display for Checklist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self.items{
        Some(value) => {
        let items: Vec<String> = value.into_iter().map(|x| format!("{}", x)).collect();
        write!(f, "### {}\n\n{}", self.name, items.join("\n"))
                }
        None => {write!(f, "")}
    }
}
}
impl Checklist {
    pub fn to_string(&self) -> String {
        let mut checklist = format!("### {}\n\n", self.name);
        let items_string = if self.items.is_some() {
            let items: Vec<String> = self.items.as_ref().unwrap().into_iter().map(|item| item.to_string()).collect();
            items.join("\n")
        }
        else {
            "".to_string()
        };
        checklist = format!("{}{}", checklist, items_string);
        checklist
    }
    pub fn from_string(text: String) -> Self {
        let lines: Vec<&str> = text.split("\n").filter(|line| line.len() > 3).collect();
        println!("{:?}", lines);
        let name = lines[0][4..].to_string();
        let items = lines[1..].into_iter().map(|item| ChecklistItem::from_string(item.to_string())).collect();
        Self {
            id: None,
            name: name,
            items: Some(items)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: Option<u32>,
    pub text: String,
    pub checked: bool
}

impl std::fmt::Display for ChecklistItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ChecklistItem {
    fn to_string(&self) -> String {
        let check = if self.checked { "[x]"} else {"[ ]"};
        let string_item = format!("{} {}", check, self.text);
        string_item

    }
    pub fn from_string(raw_text: String) -> Self {
        let text = raw_text.trim();
        let text = text.replace("[]", "[ ]");
        let check = match &text[0..3] {
            "[ ]" => false,
            "[x]" => true,
            _ => panic!("Text will be start with [ ] or [x]")
        };
        Self {
            id: None,
            text: if text.len()> 4 {text[4..].to_string()} else {"".to_string()},
            checked: check
        }
    }
}


