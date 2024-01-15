
use tabled::{settings::{object::Rows, Width, Modify, Style}, Table };
use clap::{Args, Parser, Subcommand};
use crate::models::{Card as ModelsCard};


#[derive(Subcommand)]
pub enum CardOptions {
    /// print all cards for user
    Ls {},
    /// get card info
    Get { card_id: String },
    /// edit card
    Edit { card_id: String },
    /// create new card
    New {},
    /// move card to next column
    Mv { card_id: String },
}

#[derive(Args)]
pub struct Card {
    #[clap(long, short)]
    columns: Option<String>,
}
impl CardOptions {
    pub fn get_table(&self, json: Vec<ModelsCard>) -> String {
        match self {
            CardOptions::Ls{} => {
                let mut filter_cards: Vec<&ModelsCard> = json
                    .iter()
                    .filter(|card| card.column.title != "Done" && !card.archived)
                    .collect();
                filter_cards.sort_by(|a, b| a.sort_order.partial_cmp(&b.sort_order).unwrap());
                Table::new(filter_cards)
                    .with(Style::markdown())
                    .with(Modify::new(Rows::new(0..)).with(Width::wrap(70)))
                    .to_string()
                    // .with(Disable::Column(6..10))
            }
            _ => Table::new(vec![""]).to_string(),
        }
    }
}
