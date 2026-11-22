use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

/// Reads the current system clock time as seconds since
/// the UNIX epoch.
///
/// # Errors
///
/// This function will propagate an error if the current
/// time cannot be read.
pub fn read_clock() -> Result<u64, SystemTimeError> {
    Ok(
        // We are in the shell, so it's legal to call SystemTime::now
        #[allow(clippy::disallowed_methods)]
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
    )
}
