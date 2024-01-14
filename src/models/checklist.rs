use crate::models::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Checklist {
    pub id: Option<u32>,
    pub name: String,
    pub items: Option<Vec<ChecklistItem>>,
}

impl std::fmt::Display for Checklist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.items {
            Some(value) => {
                let items: Vec<String> = value.into_iter().map(|x| format!("{}", x)).collect();
                write!(f, "### {}\n\n{}", self.name, items.join("\n"))
            }
            None => {
                write!(f, "")
            }
        }
    }
}
impl Checklist {
    pub fn to_string(&self) -> String {
        let mut checklist = format!("### {}\n\n", self.name);
        let items_string = if self.items.is_some() {
            let items: Vec<String> = self
                .items
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|item| item.to_string())
                .collect();
            items.join("\n")
        } else {
            "".to_string()
        };
        checklist = format!("{}{}", checklist, items_string);
        checklist
    }
    pub fn from_string(text: String) -> Self {
        let lines: Vec<&str> = text.split("\n").filter(|line| line.len() > 3).collect();
        println!("{:?}", lines);
        let name = lines[0][4..].to_string();
        let items = lines[1..]
            .into_iter()
            .map(|item| ChecklistItem::from_string(item.to_string()))
            .collect();
        Self {
            id: None,
            name,
            items: Some(items),
        }
    }
}
