use crate::models::{Column, User};
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use crate::command::{Commands, CardOptions};
use crate::models::common::{USERS, COLUMNS};

pub struct ApiClient {
    client: reqwest::Client,
    base_api_url: String,
    token: String,
}

impl ApiClient {
    pub fn default() -> ApiClient {
        let client = reqwest::Client::new();
        let base_api_url = env::var("API_URL").expect("Environment variable API_URL not set");
        let token = env::var("KT").expect("Environment variable KT for token not set");
        ApiClient {
            client,
            base_api_url,
            token,
        }
    }

    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/boards/96239/columns/", self.base_api_url);
        let response = self.get_data(url.as_str()).await?;
        let json: Vec<Column> = response.json().await?;
        for d in json.iter() {
            let title = d.title.as_str();
            let column = Column{title: d.title.to_string(), id: d.id, sort_order: d.sort_order};
            COLUMNS.lock().unwrap().insert(title.to_string(), column);
        };
        let url = format!("{}/users/", self.base_api_url);
        let response = self.get_data(url.as_str()).await?;
        let json: Vec<User> = response.json().await?;
        for d in json.iter() {
            let author = d.username.as_str();
            USERS.lock().unwrap().insert(author.to_string(), d.id);
        }
        Ok(())


    }
    pub fn get_url(&self, cmd: &Commands) -> String {
        match cmd {
            Commands::Card { options } => match options {
                CardOptions::Ls{} | CardOptions::New {} => {
                    format!("{}/cards?space_id=38223", self.base_api_url,)
                }
                CardOptions::Get { card_id }
                | CardOptions::Edit { card_id }
                | CardOptions::Mv { card_id } => {
                    format!("{}/cards/{}", self.base_api_url, card_id)
                }
            },
            Commands::Columns {} => {
                format!("{}/boards/96239/columns/", self.base_api_url)
            }
            Commands::Users {} => {
                format!("{}/users/", self.base_api_url)
            }
        }
    }

    pub async fn get_data(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let response = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;
        Ok(response)
    }
    pub async fn patch_data(
        &self,
        url: &str,
        data: Column,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let response = self
            .client
            .patch(url)
            .json(&data)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await?;
        Ok(response)
    }
}
