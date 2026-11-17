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
        State::Running { ends_at } if now > ends_at => start(State::Idle, duration_secs, now),
        running @ State::Running { .. } => (running, StartTimerOutcome::AlreadyRunning),
        State::Idle => (
            State::Running {
                ends_at: now.saturating_add(duration_secs),
            },
            StartTimerOutcome::Started,
        ),
        State::Paused { remaining_secs: 0 } => start(State::Idle, duration_secs, now),
        State::Paused { remaining_secs } => (
            State::Running {
                ends_at: now.saturating_add(remaining_secs),
            },
            StartTimerOutcome::Resumed,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use StartTimerOutcome as O;
    use State as S;
    use rstest::rstest;
    const NOW: u64 = 1_794_000_000;

    #[rstest]
    //      intitial state                      duration_secs       expected_state              expected_outcome
    #[case(S::Idle,                                 0,          S::Running { ends_at: NOW },        O::Started)]
    #[case(S::Idle,                                 20,         S::Running { ends_at: NOW + 20},    O::Started)]
    #[case(S::Idle,                                 u64::MAX,   S::Running { ends_at: u64::MAX },   O::Started)]
    #[case(S::Running { ends_at: NOW - 1},          20,         S::Running { ends_at: NOW + 20},    O::Started)]
    #[case(S::Running { ends_at: NOW},              20,         S::Running { ends_at: NOW + 20 },   O::Started)]
    #[case(S::Running { ends_at: NOW + 1},          20,         S::Running { ends_at: NOW + 1},     O::AlreadyRunning)]
    #[case(S::Paused { remaining_secs: 0},          20,         S::Running { ends_at: NOW + 20 },   O::Started)]
    #[case(S::Paused { remaining_secs: 30},         20,         S::Running { ends_at: NOW + 30 },   O::Resumed)]
    #[case(S::Paused { remaining_secs: u64::MAX},   20,         S::Running { ends_at: u64::MAX },   O::Resumed)]
    fn starting_returns_expected(
        #[case] initial: State,
        #[case] duration_secs: u64,
        #[case] expected_state: State,
        #[case] expected_outcome: StartTimerOutcome,
    ) {
        let (new_state, outcome) = start(initial, duration_secs, NOW);
        assert_eq!(new_state, expected_state);
        assert_eq!(outcome, expected_outcome);
    }
}
