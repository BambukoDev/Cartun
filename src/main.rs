use ratatui;
use crossterm::event::{self, Event};
use clap::{Parser, Subcommand};
use std::{error::Error, time::Duration};
use chrono::prelude::*;

mod hexclock;
mod shared;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(long, short, action = clap::ArgAction::SetTrue , help = "Print version info")]
    version: bool
}

#[derive(Subcommand)]
enum Command {
    Hexclock {
        #[arg(long, help = "Sets the background color of the clock")]
        bg_color: Option<String>,
        #[arg(long, help = "Toggle between simple and big ASCII clock")]
        big: Option<bool>
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    fn module(cmd: Option<Command>) -> (Option<Box<dyn Fn(&mut ratatui::Frame)>>, Option<Duration>) {
        match cmd {
            None => {
                return (None, None);
            }
            Some(Command::Hexclock {bg_color, big}) => {
                return (
                    Some(Box::new(
                        move |frame: &mut ratatui::Frame| {
                            let now = Local::now();
                            hexclock::bg(frame, now);
                            if big.unwrap_or(false) {
                                hexclock::clock_big(frame, now);
                            } else {
                                hexclock::clock_normal(frame, now);
                            }
                        }
                    )),
                    Some(Duration::from_millis(50))
                );
            }
        }
    }

    let args = Args::parse();
    let mut terminal = Box::new(ratatui::init());
    let mut render: Option<Box<dyn Fn(&mut ratatui::Frame)>>;
    let mut delay: Option<Duration>;
    (render, delay) = module(args.command);

    if args.version {
        print!("Cartun version {}\n", env!("CARGO_PKG_VERSION"));
        ratatui::restore();
        return Ok(());
    }

    if render.is_none() { // TODO
        (render, delay) = module(None);
    }

    if render.is_some() {
        let delay = delay.unwrap_or(Duration::from_millis(10));
        loop {
            unsafe {
                terminal.draw(render.as_ref().unwrap_unchecked())?;
            }
            match event::poll(delay) {
                Ok(true) => {
                    if matches!(event::read()?, Event::Key(_)) {
                        break;
                    }
                }
                Ok(false) => {}
                _ => {
                    break;
                }
            }

        }
    }
    ratatui::restore();

    Ok(())
}
