use std::{num::ParseIntError, str::FromStr};

/// A non-negative number of seconds. Good enough for the purposes of
/// pomodoro timers.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct Duration(u64);

const ONE_MINUTE: Duration = Duration::secs(60);

impl Duration {
    pub const ZERO: Self = Self(0);

    /// Create a new duration from a number of seconds.
    #[must_use]
    pub const fn secs(seconds: u64) -> Self {
        Self(seconds)
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self > &ONE_MINUTE {
            write!(f, "{}m", self.0.div_ceil(ONE_MINUTE.0))
        } else {
            write!(f, "{}s", self.0)
        }
    }
}

impl FromStr for Duration {
    type Err = ParseIntError;

    // Naive but good enough for my personal use case.
    // Assumes that the input is minutes OR seconds, not a combination.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mins) = s.strip_suffix('m') {
            Ok(Self(mins.parse::<u64>()?.saturating_mul(ONE_MINUTE.0)))
        } else if let Some(secs) = s.strip_suffix('s') {
            Ok(Self(secs.parse::<u64>()?))
        } else {
            Ok(Self(s.parse::<u64>()?))
        }
    }
}

impl From<Duration> for u64 {
    fn from(value: Duration) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("30", 30)]
    #[case("60s", 60)]
    #[case("10m", 600)]
    fn valid_duration_parses(#[case] input: &str, #[case] expected: u64) {
        let duration = input.parse::<Duration>().expect("a valid duration");
        assert_eq!(duration, Duration(expected));
    }

    #[rstest]
    #[case("")]
    #[case("foo")]
    #[case("1m30")]
    #[case("1m30s")]
    #[case("1m 30s")]
    #[case("1h")]
    fn invalid_duration_does_not_parse(#[case] input: &str) {
        let duration = input.parse::<Duration>();
        assert!(duration.is_err());
    }
}
