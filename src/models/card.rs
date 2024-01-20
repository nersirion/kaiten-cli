use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{fs::File, process::Command};
use tabled::Tabled;
use tempfile::Builder;
use crate::models::*;
use crate::models::common::{COLUMNS, USERS};

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Card {
    id: u32,
    pub title: String,
    pub column: Column,
    lane: Lane,
    #[tabled(rename = "type")]
    r#type: CardType,
    #[tabled(skip)]
    pub sort_order: f32,
    #[tabled(display_with = "display_members")]
    members: Option<Vec<User>>,
    #[tabled(display_with = "display_tags")]
    tags: Option<Vec<Tag>>,
    #[tabled(skip)]
    description: Option<String>,
    #[tabled(skip)]
    pub archived: bool,
    // #[header(hidden)]
    #[tabled(skip)]
    created: String,
    #[tabled(skip)]
    checklists: Option<Vec<Checklist>>,
}

impl Card {
    fn new() -> Card {
        Card {
            id: 1,
            title: "Title".to_string(),
            column: Column::new(),
            lane: Lane::new() ,
            r#type: CardType::new(),
            tags: None,
            sort_order: 0.0,
            members: None,
            description: Some("".to_string()),
            archived: false,
            created: "Test".to_string(),
            checklists: None,
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
        let checklists: Vec<String> = if self.checklists.is_some() {
            self.checklists
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        } else {
            vec!["".to_string()]
        };
        let checklists_str = format!("## Checklists:\n {}\n", checklists.join("\n"));
        let text = format!(
            "{}{}{}{}{}{}{}{}",
            title, lane, cols, column, card_type, tags, desc, checklists_str
        );
        text
    }

    pub fn from_string() -> String {
        let text = Card::new().to_string();
        let editor = env!("EDITOR");
        let mut tmpfile = Builder::new()
            .suffix(".md")
            .rand_bytes(5)
            .tempfile()
            .unwrap();
        tmpfile.write(text.as_bytes()).expect("Can't write");
        let path = tmpfile.into_temp_path();

        Command::new(editor)
            .arg(&path)
            .status()
            .expect("Something went wrong");

        let mut card_text = String::new();
        File::open(&path)
            .expect("Could not open file")
            .read_to_string(&mut card_text)
            .expect("Could not read");
        let title = Card::get_title(&card_text);
        let tags = Card::get_tags(&card_text);
        "".to_string()
    }

    fn get_title(card_text: &str) -> String {
        let title_idx = "## Title:".len();
        let lane_idx = card_text.find("## Line:").unwrap();
        card_text[title_idx..lane_idx].to_string()
    }

    fn get_lane(card_text: &str) -> Lane {
        let lane_idx = card_text.find("## Lane:").unwrap();
        let column_idx = card_text.find("## Column:").unwrap();
        let lane = &card_text[lane_idx + "## Lane:".len()..column_idx];
        Lane::new()
    }

    fn get_column(card_text: &str) -> Column {
        let column_idx = card_text.find("## Column:").unwrap();
        let card_type_idx = card_text.find("## Type:").unwrap();
        let column = &card_text[column_idx + "## Column:".len()..card_type_idx];
        Column::new()
    }

    fn get_type(card_text: &str) -> CardType {
        let card_type_idx = card_text.find("## Type:").unwrap();
        let checklists_idx = card_text.find("## Checklists:").unwrap();
        let card_type = &card_text[card_type_idx + "## Type:".len()..checklists_idx];
        CardType::new()
    }

    fn get_checklists(card_text: &str) -> Option<Vec<Checklist>> {
        let checklists_idx = card_text.rfind("## Checklists:").unwrap();
        let checklists_strr = &card_text[checklists_idx..];
        let checklists_str: Vec<&str> = checklists_strr.split("###").collect();
        let checklists = match checklists_str.len() {
            1 => None,
            _ => Some(
                checklists_str
                    .into_iter()
                    .map(|chk| Checklist::from_string(format!("###{}", chk)))
                    .collect(),
            ),
        };
        checklists
    }

    fn get_tags(card_text: &str) -> Option<Vec<Tag>> {
        let tags_idx = card_text.rfind("## Tags:").unwrap();
        let desc_idx = card_text.find("## Description:").unwrap();
        let tags_str = &card_text["## Tags:".len() + tags_idx..desc_idx];
        let tags = match tags_str {
            "" => None,
            _ => {
                let tags_str = tags_str.replace(", ", ",");
                let tags: Vec<Tag> = tags_str
                    .split(",")
                    .map(|tag| Tag::from_string(tag.to_string()))
                    .collect();
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
    match o {
        Some(descr) => {
            format!("{}", descr)
        }
        None => format!(""),
    }
}

fn display_members(o: &Option<Vec<User>>) -> String {
    match o {
        Some(members) => {
            let mems: Vec<&str> = members.into_iter().filter(|m| m.r#type.is_some() && m.r#type.unwrap() == 2).map(|m| m.username.as_str()).collect();
            format!("{}", mems.join(",\n"))
        }
        None => format!(""),
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
