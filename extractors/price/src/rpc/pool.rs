use super::{clients::build_rpc_client_states, errors::RpcError};
use crate::{rpc::clients::build_rpc_client, utils::log::log_time};
use solana_client::{
    client_error::{reqwest::StatusCode, ClientError, ClientErrorKind},
    rpc_client::RpcClient,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::{Duration, Instant},
};

pub const DEFAULT_RATE_LIMIT_COOLOFF_MS: u64 = 1000;

#[derive(Clone, Debug)]
pub struct RpcClientState {
    pub url: String,
    pub rate_limited_until: Option<Instant>,
    pub requests_per_second: u64,
    pub recent_requests: Vec<Instant>,
}

impl RpcClientState {
    pub fn new(url: &str, requests_per_second: u64) -> Self {
        Self {
            url: url.to_string(),
            requests_per_second,
            rate_limited_until: None,
            recent_requests: Vec::new(),
        }
    }

    pub fn is_available(&self) -> bool {
        if let Some(until) = self.rate_limited_until {
            if Instant::now() < until {
                return false;
            }
        }

        let one_second_ago = Instant::now() - Duration::from_secs(1);
        let current_rps = self
            .recent_requests
            .iter()
            .filter(|&&timestamp| timestamp > one_second_ago)
            .count() as u64;

        current_rps < self.requests_per_second
    }

    fn record_request(&mut self) {
        let now = Instant::now();

        self.recent_requests
            .retain(|&timestamp| now - timestamp <= Duration::from_secs(1));
        self.recent_requests.push(now);

        let current_rps = self.recent_requests.len() as u64;
        if current_rps >= self.requests_per_second {
            self.rate_limited_until =
                Some(now + Duration::from_millis(DEFAULT_RATE_LIMIT_COOLOFF_MS));
        }
    }
}

#[derive(Clone)]
pub struct RpcPoolManager {
    clients: Arc<Mutex<HashMap<String, RpcClientState>>>,
    rate_limit_duration: Duration,
    request_counter: Arc<AtomicU64>,
}

impl RpcPoolManager {
    pub fn new(rate_limit_duration: Duration) -> Self {
        let clients = build_rpc_client_states();

        Self {
            clients: Arc::new(Mutex::new(clients)),
            rate_limit_duration,
            request_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get_available_client(&self, pin: Option<u64>) -> Option<String> {
        let mut clients = self.clients.lock().unwrap();

        if let Some(pin) = pin {
            let client = clients.get_mut(&pin.to_string());
            if let Some(client) = client {
                client.record_request();
                println!(
                    "{} FOUND PIN SO RETURNING PINNED CLIENT {} | ",
                    log_time(),
                    client.url
                );
                return Some(client.url.clone());
            }
        }

        let available = clients
            .values_mut()
            .filter(|client| client.is_available())
            .map(|client| {
                client.record_request();
                client.url.clone()
            })
            .collect::<Vec<String>>();

        if available.is_empty() {
            return None;
        }

        let next_count = self.request_counter.fetch_add(1, Ordering::SeqCst);

        available
            .get(next_count as usize % available.len())
            .cloned()
    }

    pub fn mark_rate_limited(&self, url: &str) {
        let mut clients = self.clients.lock().unwrap();
        if let Some(client) = clients.get_mut(url) {
            client.rate_limited_until = Some(Instant::now() + self.rate_limit_duration);
        }
    }

    pub fn execute<F, T>(&self, operation: F, pin: Option<u64>) -> Result<T, RpcError>
    where
        F: FnOnce(&RpcClient) -> Result<T, ClientError> + Clone,
    {
        let op = operation.clone();

        let client = self.get_available_client(pin);

        let log_tag = format!("{} execute rpc op | ", log_time());

        if client.is_none() {
            println!(
                "{} No available clients at all. Sleeping until available...",
                log_tag
            );
            self.sleep_until_available();
            return self.execute(op, pin);
        }
        let client_url = client.unwrap();

        let log_tag = format!("{} client {} | ", log_tag, client_url);

        println!("{} Sending request...", log_tag);
        match operation(&build_rpc_client(client_url.clone())) {
            Ok(result) => Ok(result),
            Err(error) => {
                if let ClientError {
                    request: _,
                    kind: ClientErrorKind::Reqwest(ref reqwest_err),
                } = error
                {
                    if let Some(status) = reqwest_err.status() {
                        if status == StatusCode::TOO_MANY_REQUESTS {
                            self.mark_rate_limited(&client_url);

                            let next_available_client = self.get_available_client(pin);
                            if next_available_client.is_some() {
                                return self.execute(op, pin);
                            }

                            self.sleep_until_available();
                            return self.execute(op, pin);
                        }
                    }
                }

                Err(RpcError::ClientError(error))
            }
        }
    }

    fn sleep_until_available(&self) {
        let clients = self.clients.lock().unwrap();

        let rate_limited_until = clients
            .values()
            .map(|client| client.rate_limited_until)
            .min();

        if let Some(Some(rate_limited_until)) = rate_limited_until {
            println!(
                "All clients are rate limited. Waiting for minimum limit duration of {} seconds...",
                rate_limited_until.duration_since(Instant::now()).as_secs()
            );
            sleep(rate_limited_until - Instant::now());
        } else {
            let fallback_duration = Duration::from_secs(DEFAULT_RATE_LIMIT_COOLOFF_MS);
            println!(
                "All clients are rate limited. Waiting for fall back {} seconds...",
                fallback_duration.as_secs()
            );
            sleep(fallback_duration);
        }
    }
}
