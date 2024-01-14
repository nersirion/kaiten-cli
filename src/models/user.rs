
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct User{
    pub id: u32,
    pub username: String
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
