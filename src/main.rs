mod models;
mod command;
mod api;

use crate::command::Cli;
use clap::Parser;
use termimad::{MadSkin, rgb};
use crate::api::ApiClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::default();
    let cli = Cli::parse();
    let result = match cli.execute().await {
        Ok(result) => {result},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    };
    let api_url = cli.get_url();
    println!("{}", api_url);
    let response = client.get_data(&api_url).await?;
    let table = cli.get_table(response).await?;
    let mut skin = MadSkin::default();
    // println!("{}", skin.inline(&table));
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.print_text(&table);
    // println!("{}", skin.inline(&table));
    Ok(())
}
