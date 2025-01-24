use super::{
    errors::PumpFunProgramSignaturesError,
    program::get_pump_fun_program_address,
    signatures::{get_pump_fun_program_signatures, TransactionSignature},
};
use crate::{
    crawl_status::{
        channels::{create_crawl_status, mark_as_failed},
        errors::CrawlStatusQueryError,
        table::CrawlStatusOperation,
    },
    dragonfly::client::dragonfly_client,
    rpc::pool::RpcPoolManager,
    termination::{is_terminated, terminate, terminate_on_error, TerminationFlag},
    utils::log::log_time,
};
use crossbeam::channel::Sender;
use std::thread;

pub fn pump_fun_program_signatures_threads(
    pump_fun_program_signatures_tx: &Sender<TransactionSignature>,
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    rpc_pool_manager: &RpcPoolManager,
    termination_flag: &TerminationFlag,
) -> Vec<thread::JoinHandle<()>> {
    let concurrency = 1; // get_rpc_nodes_count();
    let mut handles = Vec::with_capacity(concurrency);

    let pump_fun_program_signatures_tx = pump_fun_program_signatures_tx.clone();
    let rpc_pool_manager = rpc_pool_manager.clone();

    let program_address = get_pump_fun_program_address();

    for thread_index in 0..concurrency {
        let log_tag = format!(
            "     {} pump program signatures #{} | ",
            log_time(),
            thread_index
        );

        let pump_fun_program_signatures_tx = pump_fun_program_signatures_tx.clone();
        let crawl_status_tx = crawl_status_tx.clone();
        let rpc_pool_manager = rpc_pool_manager.clone();
        let termination_flag = termination_flag.clone();
        let dragonfly_client = dragonfly_client();

        let handle = thread::spawn(move || loop {
            if is_terminated(&termination_flag) {
                println!("{} Termination flag set. Exiting", log_tag);
                break;
            }

            let result = get_pump_fun_program_signatures(
                &rpc_pool_manager,
                &dragonfly_client,
                &program_address,
                thread_index as u64,
            );

            match result {
                Ok(signatures_and_statuses) => {
                    for (signature, crawl_status) in signatures_and_statuses {
                        terminate_on_error(
                            &termination_flag,
                            pump_fun_program_signatures_tx.send(signature),
                        );
                        terminate_on_error(
                            &termination_flag,
                            create_crawl_status(&crawl_status_tx, crawl_status),
                        );
                    }
                }
                Err(PumpFunProgramSignaturesError::GetWindowConfigFailed(
                    CrawlStatusQueryError::HistoryComplete,
                )) => {
                    println!("{} History complete. Skipping...", log_tag);
                    continue;
                }
                Err(error) => {
                    println!(
                        "{} Error in pump fun program signatures thread: {}",
                        log_tag, error
                    );
                    terminate(&termination_flag);
                }
            }
        });

        handles.push(handle);
    }

    handles
}
