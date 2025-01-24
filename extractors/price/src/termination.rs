use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::utils::log::log_time;

pub type TerminationFlag = Arc<AtomicBool>;

pub fn init() -> TerminationFlag {
    TerminationFlag::new(AtomicBool::new(false))
}

pub fn terminate(terminate_flag: &TerminationFlag) -> ! {
    terminate_flag.store(true, Ordering::Relaxed);
    process::exit(1);
}

pub fn is_terminated(terminate_flag: &TerminationFlag) -> bool {
    terminate_flag.load(Ordering::Relaxed)
}

pub fn terminate_on_error<T, E>(termination_flag: &TerminationFlag, result: Result<T, E>) -> T
where
    E: std::fmt::Debug,
{
    if let Err(error) = result {
        println!("{} Error: {:?}", log_time(), error);
        terminate(&termination_flag);
    } else {
        result.unwrap()
    }
}
