use serde::{Deserialize, Serialize};

use crate::{
    errors::CrawlerError,
    response_processor::traits::{TExtractData, TStoreType},
    OUTPUT_PATH,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayStoreReview {
    pub date: String,
    pub star: i32,
    pub like: i32,
    pub dislike: i32,
    pub title: String,
    pub review: String,
}

impl PlayStoreReview {
    pub fn new() -> Self {
        Self {
            date: String::new(),
            star: 0,
            like: 0,
            dislike: 0,
            title: String::new(),
            review: String::new(),
        }
    }
}

impl TStoreType for PlayStoreReview {
    fn get_output_path(&self, app_id: &str) -> String {
        format!("{OUTPUT_PATH}/play_store/{app_id}.csv")
    }
}

impl TExtractData for PlayStoreReview {
    fn extract_data(&self, _response: &[u8]) -> Result<Vec<Self>, CrawlerError> {
        // TODO: Play Store review parsing not implemented yet
        todo!("Play Store review parsing not implemented yet")
    }
}
