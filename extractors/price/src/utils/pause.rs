use crate::client::{IS_ONE_TOKEN_AT_A_TIME, WAIT_FOR_UNPAUSE_MS};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub type PauseSignal = Arc<AtomicUsize>;

pub fn pause(signal: &PauseSignal) {
    signal.fetch_add(1, Ordering::SeqCst);
}

pub fn unpause(signal: &PauseSignal) {
    signal.fetch_sub(1, Ordering::SeqCst);
}

pub fn wait_for_unpause(signal: &PauseSignal) {
    if !IS_ONE_TOKEN_AT_A_TIME {
        return;
    }

    while signal.load(Ordering::SeqCst) > 0 {
        thread::sleep(Duration::from_millis(WAIT_FOR_UNPAUSE_MS));
    }
}
