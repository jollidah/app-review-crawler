use std::sync::OnceLock;

use reqwest::{Client, Response};

use crate::errors::CrawlerError;

pub mod app_store;
pub mod play_store;
pub mod traits;
pub use traits::{HasAppInfo, TBuildRequest};

pub struct Crawler<C: TBuildRequest> {
    client: C,
}

impl<C: TBuildRequest> Crawler<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn run(&mut self) -> Result<Vec<Response>, CrawlerError> {
        let mut responses = Vec::new();

        // 페이지가 10이 될 때까지 계속 크롤링
        while self.client.has_more_pages() {
            tracing::debug!("Crawling page {}", self.client.get_current_page());

            let response = self
                .client
                .build_request()
                .send()
                .await
                .map_err(|e| CrawlerError::Request(e.to_string()))?;

            responses.push(response);
            self.client.increment_page();
        }

        Ok(responses)
    }
}

pub fn get_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

pub(crate) fn get_default_pages() -> u32 {
    1
}
