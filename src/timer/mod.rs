use crate::timer::state::State;

pub mod state;

#[derive(Debug, PartialEq, Eq)]
pub enum StartTimerOutcome {
    Started,
    Resumed,
    AlreadyRunning,
}

/// Starts a timer if it is not already running.
/// Consumes the given state and returns a new state.
#[allow(clippy::needless_pass_by_value)]
pub const fn start(state: State, duration_secs: u64, now: u64) -> (State, StartTimerOutcome) {
    match state {
        running @ State::Running { .. } => (running, StartTimerOutcome::AlreadyRunning),
        State::Idle => (
            State::Running {
                ends_at: now.saturating_add(duration_secs),
            },
            StartTimerOutcome::Started,
        ),
        State::Paused { remaining_secs } => (
            State::Running {
                ends_at: now.saturating_add(remaining_secs),
            },
            StartTimerOutcome::Resumed,
        ),
    }
}
