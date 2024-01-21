
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardType {
    id: u32,
    pub name: String,
    pub letter: String,
    archived: bool,
}
impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.letter)
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
}
