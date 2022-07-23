mod kaiten;
mod cli;

use reqwest;
use crate::kaiten::{Author, Card};
use crate::cli::Cli;
use clap::Parser;
use termimad::{MadSkin, Alignment, rgb};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let cli = Cli::parse();
    let table = cli.get_table(client).await?;
    let mut skin = MadSkin::default();
    // println!("{}", skin.inline(&table));
    skin.set_headers_fg(rgb(255, 187, 0));
    println!("{}", skin.inline(&table));
    Ok(())
}
