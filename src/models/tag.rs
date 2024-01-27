
use tabled::Tabled;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Tag {
    id: u32,
    name: String,
}

impl Tag {
    pub fn from_string(text: String) -> Self {
        Tag {
            id: 0,
            name: text
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
