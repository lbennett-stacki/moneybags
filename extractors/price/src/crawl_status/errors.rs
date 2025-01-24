use redis::RedisError;

#[derive(Debug, PartialEq)]
pub enum CrawlStatusQueryError {
    HistoryComplete,
    Redis(RedisError),
}

impl std::fmt::Display for CrawlStatusQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HistoryComplete => write!(f, "History complete"),
            Self::Redis(err) => write!(f, "Redis error: {}", err),
        }
    }
}

impl std::error::Error for CrawlStatusQueryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Redis(err) => Some(err),
            _ => None,
        }
    }
}
