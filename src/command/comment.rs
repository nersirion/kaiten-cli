use crate::models::Comment as ModelsComment;
use clap::{Args, Subcommand};
use tabled::{
    settings::{object::Rows, Modify, Style, Width},
    Table,
};

#[derive(Args)]
pub struct Comment {
    #[command(subcommand)]
    pub command: CommentCommands,
}

#[derive(Subcommand)]
pub enum CommentCommands {
    /// get card comments
    Get { card_id: String },
    /// edit comment in card
    Edit { card_id: String, comment_id: String },
    /// create new comment in card
    New { card_id: String, comment: String },
}

impl Comment {
    pub fn get_url(&self) -> String {
        match &self.command {
            CommentCommands::Get { card_id } | CommentCommands::New { card_id, .. } => {
                format!("cards/{}/comments", card_id)
            }
            CommentCommands::Edit {
                card_id,
                comment_id,
            } => format!("cards/{}/comments/{}", card_id, comment_id),
        }
    }
    pub async fn get_table(
        &self,
        response: reqwest::Response,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let table = match &self.command {
            CommentCommands::Get { .. } => {
                let mut json: Vec<ModelsComment> = response.json().await?;
                json.sort_by(|a, b| a.created.partial_cmp(&b.created).unwrap());
                Table::new(json).with(Style::markdown()).to_string()
            }
            _ => String::new(),
        };
        Ok(table)
    }
}
