use clap::Args;

#[derive(Debug, Args)]
pub struct TickerArgs {
    ticker: String,
}

#[derive(Debug, Args)]
pub struct ModelArgs {
    model: String,
}

#[derive(Debug, Args)]
pub struct QueryArgs {
    pub query: String,
}
