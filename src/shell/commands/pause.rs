use std::{fs, path::PathBuf};

use thiserror::Error;

use crate::{
    shell::commands::{
        clock::read_clock,
        read_state::{LoadStateError, load_state},
    },
    timer,
};
pub struct PauseTimerOptions {
    /// Path to the state file
    pub state_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum PauseTimerError {
    #[error("failed to load state: {0}")]
    LoadState(#[from] LoadStateError),
    #[error("failed to get current time: {0}")]
    ReadClock(#[from] std::time::SystemTimeError),
    #[error("failed to write state file: {0}")]
    WriteState(std::io::Error),
}

pub fn pause_timer(opts: PauseTimerOptions) -> Result<(), PauseTimerError> {
    // Gather
    let state = load_state(&opts.state_path)?;
    let now = read_clock()?;

    // Decide
    let (state, outcome) = timer::pause(state, now);

    // Act
    fs::write(opts.state_path, String::from(state)).map_err(PauseTimerError::WriteState)?;

    match outcome {
        timer::PauseTimerOutcome::Paused => println!("Timer paused"),
        timer::PauseTimerOutcome::AlreadyPaused => println!("Timer already paused"),
        timer::PauseTimerOutcome::NotRunning => println!("Timer not running"),
    }

    Ok(())
}
