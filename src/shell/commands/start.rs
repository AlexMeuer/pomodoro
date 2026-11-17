use std::{fs, path::PathBuf};
use thiserror::Error;

use crate::timer::{self, state::State};

pub struct StartTimerOptions {
    pub duration_secs: u64,
    /// Path to the state file
    pub state: Option<PathBuf>,
}

#[derive(Error, Debug)]
pub enum StartTimerError {
    #[error("failed to read state file: {0}")]
    ReadState(std::io::Error),
    #[error("failed to get current time: {0}")]
    SystemClockRead(std::time::SystemTimeError),
    #[error("failed to write state file: {0}")]
    WriteState(std::io::Error),
}

pub fn start_timer(opts: StartTimerOptions) -> Result<(), StartTimerError> {
    let path = opts.state.unwrap_or_else(|| PathBuf::from(r"./state"));

    let result = fs::read_to_string(&path).map(|str| {
        str.parse::<State>().unwrap_or_else(|e| {
            eprintln!("WARN: bad state file: {e}");
            State::default()
        })
    });

    let state = match result {
        Ok(state) => state,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => State::default(),
            _ => return Err(StartTimerError::ReadState(e)),
        },
    };

    // We are in the shell, so it's legal to call SystemTime::now
    #[allow(clippy::disallowed_methods)]
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(StartTimerError::SystemClockRead)?
        .as_secs();

    let (state, outcome) = timer::start(state, opts.duration_secs, now);

    fs::write(path, String::from(state)).map_err(StartTimerError::WriteState)?;

    match outcome {
        timer::StartTimerOutcome::Started => println!("Timer started"),
        timer::StartTimerOutcome::Resumed => {
            println!("Existing paused timer resume | new timer not started");
        }
        timer::StartTimerOutcome::AlreadyRunning => {
            println!("Timer already running | no action taken");
        }
    }

    Ok(())
}
