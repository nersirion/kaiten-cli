use crate::models::Comment as ModelsComment;
use clap::{Args, Subcommand};
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table,
};
use crate::api::ApiClient;

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
        client: ApiClient,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = self.get_url();
        let response = client.get_data(&api_url).await?;
        let table = match &self.command {
            CommentCommands::Get { .. } => {
                let mut json: Vec<ModelsComment> = response.json().await?;
                json.sort_by(|a, b| a.created.partial_cmp(&b.created).unwrap());
                Table::new(json).modify(Columns::first(), Width::wrap(10)).modify(Columns::single(2), Width::wrap(80).keep_words()).with(Style::modern()).to_string()
            },
            CommentCommands::New{card_id: _, comment} => {
                let comment = ModelsComment::from_text(comment);
                let response = client.post_data(&api_url, comment).await?;
                response.text().await?
            }
            _ => String::new(),
        };
        Ok(table)
    }
}
