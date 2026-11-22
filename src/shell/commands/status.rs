use std::path::PathBuf;
use thiserror::Error;

use crate::shell::commands::{
    clock::read_clock,
    read_state::{LoadStateError, load_state},
};

pub struct TimerStatusOptions {
    pub state_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum TimerStatusError {
    #[error("failed to load state: {0}")]
    LoadState(#[from] LoadStateError),
    #[error("failed to get current time: {0}")]
    ReadClock(#[from] std::time::SystemTimeError),
}

pub fn print_timer_status(opts: &TimerStatusOptions) -> Result<(), TimerStatusError> {
    // Gather
    let state = load_state(&opts.state_path)?;
    let now = read_clock()?;

    // Decide
    let pretty = state.pretty_print(now);

    // Act
    println!("{pretty}");

    Ok(())
}
