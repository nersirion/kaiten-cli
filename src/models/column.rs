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
    #[tabled(display_with="Column::dispay_subcolumns")]
    subcolumns: Option<Vec<Column>>,
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
    fn dispay_subcolumns(subcolumns: &Option<Vec<Column>>) -> String {
        match subcolumns {
            Some(subcolumns) => {
                let mut scols = subcolumns.clone();
                scols.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                let str_cols: Vec<String> = scols
                    .iter()
                    .map(|c|  format!("{}-{}", c.title, c.id))
                    .collect();
                format!("{}", str_cols.join(", "))
            }
            None => format!(""),
        }
    }
}
