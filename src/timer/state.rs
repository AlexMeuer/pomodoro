use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

use crate::timer::Duration;

/// The state of a timer.
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum State {
    #[default]
    Idle,
    Running {
        /// The timestamp at which the timer will end, in seconds since the Unix epoch.
        ends_at: u64,
    },
    Paused {
        remaining: Duration,
    },
}

impl State {
    #[must_use]
    pub const fn recalculate(self, now: u64) -> Self {
        match self {
            Self::Running { ends_at } if now >= ends_at => Self::Idle,
            Self::Running { ends_at } => Self::Running { ends_at },
            Self::Paused {
                remaining: Duration::ZERO,
            } => Self::Idle,
            Self::Idle | Self::Paused { .. } => self,
        }
    }
    pub fn pretty_print(&self, now: u64) -> String {
        match self.recalculate(now) {
            Self::Idle => "Idle".to_string(),
            Self::Running { ends_at } => format!("{}", Duration::secs(ends_at.saturating_sub(now))),
            Self::Paused { remaining } => {
                format!("Paused ({remaining})")
            }
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseStateError {
    #[error("input is empty")]
    EmptyInput,
    #[error("status is running but value for ends_at is missing")]
    RunningButMissingEndsAt,
    #[error("status is running but value for ends_at is malformed: {0}")]
    RunningButMalformedEndsAt(ParseIntError),
    #[error("status is paused but value for remaining_seconds is missing")]
    PausedButMissingRemainingSeconds,
    #[error("status is paused but value for remaining_seconds is malformed: {0}")]
    PausedButMalformedRemainingSeconds(ParseIntError),
    #[error("invalid status: {0}")]
    InvalidStatus(String),
}

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(' ');

        match iter.next() {
            None | Some("") => Err(ParseStateError::EmptyInput),
            Some("idle") => Ok(Self::Idle),
            Some("running") => {
                let ends_at =
                    iter.next()
                        .map_or(Err(ParseStateError::RunningButMissingEndsAt), |s| {
                            s.parse::<u64>()
                                .map_err(ParseStateError::RunningButMalformedEndsAt)
                        })?;
                Ok(Self::Running { ends_at })
            }
            Some("paused") => {
                let remaining_secs = iter.next().map_or(
                    Err(ParseStateError::PausedButMissingRemainingSeconds),
                    |s| {
                        s.parse::<u64>()
                            .map_err(ParseStateError::PausedButMalformedRemainingSeconds)
                    },
                )?;
                Ok(Self::Paused {
                    remaining: Duration::secs(remaining_secs),
                })
            }
            Some(unknown) => Err(ParseStateError::InvalidStatus(unknown.to_string())),
        }
    }
}

impl From<State> for String {
    fn from(val: State) -> Self {
        match val {
            State::Idle => "idle".to_string(),
            State::Running { ends_at } => format!("running {ends_at}"),
            State::Paused { remaining } => format!("paused {}", remaining.as_secs()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(State::Idle, "idle")]
    #[case(State::Running { ends_at: 10 }, "running 10")]
    #[case(State::Paused { remaining: Duration::secs(5) }, "paused 5")]
    fn converts_to_string(#[case] state: State, #[case] expected: String) {
        assert_eq!(String::from(state), expected);
    }

    #[rstest]
    #[case("", Err(ParseStateError::EmptyInput))]
    #[case("idle", Ok(State::Idle))]
    #[case("running 10", Ok(State::Running { ends_at: 10 }))]
    #[case("paused 5", Ok(State::Paused { remaining: Duration::secs(5) }))]
    #[case("foobar 5", Err(ParseStateError::InvalidStatus("foobar".to_string())))]
    fn parses_from_string(#[case] input: &str, #[case] expected: Result<State, ParseStateError>) {
        assert_eq!(input.parse::<State>(), expected);
    }

    #[rstest]
    #[case(Duration::secs(0), "0s")]
    #[case(Duration::secs(60), "60s")]
    #[case(Duration::secs(61), "2m")]
    #[case(Duration::secs(120), "2m")]
    #[case(Duration::secs(121), "3m")]
    #[case(Duration::secs(1500), "25m")]
    fn formatting_returns_expected(#[case] duration: Duration, #[case] expected: String) {
        assert_eq!(format!("{duration}"), expected);
    }
}
