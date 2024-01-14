

use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Lane {
    pub id: u32,
    pub title: String,
}
impl std::fmt::Display for Lane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

