use reqwest;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::env;

pub struct ApiClient {
    client: reqwest::Client,
    base_api_url: reqwest::Url,
    token: String,
}

impl ApiClient {
    pub fn default() -> Result<ApiClient, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let base_api_url_str = env::var("API_URL")?;
        let base_api_url = reqwest::Url::parse(&base_api_url_str)?;
        let token = env::var("KT")?;
        Ok(ApiClient {
            client,
            base_api_url,
            token,
        })
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
    async fn make_request<T>(
        &self,
        method: reqwest::Method,
        api_url: &str,
        data: Option<T>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        let url = self.base_api_url.join(api_url)?;
        println!("{}", url);
        let request = self
            .client
            .request(method, url)
            .headers(self.common_headers());

        let request = match data {
            Some(data) => request.json(&data),
            None => request,
        };

        let response = request.send().await?;
        Ok(response)
    }


    pub async fn get_data(&self, api_url: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.make_request::<()>(reqwest::Method::GET, api_url, None).await
    }
    pub async fn patch_data<T>(
        &self,
        api_url: &str,
        data: T,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        self.make_request(reqwest::Method::PATCH, api_url, Some(data)).await
    }
    pub async fn post_data<T>(
        &self,
        api_url: &str,
        data: T,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        self.make_request(reqwest::Method::POST, api_url, Some(data)).await
    }
}
