use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_time() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
