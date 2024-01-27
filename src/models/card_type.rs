
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardType {
    id: u32,
    name: String,
    letter: String,
    archived: bool,
}
impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_letter())
    }
}
impl CardType {
    pub fn new() -> CardType {
        CardType {
            id: 0,
            name: String::new(),
            letter: String::new(),
            archived: false
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_letter(&self) -> &str {
        &self.letter
    }
}
