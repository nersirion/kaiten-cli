
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Option<u32>,
    pub name: String,
}

impl Tag {
    pub fn from_string(text: String) -> Self {
        Tag {
            id: None,
            name: text
        }
    }
}
