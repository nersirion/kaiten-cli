use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Comment {
    #[serde(skip_serializing)]
    pub created: String,
    #[serde(skip_serializing)]
    id: u32,
    text: String,
    #[tabled(skip)]
    #[serde(skip_serializing)]
    edited: bool,
    #[serde(skip_serializing)]
    author: User,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Comment {
    pub fn from_text(text: &str) -> Self {
        Comment {
            created: String::new(),
            id: 0,
            text: text.to_string(),
            edited: false,
            author: User::default()
        }
    }
}
