use chrono::prelude::*;
use clap::Parser;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs::File, process::Command};
use tabled::{
    builder::Builder as TableBuilder,
    col, row,
    settings::{
        object::{Columns, Rows},
        Alignment, Concat, Disable, Format, Merge, Panel, Settings, Span, Style, Width,
    },
    tables::PoolTable,
    Table, Tabled,
};

use crate::models::*;
use tempfile::Builder;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Card {
    id: u32,
    pub title: String,
    pub column: Column,
    #[tabled(skip)]
    column_id: u32,
    #[tabled(skip)]
    board_id: u32,
    #[tabled(skip)]
    lane_id: u32,
    #[tabled(skip)]
    lane: Lane,
    #[tabled(skip)]
    blocked: bool,
    #[tabled(skip)]
    blocking_card: Option<bool>,
    #[tabled(skip)]
    #[serde(default = "String::new")]
    block_reason: String,
    #[tabled(skip)]
    #[serde(skip_serializing)]
    properties: Option<HashMap<String, PropertiesValue>>,
    #[tabled(rename = "type")]
    r#type: CardType,
    #[tabled(skip)]
    pub sort_order: f32,
    #[tabled(display_with = "display_members")]
    #[serde(skip_serializing)]
    members: Option<Vec<User>>,
    #[tabled(display_with = "display_tags")]
    #[serde(skip_serializing)]
    tags: Option<Vec<Tag>>,
    #[tabled(skip)]
    description: Option<String>,
    #[tabled(skip)]
    pub archived: bool,
    #[tabled(skip)]
    created: String,
    // #[tabled(skip)]
    #[tabled(display_with = "Self::display_move_diff", rename = "moved")]
    last_moved_at: String,
    #[tabled(skip)]
    #[serde(skip_serializing)]
    checklists: Option<Vec<Checklist>>,
    #[serde(skip_serializing)]
    #[tabled(skip)]
    parents: Option<Vec<RelatedCard>>,
    #[serde(skip_serializing)]
    #[tabled(skip)]
    children: Option<Vec<RelatedCard>>,
    #[tabled(skip)]
    blockers: Option<Vec<Blocker>>,
    #[tabled(skip)]
    blocking_blockers: Option<Vec<Blocker>>,
}

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct RelatedCard {
    id: u32,
    title: String,
    board_id: u32,
    column_id: u32,
    lane_id: u32,
    #[tabled(rename = "type")]
    r#type: CardType,
    condition: u8,
    state: u8,
}

impl RelatedCard {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PropertiesValue {
    S(String),
    V(Vec<u32>),
}

impl Card {
    fn new() -> Card {
        Card {
            id: 1,
            title: "Title".to_string(),
            column: Column::new(),
            column_id: 0,
            board_id: 0,
            lane_id: 0,
            lane: Lane::new(),
            blocked: false,
            blocking_card: None,
            block_reason: String::new(),
            properties: None,
            r#type: CardType::new(),
            tags: None,
            sort_order: 0.0,
            members: None,
            description: Some("".to_string()),
            archived: false,
            created: String::new(),
            last_moved_at: String::new(),
            checklists: None,
            parents: None,
            children: None,
            blockers: None,
            blocking_blockers: None,
        }
    }

    fn process_blocker(
        proccessed: &mut Vec<[String; 3]>,
        blockers: Option<Vec<Blocker>>,
        red: bool,
    ) {
        if let Some(blockers) = blockers {
            for b in blockers.iter() {
                let reason = b.get_reason();
                let reason = if red {
                    reason.red().to_string()
                } else {
                    reason.yellow().to_string()
                };
                let mut btitle = String::new();
                let mut bcardid = String::new();
                if let Some(card) = b.get_card() {
                    btitle = card.get_title().to_string();
                    btitle = if red {
                        btitle.red().to_string()
                    } else {
                        btitle.yellow().to_string()
                    };
                    bcardid = card.get_id().to_string();
                }
                proccessed.push([reason, btitle, bcardid])
            }
        }
    }

