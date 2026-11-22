use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{
    shell::commands::{
        start::{StartTimerError, StartTimerOptions, start_timer},
        status::{TimerStatusError, TimerStatusOptions, print_timer_status},
    },
    timer,
};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom state file
    #[arg(short, long, value_name = "FILE")]
    state: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        /// The length of the timer.
        /// Accepts a number, which is interpreted as seconds.
        /// Also accepts the format `<minutes>m` or `<seconds>s`
        /// but not a combination of both.
        duration: timer::Duration,
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
            duration,
            state_path,
        })
        .map_err(Into::into),
        Commands::Status => {
            print_timer_status(&TimerStatusOptions { state_path }).map_err(Into::into)
        }
    }
}
