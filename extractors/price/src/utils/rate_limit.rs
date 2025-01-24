use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

pub const DEFAULT_RATE_LIMIT_COOLOFF_SECS: u64 = 5;

pub type RateLimitLock = Arc<Mutex<Option<Instant>>>;

pub fn check_rate_limit(rate_limit_lock: &RateLimitLock) {
    let lock = rate_limit_lock.lock().unwrap();
    if let Some(retry_after) = *lock {
        let now = Instant::now();
        if now < retry_after {
            let wait_time = retry_after.duration_since(now);
            thread::sleep(wait_time);
        }
    }
}

pub fn set_rate_limit(rate_limit_lock: &RateLimitLock, wait_duration: Option<Duration>) {
    let wait = wait_duration.unwrap_or(Duration::from_secs(DEFAULT_RATE_LIMIT_COOLOFF_SECS));

    let mut lock = rate_limit_lock.lock().unwrap();
    *lock = Some(Instant::now() + wait);
}