    pub fn to_table_string(&self) -> String {
        let desc = self.description.clone().unwrap_or(String::new());
        let mut proccessed: Vec<[String; 3]> = Vec::new();
        Self::process_blocker(&mut proccessed, self.blocking_blockers.clone(), false);
        Self::process_blocker(&mut proccessed, self.blockers.clone(), true);
        let blockers_len = proccessed.len();
        let s: String;
        if let Some(checklist) = &self.checklists {
            s = checklist
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join("\n\n");
        } else {
            s = String::new();
        }
        let mut table_data: Vec<[String; 3]> = Vec::from([
            ["Type".to_string(), "Column".to_string(), "Line".to_string()],
            [
                self.r#type.get_letter().to_string(),
                self.column.get_title().to_string(),
                self.lane.get_title().to_string(),
            ],
            [
                self.r#type.get_id().to_string(),
                self.column.get_id().to_string(),
                self.lane.get_id().to_string(),
            ],
            [
                self.get_string_tags(),
                self.get_string_members(),
                Self::display_move_diff(&self.last_moved_at),
            ],
        ]);

        table_data.extend(proccessed);
        table_data.extend([
            [desc, String::new(), String::new()],
            [s, String::new(), String::new()],
        ]);
        let mut table = Table::new(table_data);
        let title: String;
        if self.blocked {
            title = self.title.red().bold().to_string();
        } else if self.blocking_card.is_some_and(|v| v) {
            title = self.title.yellow().bold().to_string();
        } else {
            title = self.title.bold().to_string()
        };
        table
            .with(Style::modern())
            .with(Disable::row(Rows::first()))
            .with(Panel::header(title))
            .modify((5 + blockers_len, 0), Span::column(3))
            .modify((6 + blockers_len, 0), Span::column(3))
            .with(Alignment::center())
            .with(Width::wrap(130).keep_words())
            .to_string()
    }

    fn calculate_hour_diff(lst: &String) -> i64 {
        let parse_date = lst.parse::<DateTime<Utc>>();
        if parse_date.is_err() {
            return -999;
        } else {
            let target_date = parse_date.unwrap().with_timezone(&Utc);
            let now = Utc::now();
            let duration = now.signed_duration_since(target_date);
            duration.num_hours()
        }
    }

    fn display_move_diff(lst: &String) -> String {
        let mut hour_diff = Self::calculate_hour_diff(lst);
        let diff_string = if hour_diff > 24 {
            let day_diff = hour_diff / 24;
            hour_diff = hour_diff % 24;
            let s = format!("{}d{}h", day_diff, hour_diff);
            s.red().to_string()
        } else {
            format!("{}h", hour_diff)
        };
        diff_string
    }

    pub fn from_string() -> String {
        let text = String::new();
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
        card_text
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.tags.clone().unwrap_or(vec![])
    }
    pub fn get_members(&self) -> Vec<User> {
        self.members.clone().unwrap_or(vec![])
    }

    pub fn get_string_tags(&self) -> String {
        let tags = self.get_tags();
        tags.iter()
            .map(|t| t.get_name())
            .collect::<Vec<&str>>()
            .join(", ")
    }
    pub fn get_string_members(&self) -> String {
        let members = self.get_members();
        members
            .iter()
            .map(|m| {
                if m.is_responsible() {
                    m.get_username().green().bold().to_string()
                } else {
                    m.get_username().to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn is_property(&self, property_id: u32) -> bool {
        if let Some(properties) = &self.properties {
            let key = format!("id_{}", property_id);
            properties.contains_key(&key)
        } else {
            false
        }
    }

    pub fn is_property_value(&self, property_value_id: u32) -> bool {
        if let Some(properties) = &self.properties {
            for item in properties.values() {
                let contains = match item {
                    PropertiesValue::S(s) => {
                        let string_id = format!("{}", property_value_id);
                        string_id.eq(s)
                    }
                    PropertiesValue::V(v) => v.contains(&property_value_id),
                };
                if contains {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn set_column_id(&mut self, column_id: u32) {
        self.column_id = column_id
    }
    pub fn set_board_id(&mut self, board_id: u32) {
        self.board_id = board_id
    }
    pub fn set_lane_id(&mut self, lane_id: u32) {
        self.lane_id = lane_id
    }

    fn is_member(&self, username: &str) -> bool {
        if let Some(members) = &self.members {
            members.iter().any(|m| m.is_username(username))
        } else {
            false
        }
    }

    pub fn get_parents(&self) -> Vec<RelatedCard> {
        self.parents.as_ref().cloned().unwrap_or(vec![])
    }
    pub fn get_childrens(&self) -> Vec<RelatedCard> {
        self.children.as_ref().cloned().unwrap_or(vec![])
    }
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
            let mems: Vec<&str> = members
                .into_iter()
                .filter(|m| m.is_responsible())
                .map(|m| m.get_username())
                .collect();
            format!("{}", mems.join(",\n"))
        }
        None => format!(""),
    }
}

fn display_tags(tags: &Option<Vec<Tag>>) -> String {
    match tags {
        Some(tags) => {
            let str_tags: Vec<&str> = tags.into_iter().map(|tag| tag.get_name()).collect();
            format!("{}", str_tags.join("\n"))
        }
        None => format!(""),
    }
}
