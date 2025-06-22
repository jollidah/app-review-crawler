use crate::errors::CrawlerError;

pub trait TStoreType {
    fn get_output_path(&self, app_id: &str) -> String;
}

pub trait TExtractData: serde::Serialize {
    fn extract_data(&self, response: &[u8]) -> Result<Vec<Self>, CrawlerError>
    where
        Self: Sized;
}

// 공통 CSV 저장 함수
pub fn save_data_to_csv<T>(
    data: Vec<T>,
    store_type: &dyn TStoreType,
    app_id: &str,
) -> Result<(), CrawlerError>
where
    T: serde::Serialize,
{
    use std::fs;
    use std::path::PathBuf;

    // 출력 디렉토리 생성
    let output_path = PathBuf::from(store_type.get_output_path(app_id));
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| CrawlerError::Request(format!("Failed to create directory: {}", e)))?;
    }

    // CSV 파일 작성
    let mut wtr = csv::Writer::from_path(&output_path)
        .map_err(|e| CrawlerError::Request(format!("Failed to create CSV file: {}", e)))?;

    for item in data {
        wtr.serialize(item)
            .map_err(|e| CrawlerError::Request(format!("Failed to serialize data: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| CrawlerError::Request(format!("Failed to flush CSV file: {}", e)))?;

    Ok(())
}
