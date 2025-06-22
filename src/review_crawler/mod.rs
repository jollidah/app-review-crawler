use std::sync::OnceLock;

use reqwest::{Client, Response};

use crate::{errors::CrawlerError, review_crawler::traits::TBuildReqeust};

pub mod app_store;
pub mod play_store;
mod traits;

pub struct Crawler<C: TBuildReqeust> {
    client: C,
}

impl<C: TBuildReqeust> Crawler<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn run(&mut self) -> Result<Vec<Response>, CrawlerError> {
        let mut responses = Vec::new();

        // 페이지가 10이 될 때까지 계속 크롤링
        while self.client.has_more_pages() {
            crate::log_debug!("Crawling page {}", self.client.get_current_page());

            let response = self
                .client
                .build_request()
                .send()
                .await
                .map_err(|e| CrawlerError::Request(e.to_string()))?;

            responses.push(response);
            self.client.increment_page();

            // 요청 간 짧은 딜레이 추가 (rate limiting 방지)
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        Ok(responses)
    }
}

pub fn get_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}
