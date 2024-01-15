use std::collections::HashMap;
use crate::models::*;
use crate::models::common::{USERS, COLUMNS};
use clap::{Args, Parser, Subcommand};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tabled::{settings::{object::Rows, Width, Modify, Style}, Table };

const API_URL: &str = env!("API_URL");

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct M {
    column_id: u32
}
