//! These functions use the Unix `/dev/null` device and syscalls
//! to redirect the stdout and stderr.
//!
//! It uses a global mutex to prevent concurrent calls.
//! This is because the `dup` and `dup2` syscalls are not thread-safe,
//! and concurrent usage can cause the file descriptors to become out of sync.

use nix::sys::stat::Mode;

use crate::{error, opts::Opts};

/// This mutex is used to prevent concurrent calls to `run_quietly_with_opts`.
static UNIX_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn run_quietly_with_opts<TOut>(
    action: impl FnOnce() -> TOut + std::panic::UnwindSafe,
    opts: Opts,
) -> Result<TOut, error::Error<TOut>> {
    let (stdout, stderr) = opts.descriptors.to_bools();
    let mut stdout_fd = None;
    let mut stderr_fd = None;

    let _lock = UNIX_LOCK
        .lock()
        .map_err(|_| error::Error::FailedToBeginRedirection)?;

    if stdout {
        // Create a copy of the stdout file descriptor (for restoring later).
        // If there's an error, the result will be Err.
        let new_stdout_fd = nix::unistd::dup(nix::libc::STDOUT_FILENO)
            .map_err(|_| error::Error::FailedToBeginRedirection)?;

        stdout_fd = Some(new_stdout_fd);

        // Open the null device.
        // If there's an error, the result will be Err.
        let null_fd = nix::fcntl::open(
            "/dev/null",
            nix::fcntl::OFlag::O_WRONLY,
            Mode::from_bits_retain(0o666),
        )
        .map_err(|_| error::Error::FailedToBeginRedirection)?;

        // Replace the stdout file descriptor with the null device.
        // The stdout will be closed.
        // If there's an error, the result will be Err.
        nix::unistd::dup2(null_fd, nix::libc::STDOUT_FILENO)
            .map_err(|_| error::Error::FailedToBeginRedirection)?;

        // At this point, stdout is fully redirected.
    }

    if stderr {
        // Create a copy of the stderr file descriptor (for restoring later).
        // If there's an error, the result will be Err.
        let new_stderr_fd = nix::unistd::dup(nix::libc::STDERR_FILENO)
            .map_err(|_| error::Error::FailedToBeginRedirection)?;

        stderr_fd = Some(new_stderr_fd);

        // Open the null device.
        // If there's an error, the result will be Err.
        let null_fd = nix::fcntl::open(
            "/dev/null",
            nix::fcntl::OFlag::O_WRONLY,
            Mode::from_bits_retain(0o666),
        )
        .map_err(|_| error::Error::FailedToBeginRedirection)?;

        // Replace the stderr file descriptor with the null device.
        // The stderr will be closed.
        // If there's an error, the result will be Err.
        nix::unistd::dup2(null_fd, nix::libc::STDERR_FILENO)
            .map_err(|_| error::Error::FailedToBeginRedirection)?;

        // At this point, stderr is fully redirected.
    }

    // Run the function,
    // catching any panics.
    // If there's a panic, the result will be Err.
    let result = std::panic::catch_unwind(action);

    // Restore stdout and stderr.
    if let Some(stdout_fd) = stdout_fd {
        // Replace the stdout file descriptor with the original one.
        // The /dev/null device will be closed.
        // If there's an error, the result will be Err.
        if let Err(_) = nix::unistd::dup2(stdout_fd, nix::libc::STDOUT_FILENO) {
            match result {
                Ok(result) => return Err(error::Error::FailedToUndoRedirection(result)),
                Err(_) => return Err(error::Error::PanickedAndFailedToUndoRedirection),
            }
        }

        // Close the saved copy of the stdout file descriptor.
        // If there's an error, the result will be Err.
        if let Err(_) = nix::unistd::close(stdout_fd) {
            match result {
                Ok(result) => return Err(error::Error::FailedToUndoRedirection(result)),
                Err(_) => return Err(error::Error::PanickedAndFailedToUndoRedirection),
            }
        }
    }

    if let Some(stderr_fd) = stderr_fd {
        // Replace the stderr file descriptor with the original one.
        // The /dev/null device will be closed.
        // If there's an error, the result will be Err.
        if let Err(_) = nix::unistd::dup2(stderr_fd, nix::libc::STDERR_FILENO) {
            match result {
                Ok(result) => return Err(error::Error::FailedToUndoRedirection(result)),
                Err(_) => return Err(error::Error::PanickedAndFailedToUndoRedirection),
            }
        }

        // Close the saved copy of the stderr file descriptor.
        // If there's an error, the result will be Err.
        if let Err(_) = nix::unistd::close(stderr_fd) {
            match result {
                Ok(result) => return Err(error::Error::FailedToUndoRedirection(result)),
                Err(_) => return Err(error::Error::PanickedAndFailedToUndoRedirection),
            }
        }
    }

    match result {
        Ok(result) => return Ok(result),
        Err(_) => return Err(error::Error::Panicked),
    }
}
