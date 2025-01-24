use crate::{crawl_status::table::CrawlStatusOperation, rpc::errors::RpcError};
use crossbeam::channel::SendError;
use time::error::ComponentRange;

#[derive(Debug)]
pub enum PumpFunTokenCrawlError {
    AlreadyCrawled,
    CrawlStatusSend(SendError<CrawlStatusOperation>),
    TransactionFailed,
    TransactionFetchFailed(RpcError),
    BlockTimeParseError(ComponentRange),
    TransactionMessageParseFailed,
    TokenNotFound,
}

impl std::fmt::Display for PumpFunTokenCrawlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCrawled => write!(f, "Already crawled"),
            Self::CrawlStatusSend(err) => write!(f, "Crawl status send error: {}", err),
            Self::TransactionFailed => write!(f, "Transaction failed"),
            Self::TransactionFetchFailed(err) => write!(f, "Transaction fetch failed: {}", err),
            Self::BlockTimeParseError(err) => write!(f, "Block time parse error: {}", err),
            Self::TransactionMessageParseFailed => write!(f, "Transaction message parse failed"),
            Self::TokenNotFound => write!(f, "Token not found"),
        }
    }
}

impl std::error::Error for PumpFunTokenCrawlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CrawlStatusSend(err) => Some(err),
            Self::TransactionFetchFailed(err) => Some(err),
            _ => None,
        }
    }
}
