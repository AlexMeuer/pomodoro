use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::{
    shell::commands::{
        pause::{PauseTimerError, PauseTimerOptions, pause_timer},
        start::{StartTimerError, StartTimerOptions, start_timer},
        status::{TimerStatusError, TimerStatusOptions, print_timer_status},
        stop::{StopTimerError, StopTimerOptions, stop_timer},
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

        /// Start the timer, regardless of current state.
        #[arg(long)]
        force: bool,
    },
    Status,
    Pause,
    Stop {
        /// Deletes the state file.
        #[arg(long)]
        delete: bool,
    },
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to place state file: {0}")]
    PlaceStateFile(std::io::Error),
    #[error("Error starting timer: {0}")]
    Start(#[from] StartTimerError),
    #[error("Error reading timer status: {0}")]
    Status(#[from] TimerStatusError),
    #[error("Error pausing timer: {0}")]
    Pause(#[from] PauseTimerError),
    #[error("Error stopping timer: {0}")]
    Stop(#[from] StopTimerError),
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
        Commands::Start { duration, force } => start_timer(StartTimerOptions {
            duration,
            state_path,
            force,
        })
        .map_err(Into::into),
        Commands::Status => {
            print_timer_status(&TimerStatusOptions { state_path }).map_err(Into::into)
        }
        Commands::Pause => pause_timer(PauseTimerOptions { state_path }).map_err(Into::into),
        Commands::Stop { delete } => stop_timer(StopTimerOptions {
            state_path,
            delete_state_file: delete,
        })
        .map_err(Into::into),
    }
}
