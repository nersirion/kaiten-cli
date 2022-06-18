mod kaiten;
mod cli;

use reqwest;
use crate::kaiten::{Author, Card};
use crate::cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
     let cli = Cli::parse();
     let table = cli.get_table(client).await?;
     println!("{}", table);

    Ok(())
}
