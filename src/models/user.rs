
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct User{
    pub id: u32,
    pub username: String,
    #[tabled(skip)]
    pub r#type: Option<u8>
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
impl User {
    pub fn set_responsible(&mut self, responsible: bool) {
        if responsible {
            self.r#type = Some(2);
        } else {
            self.r#type = Some(1);
        }
    }
}
