use std::{fs, path::PathBuf};

use thiserror::Error;

use crate::{
    shell::commands::{
        clock::read_clock,
        read_state::{LoadStateError, load_state},
    },
    timer,
};
pub struct StopTimerOptions {
    /// Path to the state file
    pub state_path: PathBuf,
    pub delete_state_file: bool,
}

#[derive(Error, Debug)]
pub enum StopTimerError {
    #[error("failed to load state: {0}")]
    Load(#[from] LoadStateError),
    #[error("failed to get current time: {0}")]
    ReadClock(#[from] std::time::SystemTimeError),
    #[error("failed to write state file: {0}")]
    Write(std::io::Error),
    #[error("failed to remove state file: {0}")]
    Remove(std::io::Error),
}

pub fn stop_timer(opts: StopTimerOptions) -> Result<(), StopTimerError> {
    // Gather
    let state = load_state(&opts.state_path)?;
    let now = read_clock()?;

    // Decide
    let (persist, outcome) = timer::stop(state, opts.delete_state_file, now);

    // Act
    match persist {
        timer::Persist::Persist(state) => {
            fs::write(opts.state_path, String::from(state)).map_err(StopTimerError::Write)?;
        }
        timer::Persist::Delete => match fs::remove_file(opts.state_path) {
            Ok(()) => {}
            Err(err) => {
                // We want to delete the file anyway, so who cares
                // if it doesn't exist.
                if err.kind() != std::io::ErrorKind::NotFound {
                    return Err(StopTimerError::Remove(err));
                }
            }
        },
    }

    match outcome {
        timer::StopTimerOutcome::NotRunning => println!("Timer not running"),
        timer::StopTimerOutcome::Stopped => println!("Timer stopped"),
    }

    Ok(())
}
