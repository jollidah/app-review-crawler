use reqwest::RequestBuilder;

pub trait TBuildReqeust {
    fn build_request(&mut self) -> RequestBuilder;
    fn has_more_pages(&self) -> bool;
    fn increment_page(&mut self);
    fn get_current_page(&self) -> u32;
}
