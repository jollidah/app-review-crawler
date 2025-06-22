use reqwest::RequestBuilder;

pub trait TBuildRequest {
    fn build_request(&mut self) -> RequestBuilder;
    fn has_more_pages(&self) -> bool;
    fn increment_page(&mut self);
    fn get_current_page(&self) -> u32;
}

pub trait HasAppInfo {
    fn app_id(&self) -> &str;
    fn country(&self) -> &str;
}
