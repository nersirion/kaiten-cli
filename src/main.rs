mod models;
mod command;
mod api;

use crate::command::Cli;
use clap::Parser;
use termimad::{MadSkin, rgb};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let result = match cli.execute().await {
        Ok(result) => {result},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1)
        }
    };
    let mut skin = MadSkin::default();
    // println!("{}", skin.inline(&table));
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.print_text(&result);
    // println!("{}", skin.inline(&table));
    Ok(())
}
