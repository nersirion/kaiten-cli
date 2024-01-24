
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;
use super::{Column, Lane};
#[derive(Serialize, Deserialize, Debug, Tabled, Clone)]
pub struct Board{
    id: u32,
    space_id: u32,
    title: String,
    #[tabled(skip)]
    columns: Option<Vec<Column>>,
    #[tabled(skip)]
    lanes: Option<Vec<Lane>>
}
impl Board {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_columns(&self) -> Vec<Column> {
        self.columns.as_ref().cloned().unwrap_or(vec![])
    }

    pub fn get_lanes(&self) -> Vec<Lane> {
        self.lanes.as_ref().cloned().unwrap_or(vec![])
    }

    pub fn get_space_id(&self) -> u32 {
        self.space_id
    }
}
