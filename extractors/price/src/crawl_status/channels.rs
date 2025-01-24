use super::table::{CrawlStatusOperation, CrawlStatusRow};
use crate::trades::errors::TradeCrawlError;
use crossbeam::channel::Sender;

pub fn mark_as_failed(
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    token_tx_signature: &str,
    error: &str,
) -> Result<(), TradeCrawlError> {
    crawl_status_tx
        .send(CrawlStatusOperation::MarkAsFailed(
            token_tx_signature.to_string(),
            error.to_string(),
        ))
        .map_err(TradeCrawlError::CrawlStatusSend)
}

pub fn mark_as_succeeded(
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    token_tx_signature: &str,
) -> Result<(), TradeCrawlError> {
    crawl_status_tx
        .send(CrawlStatusOperation::MarkAsSucceeded(
            token_tx_signature.to_string(),
        ))
        .map_err(TradeCrawlError::CrawlStatusSend)
}

pub fn create_crawl_status(
    crawl_status_tx: &Sender<CrawlStatusOperation>,
    crawl_status: CrawlStatusRow,
) -> Result<(), TradeCrawlError> {
    crawl_status_tx
        .send(CrawlStatusOperation::Create(crawl_status))
        .map_err(TradeCrawlError::CrawlStatusSend)
}
