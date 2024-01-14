
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Column {
    pub id: u32,
    pub title: String,
    #[tabled(skip)]
    pub sort_order: f32,
}
impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

