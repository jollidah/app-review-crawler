use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CrawlerError {
    ConfigLoad(String),
    Request(String),
    Parse(String),
}

impl fmt::Display for CrawlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrawlerError::ConfigLoad(msg) => write!(f, "Config load error: {msg}"),
            CrawlerError::Request(msg) => write!(f, "Request error: {msg}"),
            CrawlerError::Parse(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl Error for CrawlerError {}
