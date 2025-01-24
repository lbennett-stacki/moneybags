use app::{App, Parser};
use clickhouse::{client, query, remove, start, stop};
use commands::{
    ClickhouseCommand, CoinsCommand, Command, DevCommand, ExtractCommand, ModelsCommand,
};
use std::error::Error;

mod app;
mod args;
mod clickhouse;
mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to moneybags");

    let app = App::parse();

    println!("Well..... {:?}", app);

    match app.command {
        Command::Extract(ExtractCommand::Prices(ticker)) => {
            println!("Extracting prices {:?}", ticker)
        }
        Command::Extract(ExtractCommand::Socials(ticker)) => {
            println!("Extracting socials {:?}", ticker)
        }

        Command::Models(ModelsCommand::Train(model)) => println!("Models train {:?}", model),
        Command::Models(ModelsCommand::Predict(model)) => println!("Models predict {:?}", model),

        Command::Coins(CoinsCommand::Get(ticker)) => println!("Coining get {:?}", ticker),
        Command::Coins(CoinsCommand::List) => println!("Coining list"),

        Command::Trade(ticker) => println!("Trading {:?}", ticker),

        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Start)) => {
            stop();
            remove();
            start()
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Stop)) => stop(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Remove)) => {
            stop();
            remove()
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Client)) => client(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Query(query_input))) => {
            query(query_input.query)
        }
    }

    Ok(())
}
