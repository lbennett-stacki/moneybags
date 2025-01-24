use crate::crawl_status::table::CrawlStatusOperation;
use crossbeam::channel::SendError;
use time::error::ComponentRange;

#[derive(Debug)]
pub enum TradeCrawlError {
    AlreadyCrawled,
    CrawlStatusSend(SendError<CrawlStatusOperation>),
    TransactionFailed,
    TransactionFetchFailed,
    TransactionMessageParseFailed,
    BlockTimeParseError(ComponentRange),
}

impl std::fmt::Display for TradeCrawlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyCrawled => write!(f, "Transaction already crawled"),
            Self::CrawlStatusSend(err) => write!(f, "Failed to send crawl status: {}", err),
            Self::TransactionFailed => write!(f, "Transaction failed"),
            Self::TransactionFetchFailed => write!(f, "Failed to fetch transaction"),
            Self::TransactionMessageParseFailed => write!(f, "Failed to parse transaction message"),
            Self::BlockTimeParseError(err) => write!(f, "Failed to parse block time: {}", err),
        }
    }
}

impl std::error::Error for TradeCrawlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CrawlStatusSend(err) => Some(err),
            Self::BlockTimeParseError(err) => Some(err),
            _ => None,
        }
    }
}
