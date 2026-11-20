use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::shell::commands::{
    start::{StartTimerError, StartTimerOptions, start_timer},
    status::{TimerStatusError, TimerStatusOptions, print_timer_status},
};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Sets a custom state file
    #[arg(short, long, value_name = "FILE")]
    state: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        // TODO: parse human-readable durations?
        /// The length of the timer, in seconds.
        duration: u64,
    },
    Status,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to place state file | {0}")]
    PlaceStateFile(std::io::Error),
    #[error("Error starting timer | {0}")]
    Start(#[from] StartTimerError),
    #[error("Error reading timer status | {0}")]
    Status(#[from] TimerStatusError),
}

pub fn run() -> Result<(), CommandError> {
    let cli = Cli::parse();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    let state_path = cli.state.map_or_else(
        || {
            xdg::BaseDirectories::with_prefix("pomodoro")
                .place_state_file("state")
                .map_err(CommandError::PlaceStateFile)
        },
        Ok,
    )?;

    match cli.command {
        Commands::Start { duration } => start_timer(StartTimerOptions {
            duration_secs: duration,
            state_path,
        })
        .map_err(Into::into),
        Commands::Status => {
            print_timer_status(TimerStatusOptions { state_path }).map_err(Into::into)
        }
    }
}
