use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lane {
    pub id: u32,
    pub title: String,
    board_id: u32,
    condition: u8,
    sort_order: f32,
}

impl std::fmt::Display for Lane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
impl Lane {
    pub fn new() -> Lane {
        Lane {
            id: 0,
            title: String::new(),
            board_id: 0,
            condition: 1,
            sort_order: 0.0,
        }
    }
}
