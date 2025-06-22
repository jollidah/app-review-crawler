use std::{fs::File, io::BufReader};

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::{
    errors::CrawlerError,
    review_crawler::{app_store::AppStoreClient, play_store::PlayStoreClient},
};

// JSON 구조에 맞는 중간 구조체들
#[derive(Debug, Deserialize)]
struct ClientsConfig {
    #[serde(default)]
    app_store: Option<Vec<AppStoreClient>>,
    #[serde(default)]
    play_store: Option<Vec<PlayStoreClient>>,
}

#[derive(Debug)]
pub struct Clients {
    pub app_store_apps: RwLock<Vec<AppStoreClient>>,
    pub play_store_apps: RwLock<Vec<PlayStoreClient>>,
}

pub fn load_target_apps(path: &str) -> Result<Clients, CrawlerError> {
    println!("[DEBUG] Starting load_target_apps with path: {}", path);

    // 파일 열기
    println!("[DEBUG] Attempting to open file: {}", path);
    let file = match File::open(path) {
        Ok(file) => {
            println!("[DEBUG] Successfully opened file");
            file
        }
        Err(e) => {
            println!("[ERROR] Failed to open file: {}", e);
            return Err(CrawlerError::ConfigLoad(e.to_string()));
        }
    };

    let reader = BufReader::new(file);
    println!("[DEBUG] Created BufReader");

    // JSON 파싱
    println!("[DEBUG] Attempting to parse JSON");
    let config: ClientsConfig = match serde_json::from_reader(reader) {
        Ok(config) => {
            println!("[DEBUG] Successfully parsed JSON");
            config
        }
        Err(e) => {
            println!("[ERROR] Failed to parse JSON: {}", e);
            return Err(CrawlerError::ConfigLoad(e.to_string()));
        }
    };

    println!("[DEBUG] Parsed config: {:?}", config);

    // App Store 앱들 처리
    println!("[DEBUG] Processing app_store apps");
    let app_store_apps = match &config.app_store {
        Some(apps) => {
            println!("[DEBUG] Found {} app_store apps", apps.len());
            RwLock::new(apps.clone())
        }
        None => {
            println!("[DEBUG] No app_store apps found, using empty vector");
            RwLock::new(Vec::new())
        }
    };

    // Play Store 앱들 처리
    println!("[DEBUG] Processing play_store apps");
    let play_store_apps = match &config.play_store {
        Some(apps) => {
            println!("[DEBUG] Found {} play_store apps", apps.len());
            RwLock::new(apps.clone())
        }
        None => {
            println!("[DEBUG] No play_store apps found, using empty vector");
            RwLock::new(Vec::new())
        }
    };

    println!("[DEBUG] Successfully created Clients struct");
    Ok(Clients {
        app_store_apps,
        play_store_apps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_target_apps_from_json(json_content: &str) -> Result<Clients, CrawlerError> {
        let config: ClientsConfig = serde_json::from_str(json_content)
            .map_err(|e| CrawlerError::ConfigLoad(e.to_string()))?;

        let app_store_apps = RwLock::new(config.app_store.unwrap_or_default());
        let play_store_apps = RwLock::new(config.play_store.unwrap_or_default());

        Ok(Clients {
            app_store_apps,
            play_store_apps,
        })
    }

    #[tokio::test]
    async fn test_load_target_apps_with_app_store_only() {
        let json_content = r#"
        {
            "app_store": [
                {
                    "country": "us",
                    "app_id": "1194408342"
                },
                {
                    "country": "us",
                    "app_id": "284882215"
                },
                {
                    "country": "kr",
                    "app_id": "284882216"
                }
            ]
        }
        "#;

        let result = load_target_apps_from_json(json_content);

        assert!(result.is_ok());
        let target_apps = result.unwrap();

        // App Store 앱들이 올바르게 로드되었는지 확인
        assert_eq!(target_apps.app_store_apps.read().await.len(), 3);
        assert_eq!(target_apps.play_store_apps.read().await.len(), 0);

        // 특정 앱이 올바르게 로드되었는지 확인
        let app_store_apps = target_apps.app_store_apps.read().await;
        let weather_app = app_store_apps
            .iter()
            .find(|app| app.app_id == "1194408342")
            .unwrap();
        assert_eq!(weather_app.country, "us");

        let facebook_us = app_store_apps
            .iter()
            .find(|app| app.app_id == "284882215")
            .unwrap();
        assert_eq!(facebook_us.country, "us");

        let facebook_kr = app_store_apps
            .iter()
            .find(|app| app.app_id == "284882216")
            .unwrap();
        assert_eq!(facebook_kr.country, "kr");
    }

    #[tokio::test]
    async fn test_load_target_apps_with_play_store_only() {
        let json_content = r#"
        {
            "play_store": [
                {
                    "country": "us",
                    "app_id": "com.instagram.android"
                },
                {
                    "country": "kr",
                    "app_id": "com.instagram.android"
                }
            ]
        }
        "#;

        let result = load_target_apps_from_json(json_content);

        assert!(result.is_ok());
        let target_apps = result.unwrap();

        // Play Store 앱들이 올바르게 로드되었는지 확인
        assert_eq!(target_apps.app_store_apps.read().await.len(), 0);
        assert_eq!(target_apps.play_store_apps.read().await.len(), 2);

        // PlayStoreClient 타입으로 올바르게 deserialize되었는지 확인
        let play_store_apps = target_apps.play_store_apps.read().await;
        let instagram_apps: Vec<_> = play_store_apps
            .iter()
            .filter(|app| app.app_id == "com.instagram.android")
            .collect();
        assert_eq!(instagram_apps.len(), 2);
    }

    #[tokio::test]
    async fn test_load_target_apps_with_both_stores() {
        let json_content = r#"
        {
            "app_store": [
                {
                    "country": "us",
                    "app_id": "123456789"
                }
            ],
            "play_store": [
                {
                    "country": "us",
                    "app_id": "com.weather.app"
                }
            ]
        }
        "#;

        let result = load_target_apps_from_json(json_content);

        assert!(result.is_ok());
        let target_apps = result.unwrap();

        // 두 스토어 모두 올바르게 로드되었는지 확인
        assert_eq!(target_apps.app_store_apps.read().await.len(), 1);
        assert_eq!(target_apps.play_store_apps.read().await.len(), 1);

        // App Store 앱
        let app_store_apps = target_apps.app_store_apps.read().await;
        let app_store_app = &app_store_apps[0];
        assert_eq!(app_store_app.app_id, "123456789");
        assert_eq!(app_store_app.country, "us");

        // Play Store 앱
        let play_store_apps = target_apps.play_store_apps.read().await;
        let play_store_app = &play_store_apps[0];
        assert_eq!(play_store_app.app_id, "com.weather.app");
        assert_eq!(play_store_app.country, "us");
    }

    #[tokio::test]
    async fn test_load_target_apps_with_empty_json() {
        let json_content = "{}";

        let result = load_target_apps_from_json(json_content);

        assert!(result.is_ok());
        let target_apps = result.unwrap();

        // 빈 JSON의 경우 두 벡터 모두 비어있어야 함
        assert_eq!(target_apps.app_store_apps.read().await.len(), 0);
        assert_eq!(target_apps.play_store_apps.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_load_target_apps_with_unknown_keys() {
        let json_content = r#"
        {
            "app_store": [
                {
                    "country": "us",
                    "app_id": "123456789"
                }
            ],
            "unknown_store": [
                {
                    "country": "us",
                    "app_id": "999999999"
                }
            ]
        }
        "#;

        let result = load_target_apps_from_json(json_content);

        assert!(result.is_ok());
        let target_apps = result.unwrap();

        // unknown_store는 무시되고 app_store만 처리되어야 함
        assert_eq!(target_apps.app_store_apps.read().await.len(), 1);
        assert_eq!(target_apps.play_store_apps.read().await.len(), 0);

        let app_store_apps = target_apps.app_store_apps.read().await;
        let app = &app_store_apps[0];
        assert_eq!(app.app_id, "123456789");
        assert_eq!(app.country, "us");
    }

    #[test]
    fn test_load_target_apps_with_invalid_json() {
        let json_content = "{ invalid json }";

        let result = load_target_apps_from_json(json_content);

        // 잘못된 JSON은 에러를 반환해야 함
        assert!(result.is_err());
        match result.unwrap_err() {
            CrawlerError::ConfigLoad(_) => (), // 예상된 에러 타입
            _ => panic!("Expected ConfigLoad"),
        }
    }

    #[test]
    fn test_load_target_apps_with_nonexistent_file() {
        let result = load_target_apps("nonexistent_file.json");

        // 존재하지 않는 파일은 에러를 반환해야 함
        assert!(result.is_err());
        match result.unwrap_err() {
            CrawlerError::ConfigLoad(_) => (), // 예상된 에러 타입
            _ => panic!("Expected ConfigLoad"),
        }
    }
}
