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
    command: Option<Command>
}

#[derive(Subcommand)]
enum Command {
    Hexclock
}

fn main() -> Result<(), Box<dyn Error>> {
    fn module(cmd: Option<Command>) -> (Option<Box<dyn Fn(&mut ratatui::Frame)>>, Option<Duration>) {
        match cmd {
            None => {
                return (None, None);
            }
            Some(Command::Hexclock) => {
                return (
                    Some(Box::new(
                        |frame: &mut ratatui::Frame| {
                            let now = Local::now();
                            hexclock::bg(frame, now);
                            hexclock::clock_1(frame, now);
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
