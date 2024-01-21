
use tabled::Tabled;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Tag {
    id: u32,
    pub name: String,
}

impl Tag {
    pub fn from_string(text: String) -> Self {
        Tag {
            id: 0,
            name: text
        }
    }
}
