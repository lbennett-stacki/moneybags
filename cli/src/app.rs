use crate::commands::Command;

pub use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "moneybags-cli", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}
