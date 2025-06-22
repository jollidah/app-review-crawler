use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use crate::review_crawler::{get_client, HasAppInfo, TBuildRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayStoreClient {
    pub app_id: String,
    pub country: String,
    #[serde(default)]
    pub pages: u32,
}

impl HasAppInfo for PlayStoreClient {
    fn app_id(&self) -> &str {
        &self.app_id
    }

    fn country(&self) -> &str {
        &self.country
    }
}

impl TBuildRequest for PlayStoreClient {
    fn build_request(&mut self) -> RequestBuilder {
        // Play Store API endpoint (placeholder - needs actual implementation)
        get_client()
            .get(format!(
                "https://play.google.com/store/getreviews?hl={}&gl={}&reviewType=0&reviewSortOrder=4&pageNum={}&id={}",
                self.country, self.country, self.pages, self.app_id
            ))
    }

    fn has_more_pages(&self) -> bool {
        self.pages <= 100
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
    fn test_play_store_client_pagination() {
        let mut client = PlayStoreClient {
            app_id: "test_app".to_string(),
            country: "us".to_string(),
            pages: 0,
        };

        // Test initial state
        assert_eq!(client.get_current_page(), 0);
        assert!(client.has_more_pages());

        // Test pagination through all pages (100 for Play Store)
        for expected_page in 0..=100 {
            assert_eq!(client.get_current_page(), expected_page);
            assert!(client.has_more_pages());
            client.increment_page();
        }

        // After 100 pages, should not have more pages
        assert_eq!(client.get_current_page(), 101);
        assert!(!client.has_more_pages());
    }

    #[test]
    fn test_play_store_client_request_building() {
        let mut client = PlayStoreClient {
            app_id: "com.example.app".to_string(),
            country: "kr".to_string(),
            pages: 5,
        };

        let request = client.build_request();
        let url = request.build().unwrap().url().to_string();

        // Check that the URL contains the expected components
        assert!(url.contains("play.google.com"));
        assert!(url.contains("kr"));
        assert!(url.contains("com.example.app"));
        assert!(url.contains("pageNum=5"));
        assert!(url.contains("reviewType=0"));
        assert!(url.contains("reviewSortOrder=4"));
    }
}
