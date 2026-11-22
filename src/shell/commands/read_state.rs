use crate::timer::state::{ParseStateError, State};
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadStateError {
    #[error("failed to read state file: {0}")]
    Read(std::io::Error),
    #[error("malformed state file: {0}")]
    Malformed(ParseStateError),
}

pub fn load_state(path: &PathBuf) -> Result<State, LoadStateError> {
    let result = fs::read_to_string(path);

    match result {
        Ok(content) => content.parse::<State>().map_err(LoadStateError::Malformed),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(State::default()),
            _ => Err(LoadStateError::Read(e)),
        },
    }
}
