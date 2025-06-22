use tokio::task;

use crate::{
    response_processor::{
        app_store::AppStoreReview, play_store::PlayStoreReview, RawResponse, ResponseProcessor,
    },
    review_crawler::Crawler,
    target_app::load_target_apps,
};

mod errors;
mod response_processor;
mod review_crawler;
mod target_app;
mod logger;

const TARGET_APPS_PATH: &str = "target_apps.json";
const OUTPUT_PATH: &str = "output";

#[tokio::main]
async fn main() {
    crate::log_info!("Starting app review crawler...");

    let target_apps = match load_target_apps(TARGET_APPS_PATH) {
        Ok(apps) => {
            crate::log_info!("Successfully loaded target apps");
            apps
        }
        Err(e) => {
            crate::log_error!("Failed to load target apps: {}", e);
            return;
        }
    };

    task::spawn(async move {
        crate::log_info!("Starting App Store crawler task");
        let app_store_apps = target_apps.app_store_apps.read().await.clone();
        crate::log_info!("Found {} App Store apps to crawl", app_store_apps.len());

        for (i, app) in app_store_apps.iter().enumerate() {
            crate::log_info!(
                "Crawling App Store app {}/{}: {} (country: {})",
                i + 1,
                app_store_apps.len(),
                app.app_id,
                app.country
            );

            let app_id = app.app_id.clone();
            let mut crawler = Crawler::new(app.clone());

            match crawler.run().await {
                Ok(response) => {
                    crate::log_info!("Successfully got response for app: {}", app_id);
                    let processor: ResponseProcessor<AppStoreReview> = ResponseProcessor::new(
                        RawResponse::new(response),
                        AppStoreReview::new(),
                        app_id.clone(),
                    );

                    match processor.run().await {
                        Ok(_) => crate::log_info!(
                            "Successfully processed and saved reviews for app: {}",
                            app_id
                        ),
                        Err(e) => crate::log_error!(
                            "Failed to process reviews for app {}: {}",
                            app_id, e
                        ),
                    }
                }
                Err(e) => {
                    crate::log_error!("Failed to crawl app {}: {}", app_id, e);
                }
            }
        }
    });

    task::spawn(async move {
        crate::log_info!("Starting Play Store crawler task");
        let play_store_apps = target_apps.play_store_apps.read().await.clone();
        crate::log_info!("Found {} Play Store apps to crawl", play_store_apps.len());

        for (i, app) in play_store_apps.iter().enumerate() {
            crate::log_info!(
                "Crawling Play Store app {}/{}: {} (country: {})",
                i + 1,
                play_store_apps.len(),
                app.app_id,
                app.country
            );

            let app_id = app.app_id.clone();
            let mut crawler = Crawler::new(app.clone());

            match crawler.run().await {
                Ok(response) => {
                    crate::log_info!("Successfully got response for app: {}", app_id);
                    let processor: ResponseProcessor<PlayStoreReview> = ResponseProcessor::new(
                        RawResponse::new(response),
                        PlayStoreReview::new(),
                        app_id.clone(),
                    );

                    match processor.run().await {
                        Ok(_) => crate::log_info!(
                            "Successfully processed and saved reviews for app: {}",
                            app_id
                        ),
                        Err(e) => crate::log_error!(
                            "Failed to process reviews for app {}: {}",
                            app_id, e
                        ),
                    }
                }
                Err(e) => {
                    crate::log_error!("Failed to crawl app {}: {}", app_id, e);
                }
            }
        }
    });

    // 메인 스레드가 종료되지 않도록 잠시 대기
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    crate::log_info!("Crawler finished");
}
