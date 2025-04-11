use ratatui;
use clap::{Parser, Subcommand};
use std::error::Error;

mod hexclock;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Hexclock {
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::Hexclock {} => {}
    }
    Ok(())
}
