use tokio::task;

use crate::{
    response_processor::{
        app_store::AppStoreReview, play_store::PlayStoreReview, RawResponse, ResponseProcessor,
    },
    review_crawler::{
        traits::{HasAppInfo, TBuildRequest},
        Crawler,
    },
    target_app::load_target_apps,
};

mod errors;
mod response_processor;
mod review_crawler;
mod target_app;
mod logger;

const TARGET_APPS_PATH: &str = "target_apps.json";
const OUTPUT_PATH: &str = "output";
const APP_STORE_MAX_PAGES: u32 = 10;
const GOOGLE_PLAY_MAX_PAGES: u32 = 100;

async fn run_store_crawler<C, D, F>(store_name: &str, apps: Vec<C>, make_extractor: F)
where
    C: TBuildRequest + HasAppInfo + Clone + Send + 'static,
    D: response_processor::traits::TExtractData
        + response_processor::traits::TStoreType
        + Send
        + 'static,
    F: Fn() -> D,
{
    crate::log_info!("Starting {} crawler task", store_name);
    crate::log_info!("Found {} {} apps to crawl", apps.len(), store_name);

    for (i, app) in apps.iter().enumerate() {
        crate::log_info!(
            "Crawling {} app {}/{}: {} (country: {})",
            store_name,
            i + 1,
            apps.len(),
            app.app_id(),
            app.country()
        );

        let app_id = app.app_id().to_string();
        let mut crawler = Crawler::new(app.clone());

        match crawler.run().await {
            Ok(response) => {
                crate::log_info!("Successfully got response for app: {}", app_id);
                let processor: ResponseProcessor<D> = ResponseProcessor::new(
                    RawResponse::new(response),
                    make_extractor(),
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
}

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
        let apps = target_apps.app_store_apps.read().await.clone();
        run_store_crawler("App Store", apps, AppStoreReview::new).await;
    });

    task::spawn(async move {
        let apps = target_apps.play_store_apps.read().await.clone();
        run_store_crawler("Play Store", apps, PlayStoreReview::new).await;
    });

    // 메인 스레드가 종료되지 않도록 잠시 대기
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    crate::log_info!("Crawler finished");
}
