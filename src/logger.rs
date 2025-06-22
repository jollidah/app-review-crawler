use tracing::{info_span, Span};
use tracing_subscriber::fmt;

pub fn init() {
    fmt()
        .with_target(false)
        .with_level(true)
        .with_ansi(true)
        .init();
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*);
    };
}

pub fn span(name: &str) -> Span {
    info_span!(name)
}
