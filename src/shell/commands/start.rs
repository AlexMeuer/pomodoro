use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::{
    shell::commands::{
        clock::read_clock,
        read_state::{LoadStateError, load_state},
    },
    timer,
};

pub struct StartTimerOptions {
    pub duration: timer::Duration,
    /// Path to the state file
    pub state_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum StartTimerError {
    #[error("failed to load state: {0}")]
    LoadState(#[from] LoadStateError),
    #[error("failed to get current time: {0}")]
    ReadClock(#[from] std::time::SystemTimeError),
    #[error("failed to write state file: {0}")]
    WriteState(std::io::Error),
}

pub fn start_timer(opts: StartTimerOptions) -> Result<(), StartTimerError> {
    // Gather
    let state = load_state(&opts.state_path)?;
    let now = read_clock()?;

    // Decide
    let (state, outcome) = timer::start(state, opts.duration, now);

    // Act
    fs::write(opts.state_path, String::from(state)).map_err(StartTimerError::WriteState)?;

    match outcome {
        timer::StartTimerOutcome::Started => println!("Timer started"),
        timer::StartTimerOutcome::Resumed => {
            println!("Existing paused timer resumed: new timer not started");
        }
        timer::StartTimerOutcome::AlreadyRunning => {
            println!("Timer already running: no action taken");
        }
    }

    Ok(())
}
