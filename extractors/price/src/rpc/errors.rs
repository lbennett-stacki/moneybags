use solana_client::client_error::ClientError;

#[derive(Debug)]
pub enum RpcError {
    ClientError(ClientError),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for RpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ClientError(err) => Some(err),
            _ => None,
        }
    }
}
