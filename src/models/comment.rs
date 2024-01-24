use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Comment {
    pub created: String,
    id: u32,
    text: String,
    #[tabled(skip)]
    edited: bool,
    #[tabled(skip)]
    card_id: u32,
    #[tabled(skip)]
    author_id: u32,
    author: User,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
