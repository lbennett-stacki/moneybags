use app::{App, Parser};
use clickhouse::{
    client as client_clickhouse, open_ui as open_ui_clickhouse, query as query_clickhouse,
    remove_safe as remove_safe_clickhouse, start_safe as start_safe_clickhouse,
    stop as stop_clickhouse,
};
use commands::{
    ClickhouseCommand, CoinsCommand, Command, DevCommand, DragonflyCommand, ExtractCommand,
    ModelsCommand,
};
use dragonfly::{
    client as client_dragonfly, open_ui as open_ui_dragonfly, remove_safe as remove_safe_dragonfly,
    start_safe as start_safe_dragonfly, stop as stop_dragonfly,
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

        Command::Dev(DevCommand::Start) => {
            start_safe_clickhouse();
            start_safe_dragonfly();
        }
        Command::Dev(DevCommand::Stop) => {
            stop_clickhouse();
            stop_dragonfly();
        }
        Command::Dev(DevCommand::Remove) => {
            remove_safe_clickhouse();
            remove_safe_dragonfly();
        }
        Command::Dev(DevCommand::UI) => {
            open_ui_clickhouse();
            open_ui_dragonfly();
        }

        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Start)) => {
            start_safe_clickhouse();
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Stop)) => stop_clickhouse(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Remove)) => remove_safe_clickhouse(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Client)) => client_clickhouse(),
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::Query(query_input))) => {
            query_clickhouse(query_input.query)
        }
        Command::Dev(DevCommand::Clickhouse(ClickhouseCommand::UI)) => open_ui_clickhouse(),

        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Start)) => start_safe_dragonfly(),
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Stop)) => stop_dragonfly(),
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Remove)) => remove_safe_dragonfly(),
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::Client)) => client_dragonfly(),
        Command::Dev(DevCommand::Dragonfly(DragonflyCommand::UI)) => open_ui_dragonfly(),
    }

    Ok(())
}
