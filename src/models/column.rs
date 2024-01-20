use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Column {
    pub id: u32,
    pub title: String,
    board_id: u32,
    #[tabled(skip)]
    pub sort_order: f32,
    #[tabled(skip)]
    column_id: Option<u32>,
    #[tabled(skip)]
    subcolumns: Option<Vec<Column>>
}
impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Column {
    pub fn new() -> Column {
        Column {
            id: 0,
            title: String::new(),
            board_id: 0,
            column_id: None,
            subcolumns: None,
            sort_order: 0.0,
        }
    }
}
