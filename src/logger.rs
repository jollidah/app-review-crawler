use tracing_subscriber::fmt;

pub fn init(level: tracing::Level) {
    fmt().with_max_level(level).with_level(true).init();
}
