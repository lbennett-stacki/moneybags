use crate::args::{ModelArgs, QueryArgs, TickerArgs};
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    Trade(TickerArgs),

    #[clap(subcommand)]
    Extract(ExtractCommand),

    #[clap(subcommand)]
    Models(ModelsCommand),

    #[clap(subcommand)]
    Coins(CoinsCommand),

    #[clap(subcommand)]
    Dev(DevCommand),
}

#[derive(Debug, Subcommand)]
pub enum ExtractCommand {
    Prices(TickerArgs),
    Socials(TickerArgs),
}

#[derive(Debug, Subcommand)]
pub enum ModelsCommand {
    Train(ModelArgs),
    Predict(ModelArgs),
}

#[derive(Debug, Subcommand)]
pub enum CoinsCommand {
    Get(TickerArgs),
    List,
}

#[derive(Debug, Subcommand)]
pub enum DevCommand {
    Start,
    Stop,
    Remove,
    UI,

    #[clap(subcommand)]
    Clickhouse(ClickhouseCommand),

    #[clap(subcommand)]
    Dragonfly(DragonflyCommand),
}

#[derive(Debug, Subcommand)]
pub enum ClickhouseCommand {
    Start,
    Stop,
    Remove,
    Client,
    Query(QueryArgs),
    UI,
}

#[derive(Debug, Subcommand)]
pub enum DragonflyCommand {
    Start,
    Stop,
    Remove,
    Client,
    UI,
}
