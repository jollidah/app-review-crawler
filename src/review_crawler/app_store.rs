use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::review_crawler::{get_client, HasAppInfo, TBuildRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStoreClient {
    pub app_id: String,
    pub country: String,
    #[serde(default)]
    pub pages: u32,
}

impl HasAppInfo for AppStoreClient {
    fn app_id(&self) -> &str {
        &self.app_id
    }

    fn country(&self) -> &str {
        &self.country
    }
}

impl TBuildRequest for AppStoreClient {
    fn build_request(&mut self) -> RequestBuilder {
        get_client().get(format!(
            "https://itunes.apple.com/{}/rss/customerreviews/id={}/page={}/sortby=mostrecent/xml",
            self.country, self.app_id, self.pages
        ))
    }
    fn has_more_pages(&self) -> bool {
        self.pages <= 10
    }
    fn increment_page(&mut self) {
        self.pages += 1;
    }
    fn get_current_page(&self) -> u32 {
        self.pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_store_client_pagination() {
        let mut client = AppStoreClient {
            app_id: "test_app".to_string(),
            country: "us".to_string(),
            pages: 1,
        };

        // Test initial state
        assert_eq!(client.get_current_page(), 1);
        assert!(client.has_more_pages());

        // Test pagination through all pages
        for expected_page in 1..=10 {
            assert_eq!(client.get_current_page(), expected_page);
            assert!(client.has_more_pages());
            client.increment_page();
        }

        // After 10 pages, should not have more pages
        assert_eq!(client.get_current_page(), 11);
        assert!(!client.has_more_pages());
    }

    #[test]
    fn test_app_store_client_request_building() {
        let mut client = AppStoreClient {
            app_id: "123456789".to_string(),
            country: "kr".to_string(),
            pages: 5,
        };

        let request = client.build_request();
        let url = request.build().unwrap().url().to_string();

        // Check that the URL contains the expected components
        assert!(url.contains("itunes.apple.com"));
        assert!(url.contains("kr"));
        assert!(url.contains("123456789"));
        assert!(url.contains("page=5"));
        assert!(url.contains("sortby=mostrecent"));
        assert!(url.contains("xml"));
    }
}
