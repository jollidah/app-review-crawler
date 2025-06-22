#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        println!("\x1b[34m[DEBUG]\x1b[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("\x1b[32m[INFO]\x1b[0m {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31m[ERROR]\x1b[0m {}", format!($($arg)*))
    };
}
