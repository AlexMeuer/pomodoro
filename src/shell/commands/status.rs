use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::timer::state::{ParseStateError, State};

pub struct TimerStatusOptions {
    pub state_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum TimerStatusError {
    #[error("failed to read state file: {0}")]
    ReadState(std::io::Error),
    #[error("malformed state file: {0}")]
    MalformedState(#[from] ParseStateError),
    #[error("failed to get current time: {0}")]
    SystemClockRead(std::time::SystemTimeError),
}

pub fn print_timer_status(opts: TimerStatusOptions) -> Result<(), TimerStatusError> {
    let result = fs::read_to_string(opts.state_path);

    let state = match result {
        Ok(content) => content
            .parse::<State>()
            .map_err(TimerStatusError::MalformedState),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(State::default()),
            _ => Err(TimerStatusError::ReadState(e)),
        },
    }?;

    // We are in the shell, so it's legal to call SystemTime::now
    #[allow(clippy::disallowed_methods)]
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(TimerStatusError::SystemClockRead)?
        .as_secs();

    println!("{}", state.pretty_print(now));

    Ok(())
}
