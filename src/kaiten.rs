use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

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
    #[tabled(skip)]
    checklists: Option<Vec<Checklist>>
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
        let lines: Vec<&str> = text.split("\n").collect();
        let name = lines[0][4..].to_string();
        let items = lines[2..].into_iter().map(|item| ChecklistItem::from_string(item.to_string())).collect();
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
    pub fn from_string(text: String) -> Self {
        let check = match &text[0..3] {
            "[ ]" => false,
            "[x]" => true,
            _ => panic!("Text will be start with [ ] or [x]")
        };
        Self {
            id: None,
            text: text[4..].to_string(),
            checked: check
        }
    }
}


