use std::marker::PhantomData;

use reqwest::Response;

use crate::{
    errors::CrawlerError,
    response_processor::traits::{save_data_to_csv, TExtractData, TStoreType},
};

pub mod app_store;
pub mod play_store;
pub mod traits;

pub struct RawResponse<T> {
    responses: Vec<Response>,
    phantom: PhantomData<T>,
}

impl<T> RawResponse<T> {
    pub fn new(responses: Vec<Response>) -> Self {
        Self {
            responses,
            phantom: PhantomData,
        }
    }
}

pub struct ResponseProcessor<D: TExtractData + TStoreType> {
    data: RawResponse<D>,
    extractor: D,
    app_id: String,
}

impl<D: TExtractData + TStoreType> ResponseProcessor<D> {
    pub fn new(data: RawResponse<D>, extractor: D, app_id: String) -> Self {
        Self {
            data,
            extractor,
            app_id,
        }
    }

    pub async fn run(self) -> Result<(), CrawlerError> {
        let mut all_data = Vec::new();
        let responses_count = self.data.responses.len();

        // 모든 응답에서 데이터 추출
        for (i, response) in self.data.responses.into_iter().enumerate() {
            crate::log_debug!("Processing response {}/{}", i + 1, responses_count);

            let bytes = response
                .bytes()
                .await
                .map_err(|e| CrawlerError::Request(e.to_string()))?;

            let data = self.extractor.extract_data(&bytes)?;
            all_data.extend(data);
        }

        // 모든 데이터를 하나의 CSV 파일로 저장
        save_data_to_csv(all_data, &self.extractor, &self.app_id)?;
        Ok(())
    }
}
