use crate::timer::state::State;

mod duration;
pub mod state;
pub use duration::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum StartTimerOutcome {
    Started,
    Resumed,
    AlreadyRunning,
}

/// Starts a timer if it is not already running.
/// Consumes the given state and returns a new state.
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn start(state: State, duration: Duration, now: u64) -> (State, StartTimerOutcome) {
    match state {
        State::Running { ends_at } if now >= ends_at => start(State::Idle, duration, now),
        running @ State::Running { .. } => (running, StartTimerOutcome::AlreadyRunning),
        State::Idle => (
            State::Running {
                ends_at: now.saturating_add(duration.into()),
            },
            StartTimerOutcome::Started,
        ),
        State::Paused {
            remaining: Duration::ZERO,
        } => start(State::Idle, duration, now),
        State::Paused { remaining } => (
            State::Running {
                ends_at: now.saturating_add(remaining.into()),
            },
            StartTimerOutcome::Resumed,
        ),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PauseTimerOutcome {
    Paused,
    AlreadyPaused,
    NotRunning,
}

/// Pauses a timer if it is running.
/// Consumes the given state and returns a new state.
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub const fn pause(state: State, now: u64) -> (State, PauseTimerOutcome) {
    match state {
        idle @ State::Idle => (idle, PauseTimerOutcome::NotRunning),
        State::Running { ends_at } if now >= ends_at => pause(State::Idle, now),
        State::Running { ends_at } => (
            State::Paused {
                remaining: Duration::secs(ends_at.saturating_sub(now)),
            },
            PauseTimerOutcome::Paused,
        ),
        paused @ State::Paused { .. } => (paused, PauseTimerOutcome::AlreadyPaused),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PauseTimerOutcome as PTO;
    use StartTimerOutcome as STO;
    use State as S;
    use rstest::rstest;
    const NOW: u64 = 1_794_000_000;

    #[rstest]
    //      intitial state                      duration_secs       expected_state              expected_outcome
    #[case(S::Idle,                                 0,          S::Running { ends_at: NOW },        STO::Started)]
    #[case(S::Idle,                                 20,         S::Running { ends_at: NOW + 20},    STO::Started)]
    #[case(S::Idle,                                 u64::MAX,   S::Running { ends_at: u64::MAX },   STO::Started)]
    #[case(S::Running { ends_at: NOW - 1},          20,         S::Running { ends_at: NOW + 20},    STO::Started)]
    #[case(S::Running { ends_at: NOW},              20,         S::Running { ends_at: NOW + 20 },   STO::Started)]
    #[case(S::Running { ends_at: NOW + 1},          20,         S::Running { ends_at: NOW + 1},     STO::AlreadyRunning)]
    #[case(S::Paused { remaining: Duration::secs(0)},          20,         S::Running { ends_at: NOW + 20 },   STO::Started)]
    #[case(S::Paused { remaining: Duration::secs(30)},         20,         S::Running { ends_at: NOW + 30 },   STO::Resumed)]
    #[case(S::Paused { remaining: Duration::secs(u64::MAX)},   20,         S::Running { ends_at: u64::MAX },   STO::Resumed)]
    fn starting_returns_expected(
        #[case] initial: State,
        #[case] duration_secs: u64,
        #[case] expected_state: State,
        #[case] expected_outcome: StartTimerOutcome,
    ) {
        let (new_state, outcome) = start(initial, Duration::secs(duration_secs), NOW);
        assert_eq!(new_state, expected_state);
        assert_eq!(outcome, expected_outcome);
    }

    #[rstest]
    #[case(S::Idle, S::Idle, PTO::NotRunning)]
    fn pausing_returns_expected(
        #[case] initial: State,
        #[case] expected_state: State,
        #[case] expected_outcome: PauseTimerOutcome,
    ) {
        let (new_state, outcome) = pause(initial, NOW);
        assert_eq!(new_state, expected_state);
        assert_eq!(outcome, expected_outcome);
    }
}
