use super::signatures::TransactionSignature;
use crate::{
    crawl_status::{errors::CrawlStatusQueryError, table::CrawlStatusOperation},
    rpc::errors::RpcError,
};
use crossbeam::channel::SendError;

#[derive(Debug)]
pub enum PumpFunProgramSignaturesError {
    SendCrawlStatus(SendError<CrawlStatusOperation>),
    SendTransactionSignature(SendError<TransactionSignature>),
    GetWindowConfigFailed(CrawlStatusQueryError),
    GetSignaturesFailed(RpcError),
}

impl std::fmt::Display for PumpFunProgramSignaturesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendCrawlStatus(err) => write!(f, "Failed to send crawl status: {}", err),
            Self::SendTransactionSignature(err) => {
                write!(f, "Failed to send transaction signature: {}", err)
            }
            Self::GetWindowConfigFailed(err) => write!(f, "Failed to get window config: {}", err),
            Self::GetSignaturesFailed(err) => write!(f, "Failed to get signatures: {}", err),
        }
    }
}

impl std::error::Error for PumpFunProgramSignaturesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SendCrawlStatus(err) => Some(err),
            Self::SendTransactionSignature(err) => Some(err),
            Self::GetWindowConfigFailed(err) => Some(err),
            Self::GetSignaturesFailed(err) => Some(err),
        }
    }
}
