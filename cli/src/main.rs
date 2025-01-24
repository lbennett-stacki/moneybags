use app::{App, Parser};
use clickhouse::{
    client as clientCH, query as queryCH, remove as removeCH, start as startCH, stop as stopCH,
};
use commands::{
    ClickhouseCommand, CoinsCommand, Command, DevCommand, DragonflyCommand, ExtractCommand,
    ModelsCommand,
};
use dragonfly::{
    client as clientDragonfly, remove as removeDragonfly, start as startDragonfly,
    stop as stopDragonfly,
};
use std::error::Error;

mod app;
mod args;
mod clickhouse;
mod commands;
mod dragonfly;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to moneybags");

    let app = App::parse();

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
            stopCH();
            removeCH();
            startCH()
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Stop)) => stopCH(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Remove)) => {
            stopCH();
            removeCH()
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Client)) => clientCH(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Query(query_input))) => {
            queryCH(query_input.query)
        }

        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Start)) => {
            stopDragonfly();
            removeDragonfly();
            startDragonfly()
        }
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Stop)) => stopDragonfly(),
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Remove)) => {
            stopDragonfly();
            removeDragonfly()
        }
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Client)) => clientDragonfly(),
    }

    Ok(())
}
