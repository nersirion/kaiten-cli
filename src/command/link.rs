use crate::api::ApiClient;
use crate::models::Link as ModelsLink;
use clap::{Args, Subcommand};
use tabled::{
    settings::{object::Columns, Style, Width},
    Table,
};

#[derive(Args)]
pub struct Link {
    #[command(subcommand)]
    pub command: LinkCommands,
}

#[derive(Subcommand)]
pub enum LinkCommands {
    /// get card comments
    Get { card_id: String },
    /// edit comment in card
    Edit { card_id: String, link_id: String },
    /// create new comment in card
    New { card_id: String,
        #[arg(long)]
        link: String,
        #[arg(long, short)]
        description: Option<String>
    },
}

impl Link {
    pub fn get_url(&self) -> String {
        match &self.command {
            LinkCommands::Get { card_id } | LinkCommands::New { card_id, .. } => {
                format!("cards/{}/external-links", card_id)
            }
            LinkCommands::Edit {
                card_id,
                link_id,
            } => format!("cards/{}/external-links/{}", card_id, link_id),
        }
    }
    pub async fn get_table(&self, client: ApiClient) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = self.get_url();
        let response = client.get_data(&api_url).await?;
        let table = match &self.command {
            LinkCommands::Get { .. } => {
                let links: Vec<ModelsLink> = response.json().await?;
                Table::new(links)
                    .modify(Columns::first(), Width::wrap(10))
                    .modify(Columns::single(2), Width::wrap(80).keep_words())
                    .with(Style::modern())
                    .to_string()
            }
            LinkCommands::New {card_id: _, link, description} => {
                let link = ModelsLink::from(link.to_owned(), description.to_owned());
                let _ = client.post_data(&api_url, link).await?;
                String::new()
            }
            _ => String::new(),
        };
        Ok(table)
    }
}
