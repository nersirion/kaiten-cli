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
        println!("{}", url);
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
