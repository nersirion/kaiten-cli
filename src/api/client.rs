use crate::command::{CardCommands, Commands};
use crate::models::common::{COLUMNS, USERS};
use crate::models::{Column, User};
use reqwest;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::env;

const BOARD_ID: &str = "96239";
const SPACE_ID: &str = "38223";

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

    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("boards/{}/columns/", BOARD_ID);
        let response = self.get_data(url.as_str()).await?;
        let json: Vec<Column> = response.json().await?;
        for d in json.iter() {
            let title = d.title.as_str();
            let column = Column {
                title: d.title.to_string(),
                id: d.id,
                sort_order: d.sort_order,
            };
            COLUMNS.lock().unwrap().insert(title.to_string(), column);
        }
        let url = String::from("users/");
        let response = self.get_data(url.as_str()).await?;
        let json: Vec<User> = response.json().await?;
        for d in json.iter() {
            let author = d.username.as_str();
            USERS.lock().unwrap().insert(author.to_string(), d.id);
        }
        Ok(())
    }

    fn common_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }


    pub async fn get_data(&self, api_url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.base_api_url, api_url);
        let response = self
            .client
            .get(url)
            .headers(self.common_headers())
            .send()
            .await?;
        Ok(response)
    }
    pub async fn patch_data<T>(
        &self,
        api_url: &str,
        data: T,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        T: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_api_url, api_url);
        let response = self
            .client
            .patch(url)
            .json(&data)
            .headers(self.common_headers())
            .send()
            .await?;
        Ok(response)
    }
}
